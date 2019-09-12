extern crate embedded_hal_mock as hal;
use crate::hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::{EepromWriteProtection, Error};
mod common;
use crate::common::{
    destroy_mcp79410, destroy_mcp79411, destroy_mcp79412, new_mcp79410, new_mcp79411, new_mcp79412,
    EEPROM_ADDRESS,
};

macro_rules! set_invalid_eeprom_test {
    ($name:ident, $method:ident $(, $value:expr)*) => {
        mod $name {
            use super::*;
            for_all_ics_with_eeprom!(cannot_set_invalid, set_invalid_test, $method, $($value),*);
        }
    };
}

set_invalid_eeprom_test!(read_byte_invalid1, read_eeprom_byte, 0x80);
set_invalid_eeprom_test!(read_byte_invalid2, read_eeprom_byte, 0xF0);

for_all_ics_with_eeprom!(
    can_read_byte,
    get_test,
    read_eeprom_byte,
    [I2cTrans::write_read(EEPROM_ADDRESS, vec![0x00], vec![15])],
    15,
    0x00
);

for_all_ics_with_eeprom!(
    can_read_current_byte,
    get_test,
    read_eeprom_current_byte,
    [I2cTrans::read(EEPROM_ADDRESS, vec![15])],
    15
);

set_invalid_eeprom_test!(read_data_invalid1, read_eeprom_data, 0x80, &mut [0]);
set_invalid_eeprom_test!(read_data_invalid2, read_eeprom_data, 0xF0, &mut [0]);
set_invalid_eeprom_test!(read_data_too_much1, read_eeprom_data, 0x7F, &mut [0; 2]);
set_invalid_eeprom_test!(read_data_too_much2, read_eeprom_data, 0x00, &mut [0; 129]);

#[macro_export]
macro_rules! read_data_test {
    ($name:ident, $create_method:ident, $destroy_method:ident,
    $method:ident, $transactions:expr, $addr:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            let mut data = [0; 5];
            dev.$method($addr, &mut data).unwrap();
            assert_eq!($expected, data);
            $destroy_method(dev);
        }
    };
}

for_all_ics_with_eeprom!(
    can_read_data,
    read_data_test,
    read_eeprom_data,
    [I2cTrans::write_read(
        EEPROM_ADDRESS,
        vec![0x00],
        vec![1, 2, 3, 4, 5]
    )],
    0x00,
    [1, 2, 3, 4, 5]
);

set_invalid_eeprom_test!(write_byte_invalid1, write_eeprom_byte, 0x80, 0);
set_invalid_eeprom_test!(write_byte_invalid3, write_eeprom_byte, 0xF0, 0);

for_all_ics_with_eeprom!(
    can_write_byte,
    call_test,
    write_eeprom_byte,
    [I2cTrans::write(EEPROM_ADDRESS, vec![0x00, 15])],
    0x00,
    15
);

set_invalid_eeprom_test!(write_data_invalid1, write_eeprom_data, 0x80, &[0]);
set_invalid_eeprom_test!(write_data_invalid2, write_eeprom_data, 0xF0, &[0]);
set_invalid_eeprom_test!(write_data_too_much1, write_eeprom_data, 0x7F, &[0; 2]);
set_invalid_eeprom_test!(write_data_too_much2, write_eeprom_data, 0x00, &[0; 129]);

#[macro_export]
macro_rules! write_data_test {
    ($name:ident, $create_method:ident, $destroy_method:ident,
    $method:ident, $transactions:expr, $addr:expr, $value:expr) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            dev.$method($addr, &$value).unwrap();
            $destroy_method(dev);
        }
    };
}

for_all_ics_with_eeprom!(
    can_write_data,
    write_data_test,
    write_eeprom_data,
    [I2cTrans::write(EEPROM_ADDRESS, vec![0x00, 1, 2, 3, 4, 5])],
    0x00,
    [1, 2, 3, 4, 5]
);

mod write_protection {
    use super::*;
    const EEPROM_STATUS: u8 = 0xFF;
    const NONE: EepromWriteProtection = EepromWriteProtection::None;
    const UPPERQUARTER: EepromWriteProtection = EepromWriteProtection::UpperQuarter;
    const UPPERHALF: EepromWriteProtection = EepromWriteProtection::UpperHalf;
    const ALL: EepromWriteProtection = EepromWriteProtection::All;

    for_all_ics_with_eeprom!(
        can_set_protection_none,
        call_test,
        set_eeprom_write_protection,
        [I2cTrans::write(EEPROM_ADDRESS, vec![EEPROM_STATUS, 0])],
        NONE
    );
    for_all_ics_with_eeprom!(
        can_set_protection_upper_quarter,
        call_test,
        set_eeprom_write_protection,
        [I2cTrans::write(
            EEPROM_ADDRESS,
            vec![EEPROM_STATUS, 0b0000_0100]
        )],
        UPPERQUARTER
    );
    for_all_ics_with_eeprom!(
        can_set_protection_upper_half,
        call_test,
        set_eeprom_write_protection,
        [I2cTrans::write(
            EEPROM_ADDRESS,
            vec![EEPROM_STATUS, 0b0000_1000]
        )],
        UPPERHALF
    );
    for_all_ics_with_eeprom!(
        can_set_protection_all,
        call_test,
        set_eeprom_write_protection,
        [I2cTrans::write(
            EEPROM_ADDRESS,
            vec![EEPROM_STATUS, 0b0000_1100]
        )],
        ALL
    );
}
