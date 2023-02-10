use crate::messages::{MessageFields, MessagesMap};

use self::gll::NmeaGllMessage;

pub mod gll;

pub enum NmeaMessages {
    GLL,
}

pub enum TalkerIds {
    GP, // GPS
    GL, // Glonass
    GN, // GNSS
    AI, // AIS
    AG, // Autopilot General
    AP, // Autopilot Magnetic
    IN, // Integrated Navigation
    II, // Integrated Instrumentation
    P,  // Proprietary
    RA, // Radar
    SD, // Sounder, depth
    SS, // Sounder, scanning
    TI, // Tourn Rate Indicator
    VD, // Velocity doppler
    VM, // Speed log, water, magnetic
    VW, // Speed log, water, mechanical
    WI, // Weather Instruments
}

/* Generated constructor for initializing all message types */
impl<'a> MessagesMap<'a> {
    pub fn add_all_messages(&mut self) {
        let msgs: Vec<Box<dyn MessageFields>> =
            vec![Box::new(NmeaGllMessage::new() as NmeaGllMessage)];

        for m in msgs {
            let k = m.get_addr();
            self.msgs.insert(k, m);
        }
    }
}
