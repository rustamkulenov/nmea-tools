pub mod nmea3;

pub enum TalkerIds {
    GP, // GPS
    GL, // Glonass
    GA, // Galileo
    GQ, // QZSS
    GB, // BeiDou
    BD, // BeiDou
    GN, // GNSS - from multiple systems

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

pub fn is_talker_id(v: &[u8]) -> bool {
    assert!(v.len() == 2);
    let s: [u8; 2] = [v[0], v[1]];
    ALL_TALKER_IDS.contains(&s) //TODO:  O(n) complexity. Replace with hashset!!!
}

