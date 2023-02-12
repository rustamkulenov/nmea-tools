use std::{
    any::Any,
    io::{BufRead, BufReader, Read},
};

use nmeaParseTest::messages::{AddrField, MessagesMap};
use nmeaParseTest::{get_message_body, HandleField};

pub struct NmeaParser {}

/// Message parsed callback.
/// Parameter can be downcasted to concrete message class:
///
/// # Example
/// ```no_run
///    fn callback(msg: &dyn Any) -> () {
///       let gll = msg.downcast_ref::<NmeaGllMessage>().unwrap();
///        println!("{:?}", gll);
///    }
/// ```
type FnMsgParsed = dyn Fn(&dyn Any) -> ();

pub struct FieldParseHandler<'a> {
    all_messages: &'a mut MessagesMap,
    callback: &'a FnMsgParsed,
}

impl NmeaParser {
    pub fn parse(inner: &mut dyn Read, callback: &'static FnMsgParsed) -> std::io::Result<()> {
        let mut br = BufReader::new(inner);
        let mut msgs_map = MessagesMap::new();
        msgs_map.add_all_messages();

        let mut h = FieldParseHandler::new(&mut msgs_map, &callback);

        loop {
            let amount = {
                let buf = br.fill_buf().unwrap();
                if buf.is_empty() {
                    break;
                };
                let (consume_amt, msg) = get_message_body(&buf, &mut h);
                println!("Consumed {consume_amt} chars. CRC ok: {:?}", msg.crc_ok);
                consume_amt
            };
            br.consume(amount);
        }

        Ok(())
    }
}

impl<'a> FieldParseHandler<'a> {
    fn new(msgs_map: &'a mut MessagesMap, callback: &'a FnMsgParsed) -> FieldParseHandler<'a> {
        FieldParseHandler {
            all_messages: msgs_map,
            callback,
        }
    }
}

impl<'a> HandleField for FieldParseHandler<'a> {
    fn handle(&mut self, addr_field: &AddrField<'_>, field_idx: u8, field: &[u8]) {
        let boxed_msg = self.all_messages.get_mut(addr_field).unwrap();

        boxed_msg.set_field(field_idx, field);

        println!("Field {:?} from {:?}", field_idx, boxed_msg.field_count());

        if field_idx == boxed_msg.field_count() - 1 {
            // Last field parsed, notify listeners
            let boxed_msg = self.all_messages.get(&addr_field).unwrap();
            let orig_msg: &dyn Any = boxed_msg.as_any();
            (self.callback)(orig_msg);
        }
    }
}
