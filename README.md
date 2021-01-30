# Rust MCP794xx Real-Time Clock / Calendar Family Driver

[![crates.io](https://img.shields.io/crates/v/mcp794xx.svg)](https://crates.io/crates/mcp794xx)
[![Docs](https://docs.rs/mcp794xx/badge.svg)](https://docs.rs/mcp794xx)
[![Build Status](https://github.com/eldruin/mcp794xx-rs/workflows/Build/badge.svg)](https://github.com/eldruin/mcp794xx-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/eldruin/mcp794xx-rs/badge.svg?branch=master)](https://coveralls.io/r/eldruin/mcp794xx-rs?branch=master)

This is a platform agnostic Rust driver for the MCP794xx real-time clock
/ calendar family, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
  - Read and set date and time. See: `get_datetime()`.
  - Read and set date. See: `get_date()`.
  - Read and set time. See: `get_time()`.
  - Read and set date and time individual elements. For example, see: `get_year()`.
  - Enable and disable the real-time clock. See: `enable()`.
  - Read whether the oscillator is running. See: `is_oscillator_running()`.
  - Read whether the current year is a leap year. See: `is_leap_year()`.
  - Enable and disable the usage of an external oscillator source. See: `enable_external_oscillator.
  - Set the output pin logic level. See: `set_output_pin()`.
  - Enable and disable coarse trim. See: `enable_coarse_trim()`.
  - Set trimming value. See: `set_trimming()`.
  - Power:
      - Read whether the power has failed. See: `has_power_failed()`.
      - Clear the has-power-failed flag. See: `clear_power_failed()`.
      - Read the date/time when power went down. See: `get_power_down_datetime()`.
      - Read the date/time when power went back up. See: `get_power_up_datetime()`.
      - Enable and disable usage of backup battery power. See: `enable_backup_battery_power()`.
  - SRAM:
      - Read and write byte to SRAM. See: `read_sram_byte()`.
      - Read and write byte array to SRAM. See: `read_sram_data()`.
      - Read current position from SRAM. See: `read_sram_current_byte()`.
  - Alarms:
      - Enable and disable alarms. See: `enable_alarm()`.
      - Set alarms with several matching policies and output pin polarities. See: `set_alarm`.
      - Read whether alarms have matched. See: `has_alarm_matched`.
      - Clear flag indicating that alarms have matched. See: `clear_alarm_matched_flag`.
  - Wave generation:
      - Enable and disable the square-wave generation. See: `enable_square_wave`.
      - Select the square-wave frequency. See: `set_square_wave_frequency`.
  - Protected EEPROM:
      - Read and write byte to the protected EEPROM. See: `read_protected_eeprom_byte()`.
      - Read and write byte array to the protected EEPROM. See: `read_protected_eeprom_data()`.
      - Read EUI-48. See: `read_eui48()`.
      - Read EUI-64. See: `read_eui64()`.
  - EEPROM:
      - Read and write byte to the EEPROM. See: `read_eeprom_byte()`.
      - Read and write byte array to the EEPROM. See: `read_eeprom_data()`.
      - Set EEPROM block write protection. See: `set_eeprom_write_protection()`.
      - Read current position from the EEPROM. See: `read_eeprom_current_byte()`.

[Introductory blog post](https://blog.eldruin.com/mcp794xx-real-time-clock-rtc-driver-in-rust/)

## The devices

This driver is compatible with the devices: MCP7940N, MCP7940M, MCP79400, MCP79401, MCP79402, MCP79410, MCP79411 and MCP79412.

The Real-Time Clock/Calendar (RTCC) tracks time using internal counters for hours, minutes, seconds, days, months, years, and day of week. Alarms can be configured on all counters up to and including months. For usage and configuration, the devices support I2C communications up to 400 kHz.

The open-drain, multi-functional output can be configured to assert on an alarm match, to output a selectable frequency square wave, or as a general purpose output.

The devices are designed to operate using a 32.768 kHz tuning fork crystal with external crystal load capacitors. On-chip digital trimming can be used to adjust for frequency variance caused by crystal tolerance and temperature.

SRAM and timekeeping circuitry are powered from the back-up supply when main power is lost, allowing the device to maintain accurate time and the SRAM contents. The times when the device switches over to the back-up supply and when primary power returns are both logged by the power-fail time-stamp.

Some of the devices feature 1 Kbit of internal non-volatile EEPROM with software write-protectable regions. There is an additional 64 bits of protected non-volatile memory which is only writable after an unlock sequence, making it ideal for storing a unique ID or other critical information.

Some of the devices offer a pre-programmed with EUI-48 and EUI-64 addresses. Custom programming is also available.

Datasheets:
- [MCP7940N](http://ww1.microchip.com/downloads/en/DeviceDoc/20005010F.pdf)
- [MCP7940M](http://ww1.microchip.com/downloads/en/DeviceDoc/20002292B.pdf)
- [MCP79400/MCP79401/MCP79402](http://ww1.microchip.com/downloads/en/DeviceDoc/MCP79400-MCP79401-MCP79402-Data-Sheet-20005009G.pdf)
- [MCP79410/MCP79411/MCP79412](http://ww1.microchip.com/downloads/en/DeviceDoc/20002266H.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

In the following example an instance of the device MCP7940N will be created.
Other devices can be created with similar methods like:
`Mcp794xx::new_mcp79400(...)`.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate mcp794xx;

use linux_embedded_hal::I2cdev;
use mcp794xx::{NaiveDate, Hours, Mcp794xx, Rtcc};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Mcp794xx::new_mcp7940n(dev);
    rtc.enable().unwrap();
    let datetime = NaiveDate::from_ymd(2018, 8, 20).and_hms(19, 59, 58);
    rtc.set_datetime(&datetime).unwrap();
    rtc.enable().unwrap();
    // do something else...
    let seconds = rtc.get_seconds().unwrap();
    println!("Seconds: {}", seconds);

    let _dev = rtc.destroy();
}
```

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

