use linux_embedded_hal::I2cdev;
use mcp794xx::{DateTimeAccess, Mcp794xx, NaiveDate, Rtcc};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Mcp794xx::new_mcp7940n(dev);
    rtc.enable().unwrap();
    let datetime = NaiveDate::from_ymd_opt(2018, 8, 20)
        .expect("Invalid date.")
        .and_hms_opt(19, 59, 58)
        .expect("Invalid time.");
    rtc.set_datetime(&datetime).unwrap();
    rtc.enable().unwrap();
    // do something else...
    let seconds = rtc.seconds().unwrap();
    println!("Seconds: {}", seconds);

    let _dev = rtc.destroy();
}
