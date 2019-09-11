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

    let _dev = rtc.destroy();
}
