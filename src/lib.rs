use messages::AddrField;

pub mod generated;
pub mod messages;

const DOLLAR: u8 = b'$';
const EXCLAMATION: u8 = b'!';
const ASTERISK: u8 = b'*';
const COMMA: u8 = b',';
const LF: u8 = 0x0A;
const CR: u8 = 0x0D;

/// Semi-parsed NMEA message.
/// Contains main message parts: address field (from '$' to ',') and not parsed fields.
pub struct NmeaMessage<'a> {
    pub addr_field: &'a [u8],
    pub fields: &'a [u8],
    pub crc_ok: bool,
}

/// Trait for parser a callback. Is called when a fields is parsed.
/// Is responsible to handle field's value, convert into required format and store.
pub trait HandleField {
    fn handle(&mut self, addr_field: &AddrField<'_>, field_idx: u8, field: &[u8]);
}

/// Parses single message from buffer until LF.
/// Calls a handler's callback on each field detected.
/// See returned `[consume_amt]` to know how many bytes were read from the `[buf]`.
pub fn get_message_body<'buf>(
    buf: &'buf [u8], // Source bufer
    field_handler: &mut (dyn HandleField),
) -> (usize /* consume_amt */, NmeaMessage<'buf>) {
    assert!(buf.len() > 10, "Too short NMEA message");
    assert!(
        buf[0] == DOLLAR,
        "Unexpected 1st char '{}'",
        char::from(buf[0])
    );

    let mut consume_amt = 0;
    let mut addr_end: usize = 1;
    let mut crc = 0u8;
    let mut crc_ok = false;

    // Detect address field position [1..addr_end]
    while addr_end < buf.len() && ![COMMA, CR, LF].contains(&buf[addr_end]) {
        crc ^= buf[addr_end];
        addr_end += 1;
    }
    crc ^= COMMA;

    let addr_field = AddrField::new(&buf[1..addr_end]);
    let mut asterisk_pos = addr_end + 1; // next char after 1st ','
    let mut field_start = addr_end + 1;
    let mut field_idx: u8 = 0;
    // Parse message content until CRC
    loop {
        if ![ASTERISK, CR, LF].contains(&buf[asterisk_pos]) {
            crc ^= buf[asterisk_pos];
        }

        // Detect fields and provide to concrete message parsers
        if [COMMA, ASTERISK, CR, LF].contains(&buf[asterisk_pos]) {
            let field = &buf[field_start..asterisk_pos];
            field_start = asterisk_pos + 1;

            field_handler.handle(&addr_field, field_idx, field);

            field_idx += 1;
        }

        if asterisk_pos >= buf.len() || [ASTERISK, CR, LF].contains(&buf[asterisk_pos]) {
            break;
        }
        asterisk_pos += 1;
    }

    // At this point message CRC is calculated. Compare with a CRC value in message after * if it is not empty.
    if buf.len() - asterisk_pos >= 3 && ![CR, LF].contains(&buf[asterisk_pos + 1]) {
        let expected_crc = hex_chars_to_u8(&buf[asterisk_pos + 1..asterisk_pos + 3]);
        crc_ok = expected_crc == crc;
    }

    consume_amt = asterisk_pos;
    while asterisk_pos < buf.len() && buf[asterisk_pos] != LF {
        consume_amt += 1;
        asterisk_pos += 1;
    }
    if asterisk_pos < buf.len() {
        consume_amt += 1; // LF
    }

    return (
        consume_amt,
        NmeaMessage {
            addr_field: &buf[1..addr_end],
            fields: &buf[addr_end..asterisk_pos - 1],
            crc_ok,
        },
    );
}

/// Converts 2 char ASCII hex value to a byte value.
fn hex_chars_to_u8(h: &[u8]) -> u8 {
    let mut res: u8 = 0;
    res += 16
        * if h[0] < b'A' {
            h[0] - b'0'
        } else {
            h[0] - b'A' + 10
        };
    res += if h[1] < b'A' {
        h[1] - b'0'
    } else {
        h[1] - b'A' + 10
    };
    res
}

#[cfg(test)]
mod tests {
    use crate::{get_message_body, hex_chars_to_u8, messages::AddrField, HandleField, NmeaMessage};

    struct FieldHandlerStub {}

    impl FieldHandlerStub {
        fn new() -> FieldHandlerStub {
            FieldHandlerStub {}
        }
    }

    impl HandleField for FieldHandlerStub {
        fn handle(&mut self, addr_field: &AddrField, field_idx: u8, field: &[u8]) {
            println!(
                "{} {} >>> {}",
                String::from_utf8(addr_field.data.to_vec()).unwrap(),
                field_idx,
                String::from_utf8(field.to_vec()).unwrap()
            );
        }
    }

    pub fn get_message_body_stub<'a>(buf: &'a [u8]) -> NmeaMessage {
        get_message_body(buf, &mut FieldHandlerStub::new()).1
    }

    #[test]
    fn consume_amt_test() {
        let buf = "$GPGLL,3751.65,S,14507.36,E*77".as_bytes();
        let r = get_message_body(buf, &mut FieldHandlerStub::new());
        assert_eq!(buf.len(), r.0);
    }

    #[test]
    fn consume_amt_crlf_test() {
        let buf = "$GPGLL,3751.65,S,14507.36,E*77\r\n".as_bytes();
        let r = get_message_body(buf, &mut FieldHandlerStub::new());
        assert_eq!(buf.len(), r.0);
    }

    #[test]
    fn consume_amt_lf_test() {
        let buf = "$GPGLL,3751.65,S,14507.36,E*77\n".as_bytes();
        let r = get_message_body(buf, &mut FieldHandlerStub::new());
        assert_eq!(buf.len(), r.0);
    }

    #[test]
    fn consume_2_lines_amt_test() {
        let s = format!("$GPGLL,3751.65,S,14507.36,E*77\n$GPRMC,87,E*4B");
        let buf = s.as_bytes();
        let r1 = get_message_body(buf, &mut FieldHandlerStub::new());
        let r2 = get_message_body(&buf[r1.0..], &mut FieldHandlerStub::new());
        assert_eq!(buf.len(), r1.0 + r2.0);
    }

    #[test]
    #[should_panic]
    fn incorrect_prefix() {
        let _ = get_message_body_stub("ups***KJHASDKJHASDLkjkljasd".as_bytes());
    }

    #[test]
    fn empty_valid_nmea() {
        let m = get_message_body_stub("$GPRMC,,*4B".as_bytes());
        assert_eq!(m.addr_field, "GPRMC".as_bytes());
        assert!(m.crc_ok);
    }

    #[test]
    fn simple_valid_nmea() {
        let m = get_message_body_stub("$GPGLL,3751.65,S,14507.36,E*77".as_bytes());
        assert_eq!(m.addr_field, "GPGLL".as_bytes());
        assert!(m.crc_ok);
    }

    #[test]
    fn simple_valid_nmea_without_crc() {
        let _ = get_message_body_stub("$GPGLL,3751.65,S,14507.36,E*".as_bytes());
    }

    #[test]
    fn hex_to_char_72() {
        let v = hex_chars_to_u8(&"72".as_bytes());
        assert_eq!(v, 0x72)
    }

    #[test]
    fn hex_to_char_fa() {
        let v = hex_chars_to_u8(&"FA".as_bytes());
        assert_eq!(v, 0xFA)
    }

    #[test]
    fn hex_to_char_6c() {
        let v = hex_chars_to_u8(&"6C".as_bytes());
        assert_eq!(v, 0x6C)
    }
}
