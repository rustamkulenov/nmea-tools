use std::{collections::HashSet, vec};

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

pub const ALL_TALKER_IDS: [[u8; 2]; 2] = [[0x47, 0x50], [0x47, 0x4C]]; // GP, GL
//pub const ALL_TALKER_IDS_HS: HashSet<[u8; 2]> = HashSet::from_iter(ALL_TALKER_IDS);

pub fn is_talker_id(v: &[u8]) -> bool {
    assert!(v.len() == 2);
    let s: [u8; 2] = [v[0], v[1]];
    ALL_TALKER_IDS.contains(&s) //TODO:  O(n) complexity. Replace with hashset!
}


/* Generated constructor for initializing all message types */
impl MessagesMap {
    pub fn add_all_messages(&mut self) {
        let msgs: Vec<Box<dyn MessageFields>> = vec![Box::new(NmeaGllMessage::new())];

        for m in msgs {
            let k = m.get_addr();
            self.msgs.insert(k, m);
        }
    }
}
