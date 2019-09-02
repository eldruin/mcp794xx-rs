# Rust MCP794xx Real-Time Clock / Calendar Family Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/mcp794xx.svg)](https://crates.io/crates/mcp794xx)
[![Docs](https://docs.rs/mcp794xx/badge.svg)](https://docs.rs/mcp794xx)
-->
[![Build Status](https://travis-ci.org/eldruin/mcp794xx-rs.svg?branch=master)](https://travis-ci.org/eldruin/mcp794xx-rs)
[![Coverage Status](https://coveralls.io/repos/eldruin/mcp794xx-rs/badge.svg?branch=master)](https://coveralls.io/r/eldruin/mcp794xx-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the MCP794xx real-time clock
/ calendar family, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

<!-- TODO
This driver allows you to:
-->

## The devices

### MCP7940N
The MCP7940N Real-Time Clock/Calendar (RTCC) tracks time using internal counters for hours, minutes, seconds, days, months, years, and day of week. Alarms can be configured on all counters up to and including months. For usage and configuration, the MCP7940N supports I2C communications up to 400 kHz.

The open-drain, multi-functional output can be configured to assert on an alarm match, to output a selectable frequency square wave, or as a general purpose output.

The MCP7940N is designed to operate using a 32.768 kHz tuning fork crystal with external crystal load capacitors. On-chip digital trimming can be used to adjust for frequency variance caused by crystal tolerance and temperature.

SRAM and timekeeping circuitry are powered from the back-up supply when main power is lost, allowing the device to maintain accurate time and the SRAM contents. The times when the device switches over to the back-up supply and when primary power returns are both logged by the power-fail time-stamp.

Datasheets:
- [MCP7940N](http://ww1.microchip.com/downloads/en/DeviceDoc/20005010F.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

In the following example an instance of the device MCP7940N will be created.
<!--
Other devices can be created with similar methods like:
`Mcp794xx::new_mcp7940n(...)`.
-->
Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate mcp794xx;

use linux_embedded_hal::I2cdev;
use mcp794xx::{DateTime, Hours, Mcp794xx, Rtcc};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Mcp794xx::new_mcp7940n(dev);
    rtc.enable().unwrap();
    let datetime = DateTime {
        year: 2018,
        month: 8,
        day: 20,
        weekday: 4,
        hour: Hours::H24(19),
        minute: 59,
        second: 58,
    };
    rtc.set_datetime(&datetime).unwrap();
    // do something else...
    let seconds = rtc.get_seconds().unwrap();
    println!("Seconds: {}", seconds);

    let _dev = rtc.destroy_mcp7940n();
}
```

## Status

This driver is compatible with:

- [X] MCP7940N
- [ ] MCP7940M
- [ ] MCP79400
- [ ] MCP79401
- [ ] MCP79402
- [ ] MCP79410
- [ ] MCP79411
- [ ] MCP79412

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/mcp794xx-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

