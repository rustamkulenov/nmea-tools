# NMEA Tools
Tools for NMEA protocol.

## Tools

1. NMEA Parser (written in rust) - quick (no proof yet, but I hope :) ) NMEA parser with ability to generate rust code for specific NMEA version from [json protocol specification](/src/nmea4_spec.j2.json) using [autogen](https://github.com/rustamkulenov/autogen). I tried to reuse mem buffer, minimize heap allocations, prefer refs over smart pointers, decrease number of indirection. For a flexibility ``dyn traits`` are used that leads to dynamic calls dispatch.   

### TODOs
* Better error handling (via Result<>). 
* [```is_talker_id```](/src/generated/mod.rs#L33) fn optimization (replace with Hashset).

### How to change NMEA specification (version)
1. Create new json file, or change [existing specification](./src/nmea3_spec.j2.json);
2. execute ```$ ./generate_code.sh```

### How to run sample application

```
$ cargo run ./1.nmea
Field 0 from 7
Field 1 from 7
Field 2 from 7
Field 3 from 7
Field 4 from 7
Field 5 from 7
Field 6 from 7
NmeaGllMessage { latitude: 3723.2475, latitude_dir: 78, longitude: 12158.3416, longitude_dir: 87, utc: Some("161229.487"), status: Some(65), mode: Some(65) }
Consumed 50 chars. CRC ok: true
Field 0 from 12
Field 1 from 12
Field 2 from 12
Field 3 from 12
Field 4 from 12
Field 5 from 12
Field 6 from 12
Field 7 from 12
Field 8 from 12
Field 9 from 12
Field 10 from 12
Field 11 from 12
NmeaRmcMessage { utc: Some("203522.00"), status: Some(65), latitude: 5109.0262308, latitude_dir: 78, longitude: 11401.8407342, longitude_dir: 87, sog: 0.004, cog: 133.4, date: Some("130522"), magnetic_variation: 0.0, magnetic_variation_dir: 69, mode: Some(68) }
Consumed 80 chars. CRC ok: true
```