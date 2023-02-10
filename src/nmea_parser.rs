use std::{any::Any, io::BufRead};

use crate::messages::{AddrField, MessagesMap};
use nmeaParseTest::{get_message_body, HandleField};

const BUF_SIZE: usize = 1024 * 1024 * 5;

pub struct NmeaParser {}

/* Message parsed callback
    Parameter can be downcasted to concrete message class:

*/
type FnMsgParsed = dyn Fn(&dyn Any) -> ();

pub struct FieldParseHandler<'a> {
    all_messages: &'a mut MessagesMap<'a>,
    callback: &'a FnMsgParsed,
}

impl NmeaParser {
    pub fn parse<'buf>(
        br: &mut Box<dyn BufRead>,
        callback: &'static FnMsgParsed,
    ) -> std::io::Result<()> {
        let mut buf = vec![0u8; BUF_SIZE];
        let mut msgs_map = MessagesMap::new();
        msgs_map.add_all_messages();

        let mut h = FieldParseHandler::new(&mut msgs_map, &callback);

        loop {
            let r = br.read(&mut buf);
            let _ = get_message_body(&buf, &mut h);
            break;
        }

        Ok(())
    }
}

impl<'a> FieldParseHandler<'a> {
    fn new(msgs_map: &'a mut MessagesMap<'a>, callback: &'a FnMsgParsed) -> FieldParseHandler<'a> {
        FieldParseHandler {
            all_messages: msgs_map,
            callback,
        }
    }
}

impl<'a, 'buf: 'a> HandleField<'buf> for FieldParseHandler<'a> {
    fn handle(&mut self, addr_field: &'buf [u8], field_idx: u8, field: &'buf [u8]) {
        let boxed_msg = self
            .all_messages
            .get_mut(AddrField::new(addr_field))
            .unwrap();
        if field_idx == 0 {
            boxed_msg.clear();
        }

        let f = boxed_msg.get_field_mut(field_idx);
        f.set_from_slice(field);

        println!("Field {:?} from {:?}", field_idx, boxed_msg.field_count());

        if field_idx == boxed_msg.field_count() - 1 {
            // Last field parsed, notify listeners
            let boxed_msg = self.all_messages.get(AddrField::new(addr_field)).unwrap();
            let orig_msg: &dyn Any = boxed_msg.as_any();
            (self.callback)(orig_msg);
        }
    }
}
