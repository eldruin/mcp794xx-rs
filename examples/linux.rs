use linux_embedded_hal::I2cdev;
use mcp794xx::{Mcp794xx, NaiveDate, Rtcc};

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
