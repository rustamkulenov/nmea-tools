use messages::AddrField;

pub mod generated;
pub mod messages;

const DOLLAR: u8 = b'$';
const EXCLAMATION: u8 = b'!';
const ASTERISK: u8 = b'*';
const COMMA: u8 = b',';

pub struct NmeaMessage<'a> {
    pub addr_field: &'a [u8],
    pub fields: &'a [u8],
    pub crc_ok: bool,
}

pub trait HandleField {
    fn handle(&mut self, addr_field: &AddrField<'_>, field_idx: u8, field: &[u8]);
}

/// Parses single message from buffer until CRLF.
/// Calls a callback on each field detected.
pub fn get_message_body<'buf>(
    buf: &'buf [u8], // Source bufer
    field_handler: &mut (dyn HandleField),
) -> (u8, NmeaMessage<'buf>) {
    assert!(buf.len() > 10, "Too short NMEA message");
    assert!(
        buf[0] == DOLLAR,
        "Unexpected 1st char '{}'",
        char::from(buf[0])
    );

    let consume_amt = 0u8;
    let mut addr_end: usize = 1;
    let mut crc = 0u8;

    // Detect address field position [1..addr_end]
    while addr_end < buf.len() && buf[addr_end] != COMMA {
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
        if buf[asterisk_pos] != ASTERISK {
            crc ^= buf[asterisk_pos];
        }

        // Detect fields and provide to concrete message parsers
        if buf[asterisk_pos] == COMMA || buf[asterisk_pos] == ASTERISK {
            let field = &buf[field_start..asterisk_pos];
            field_start = asterisk_pos + 1;

            field_handler.handle(&addr_field, field_idx, field);

            field_idx += 1;
        }

        if asterisk_pos >= buf.len() || buf[asterisk_pos] == ASTERISK {
            break;
        }
        asterisk_pos += 1;
    }

    // At this point message CRC is calculated. Compare with a CRC value in message after *
    if buf.len() - asterisk_pos == 3 {
        let expected_crc = hex_chars_to_u8(&buf[asterisk_pos + 1..asterisk_pos + 3]);
        assert!(expected_crc == crc, "Invalid CRC");
    }

    return (
        consume_amt,
        NmeaMessage {
            addr_field: &buf[1..addr_end],
            fields: &buf[addr_end..asterisk_pos - 1],
            crc_ok: true,
        },
    );
}

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
    #[should_panic]
    fn incorrect_prefix() {
        let _ = get_message_body_stub("ups***KJHASDKJHASDLkjkljasd".as_bytes());
    }

    #[test]
    fn empty_valid_nmea() {
        let m = get_message_body_stub("$GPRMC,,*4B".as_bytes());
        assert!(m.addr_field == "GPRMC".as_bytes());
        assert!(m.crc_ok);
    }

    #[test]
    fn simple_valid_nmea() {
        let m = get_message_body_stub("$GPGLL,3751.65,S,14507.36,E*77".as_bytes());
        assert!(m.addr_field == "GPGLL".as_bytes());
        assert!(m.crc_ok);
    }

    #[test]
    fn simple_valid_nmea_without_crc() {
        let _ = get_message_body_stub("$GPGLL,3751.65,S,14507.36,E*".as_bytes());
    }

    #[test]
    fn hex_to_char_72() {
        let v = hex_chars_to_u8(&"72".as_bytes());
        assert!(v == 0x72)
    }

    #[test]
    fn hex_to_char_fa() {
        let v = hex_chars_to_u8(&"FA".as_bytes());
        assert!(v == 0xFA)
    }

    #[test]
    fn hex_to_char_6c() {
        let v = hex_chars_to_u8(&"6C".as_bytes());
        assert!(v == 0x6C)
    }
}
