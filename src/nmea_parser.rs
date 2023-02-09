use std::{io::BufRead, ops::Deref};

use nmeaParseTest::{get_message_body, HandleField};

use crate::messages::{AddrField, MessageFields, MessagesMap};

const BUF_SIZE: usize = 1024 * 1024 * 5;

pub struct NmeaParser {}

/* Message parsed callback */
type FnMsgParsed<'buf> = dyn Fn(Box<dyn MessageFields<'buf> + 'buf>) -> ();

pub struct FieldParseHandler<'a, 'buf> {
    all_messages: &'a mut MessagesMap<'a, 'buf>,
    callback: &'a FnMsgParsed<'buf>,
}

impl NmeaParser {
    pub fn parse(br: &mut Box<dyn BufRead>, callback: &FnMsgParsed) -> std::io::Result<()> {
        let mut buf = vec![0u8; BUF_SIZE];
        let mut msgs_map = MessagesMap::new();
        msgs_map.add_all_messages();

        let mycallback = |msg| -> () {
            //callback(msg);
        };

        let mut h = FieldParseHandler::new(&mut msgs_map, &mycallback);
        //let handler: &mut dyn HandleField<'_> = &mut h;

        loop {
            let r = br.read(&mut buf);
            let _ = get_message_body(&buf, &mut h);
            break;
        }

        Ok(())
    }
}

impl<'a, 'buf> FieldParseHandler<'a, 'buf> {
    fn new(
        msgs_map: &'a mut MessagesMap<'a, 'buf>,
        callback: &'a FnMsgParsed<'buf>,
    ) -> FieldParseHandler<'a, 'buf> {
        FieldParseHandler {
            all_messages: msgs_map,
            callback,
        }
    }
}

impl<'a, 'buf> HandleField<'buf> for FieldParseHandler<'a, 'buf> {
    fn handle(&mut self, addr_field: &'buf [u8], field_idx: u8, field: &'buf [u8]) {
        let m = self
            .all_messages
            .get_mut(AddrField::new(addr_field))
            .unwrap();
        if field_idx == 0 {
            m.clear();
        }

        let f = m.get_field_mut(field_idx);
        f.set_from_slice(field);

        if field_idx == m.field_count() {
            // Last field parsed, notify listeners
            let m = self
                .all_messages
                .get(AddrField::new(addr_field))
                .unwrap();

//            let m = (**m);
//            (self.callback)(m);
        }
    }
}
