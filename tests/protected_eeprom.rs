extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::Error;
mod common;
use common::{
    destroy_mcp79400, destroy_mcp79401, destroy_mcp79402, destroy_mcp79410, destroy_mcp79411,
    destroy_mcp79412, new_mcp79400, new_mcp79401, new_mcp79402, new_mcp79410, new_mcp79411,
    new_mcp79412, DEVICE_ADDRESS as DEV_ADDR, EEPROM_ADDRESS,
};
const EEUNLOCK: u8 = 0b0000_1001;

macro_rules! set_invalid_eeprom_test {
    ($name:ident, $method:ident $(, $value:expr)*) => {
        mod $name {
            use super::*;
            for_all_ics_with_protected_eeprom!(cannot_set_invalid, set_invalid_test, $method, $($value),*);
        }
    };
}

set_invalid_eeprom_test!(
    read_eeprom_byte_too_small_address,
    read_protected_eeprom_byte,
    0xEF
);
set_invalid_eeprom_test!(
    read_eeprom_byte_too_big_address,
    read_protected_eeprom_byte,
    0xF8
);

for_all_ics_with_protected_eeprom!(
    can_read_byte,
    get_test,
    read_protected_eeprom_byte,
    [I2cTrans::write_read(EEPROM_ADDRESS, vec![0xF0], vec![15])],
    15,
    0xF0
);

for_all_ics_with_protected_eeprom!(
    can_read_current_byte,
    get_test,
    read_eeprom_current_byte,
    [I2cTrans::read(EEPROM_ADDRESS, vec![15])],
    15
);

set_invalid_eeprom_test!(
    read_data_too_sml_addr,
    read_protected_eeprom_data,
    0xEF,
    &mut [0]
);
set_invalid_eeprom_test!(
    read_data_too_big_addr,
    read_protected_eeprom_data,
    0xF8,
    &mut [0]
);
set_invalid_eeprom_test!(
    read_data_too_much1,
    read_protected_eeprom_data,
    0xF7,
    &mut [0; 2]
);
set_invalid_eeprom_test!(
    read_data_too_much2,
    read_protected_eeprom_data,
    0xF0,
    &mut [0; 9]
);

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

for_all_ics_with_protected_eeprom!(
    can_read_data,
    read_data_test,
    read_protected_eeprom_data,
    [I2cTrans::write_read(
        EEPROM_ADDRESS,
        vec![0xF0],
        vec![1, 2, 3, 4, 5]
    )],
    0xF0,
    [1, 2, 3, 4, 5]
);

for_all_ics_with_eui48!(
    can_read_eui48,
    get_test,
    read_eui48,
    [I2cTrans::write_read(
        EEPROM_ADDRESS,
        vec![0xF2],
        vec![1, 2, 3, 4, 5, 6]
    )],
    [1, 2, 3, 4, 5, 6]
);

for_all_ics_with_eui64!(
    can_read_eui64,
    get_test,
    read_eui64,
    [I2cTrans::write_read(
        EEPROM_ADDRESS,
        vec![0xF0],
        vec![1, 2, 3, 4, 5, 6, 7, 8]
    )],
    [1, 2, 3, 4, 5, 6, 7, 8]
);

set_invalid_eeprom_test!(
    write_byte_too_small_address,
    write_protected_eeprom_byte,
    0xEF,
    0
);
set_invalid_eeprom_test!(
    write_byte_too_big_address,
    write_protected_eeprom_byte,
    0xF8,
    0
);

for_all_ics_with_protected_eeprom!(
    can_write_byte,
    call_test,
    write_protected_eeprom_byte,
    [
        I2cTrans::write(DEV_ADDR, vec![EEUNLOCK, 0x55]),
        I2cTrans::write(DEV_ADDR, vec![EEUNLOCK, 0xAA]),
        I2cTrans::write(EEPROM_ADDRESS, vec![0xF0, 15])
    ],
    0xF0,
    15
);

set_invalid_eeprom_test!(
    write_data_too_sml_addr,
    write_protected_eeprom_data,
    0xEF,
    &[0]
);
set_invalid_eeprom_test!(
    write_data_too_big_addr,
    write_protected_eeprom_data,
    0xF8,
    &[0]
);
set_invalid_eeprom_test!(
    write_data_too_much1,
    write_protected_eeprom_data,
    0xF7,
    &[0; 2]
);
set_invalid_eeprom_test!(
    write_data_too_much2,
    write_protected_eeprom_data,
    0xF0,
    &[0; 9]
);

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

for_all_ics_with_protected_eeprom!(
    can_write_data,
    write_data_test,
    write_protected_eeprom_data,
    [
        I2cTrans::write(DEV_ADDR, vec![EEUNLOCK, 0x55]),
        I2cTrans::write(DEV_ADDR, vec![EEUNLOCK, 0xAA]),
        I2cTrans::write(EEPROM_ADDRESS, vec![0xF0, 1, 2, 3, 4, 5])
    ],
    0xF0,
    [1, 2, 3, 4, 5]
);
