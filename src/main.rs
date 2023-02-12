use std::{any::Any, env, fs::File, io::Read};

use nmeaParseTest::generated::nmea3::{NmeaGllMessage, NmeaMessages};
use nmea_parser::NmeaParser;

mod nmea_parser;

fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = env::args().collect();

    let mut br: Box<dyn Read> = if atty::is(atty::Stream::Stdin) {
        let f = File::open(&arguments[1])?;
        Box::new(f)
    } else {
        Box::new(std::io::stdin())
    };

    NmeaParser::parse(&mut br, &callback)?;
    Ok(())
}

fn callback(msg_type: NmeaMessages, msg: &dyn Any) -> () {
    match msg_type {
        NmeaMessages::GLL => {
            let gll = msg.downcast_ref::<NmeaGllMessage>().unwrap();
            println!("{:?}", gll);
        }
        _ => panic!(),
    }
}
