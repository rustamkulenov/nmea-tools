use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use messages::{MessageFields, MessagesMap};
use nmea_parser::NmeaParser;

pub mod generated;
pub mod messages;
mod nmea_parser;

fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = env::args().collect();

    let mut br: Box<dyn BufRead> = if atty::is(atty::Stream::Stdin) {
        let f = File::open(&arguments[1])?;
        Box::new(BufReader::new(f))
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };

    NmeaParser::parse(&mut br, &callback)?;
    Ok(())
}

fn callback(msg: Box<dyn MessageFields>) -> () {}
