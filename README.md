# NMEA Tools
Tools for NMEA protocol.

## Tools

1. NMEA Parser (written in rust) - quick NMEA parser with ability to generate rust code describing NMEA messages from [json protocol specification](/src/nmea4_spec.j2.json) using [autogen](https://github.com/rustamkulenov/autogen). I tried to reuse buffer, minimize heap allocations, prefere refs over smart pointers decrease number of redirectons. 