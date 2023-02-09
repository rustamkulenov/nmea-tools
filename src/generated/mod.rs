use crate::messages::{MessageFields, MessagesMap};

use self::gll::NmeaGllMessage;

pub mod gll;

pub enum NmeaMessages<'a> {
    GLL(NmeaGllMessage<'a>),
}

/* Generated constructor for initializing all message types */
impl<'a, 'buf: 'a> MessagesMap<'a, 'buf> {
    pub fn add_all_messages(&mut self) {
        let msgs: Vec<Box<dyn MessageFields<'buf>>> =
            vec![Box::new(NmeaGllMessage::new() as NmeaGllMessage<'buf>)];

        for m in msgs {
            let k = m.get_addr();
            self.msgs.insert(k, m);
        }
    }
}
