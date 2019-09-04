extern crate embedded_hal;
extern crate mcp794xx;
use self::mcp794xx::{ic, interface, Mcp794xx};
extern crate embedded_hal_mock as hal;
use self::hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

#[allow(unused)]
pub const DEVICE_ADDRESS: u8 = 0b1101111;
pub struct Register;

#[allow(unused)]
impl Register {
    pub const SECONDS: u8 = 0x00;
    pub const MINUTES: u8 = 0x01;
    pub const HOURS: u8 = 0x02;
    pub const WEEKDAY: u8 = 0x03;
    pub const DAY: u8 = 0x04;
    pub const MONTH: u8 = 0x05;
    pub const YEAR: u8 = 0x06;
    pub const CONTROL: u8 = 0x07;
    pub const OSCTRIM: u8 = 0x08;
    pub const ALM0SEC: u8 = 0x0A;
    pub const ALM1SEC: u8 = 0x11;
    pub const ALM0WKDAY: u8 = 0x0D;
    pub const ALM1WKDAY: u8 = 0x14;
    pub const PWRDNMIN: u8 = 0x18;
    pub const PWRUPMIN: u8 = 0x1C;
}

pub struct BitFlags;

#[allow(unused)]
impl BitFlags {
    pub const ST: u8 = 0b1000_0000;
    pub const OSCRUN: u8 = 0b0010_0000;
    pub const PWRFAIL: u8 = 0b0001_0000;
    pub const LEAPYEAR: u8 = 0b0010_0000;
    pub const VBATEN: u8 = 0b0000_1000;
    pub const OUT: u8 = 0b1000_0000;
    pub const SQWEN: u8 = 0b0100_0000;
    pub const EXTOSC: u8 = 0b0000_1000;
    pub const CRSTRIM: u8 = 0b0000_0100;
    pub const ALMPOL: u8 = 0b1000_0000;
    pub const ALM0EN: u8 = 0b0001_0000;
    pub const ALM1EN: u8 = 0b0010_0000;
    pub const ALMIF: u8 = 0b0000_1000;
}

pub fn new_mcp7940n(
    transactions: &[I2cTrans],
) -> Mcp794xx<interface::I2cInterface<I2cMock>, ic::Mcp7940n> {
    Mcp794xx::new_mcp7940n(I2cMock::new(&transactions))
}

pub fn destroy_mcp7940n(dev: Mcp794xx<interface::I2cInterface<I2cMock>, ic::Mcp7940n>) {
    dev.destroy_mcp7940n().done();
}

#[macro_export]
macro_rules! get_test {
    ($name:ident, $create_method:ident, $destroy_method:ident,
    $method:ident, $transactions:expr, $expected:expr $(, $value:expr)*) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            assert_eq!($expected, dev.$method($($value),*).unwrap());
            $destroy_method(dev);
        }
    };
}

#[macro_export]
macro_rules! assert_invalid_input_data {
    ($result:expr) => {
        match $result {
            Err(Error::InvalidInputData) => (),
            _ => panic!("InvalidInputData error not returned."),
        }
    };
}

#[macro_export]
macro_rules! set_invalid_test {
    ($name:ident, $create_method:ident, $destroy_method:ident, $method:ident $(, $value:expr)*) => {
        #[test]
        fn $name() {
            let mut dev = $create_method(&[]);
            assert_invalid_input_data!(dev.$method($($value),*));
            $destroy_method(dev);
        }
    };
}

#[macro_export]
macro_rules! set_invalid_param_test {
    ($name:ident, $method:ident $(, $value:expr)*) => {
        mod $name {
            use super::*;
            for_all_ics!(cannot_set_invalid, set_invalid_test, $method, $($value),*);
        }
    };
}

#[macro_export]
macro_rules! call_test {
    ($name:ident, $create_method:ident, $destroy_method:ident, $method:ident, $transactions:expr
    $(, $value:expr)*) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            dev.$method($($value),*).unwrap();
            $destroy_method(dev);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics {
    ($name:ident, $macroname:ident, $( $args:tt ),*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp7940n, new_mcp7940n, destroy_mcp7940n, $($args),*);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics_with_bat_power {
    ($name:ident, $macroname:ident, $( $args:tt ),*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp7940n, new_mcp7940n, destroy_mcp7940n, $($args),*);
        }
    };
}

#[macro_export]
macro_rules! get_param_test {
    ($name:ident, $method:ident, $register1:ident, $value:expr, [ $( $read_bin:expr ),+ ]) => {
        for_all_ics!(
            $name, get_test, $method,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register1], vec![$( $read_bin ),*]) ],
            $value);
    };
    ($name:ident, $method:ident, $arg:expr, $register1:ident, $value:expr, [ $( $read_bin:expr ),+ ]) => {
        for_all_ics!(
            $name, get_test, $method,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register1], vec![$( $read_bin ),*]) ],
            $value, $arg);
    };
}

#[macro_export]
macro_rules! set_param_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, [$( $binary_value:expr ),+]) => {
        for_all_ics!(
            $name,
            call_test,
            $method,
            [I2cTrans::write(
                DEV_ADDR,
                vec![Register::$register, $($binary_value),*]
            )],
            $value
        );
    };
}
