extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::Error;
mod common;
use common::{
    destroy_mcp7940m, destroy_mcp7940n, new_mcp7940m, new_mcp7940n, DEVICE_ADDRESS as DEV_ADDR,
};

set_invalid_param_test!(read_sram_byte_too_small_address, read_sram_byte, 0x19);
set_invalid_param_test!(read_sram_byte_too_big_address, read_sram_byte, 0x60);

for_all_ics!(
    can_read_byte,
    get_test,
    read_sram_byte,
    [I2cTrans::write_read(DEV_ADDR, vec![0x20], vec![15])],
    15,
    0x20
);

set_invalid_param_test!(write_sram_byte_too_small_address, write_sram_byte, 0x19, 0);
set_invalid_param_test!(write_sram_byte_too_big_address, write_sram_byte, 0x60, 0);

for_all_ics!(
    can_write_byte,
    call_test,
    write_sram_byte,
    [I2cTrans::write(DEV_ADDR, vec![0x20, 15])],
    0x20,
    15
);

set_invalid_param_test!(read_sram_data_too_sml_addr, read_sram_data, 0x19, &mut [0]);
set_invalid_param_test!(read_sram_data_too_big_addr, read_sram_data, 0x60, &mut [0]);
set_invalid_param_test!(read_sram_data_too_much1, read_sram_data, 0x5F, &mut [0; 2]);
set_invalid_param_test!(read_sram_data_too_much2, read_sram_data, 0x20, &mut [0; 65]);

#[macro_export]
macro_rules! read_data_test {
    ($name:ident, $create_method:ident, $destroy_method:ident,
    $method:ident, $transactions:expr, $addr:expr, $value:expr, $expected:expr) => {
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

for_all_ics!(
    can_read_data,
    read_data_test,
    read_sram_data,
    [I2cTrans::write_read(
        DEV_ADDR,
        vec![0x20],
        vec![1, 2, 3, 4, 5]
    )],
    0x20,
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 5]
);

set_invalid_param_test!(write_sram_data_too_sml_addr, write_sram_data, 0x19, &[0]);
set_invalid_param_test!(write_sram_data_too_big_addr, write_sram_data, 0x60, &[0]);
set_invalid_param_test!(write_sram_data_too_much1, write_sram_data, 0x5F, &[0; 2]);
set_invalid_param_test!(write_sram_data_too_much2, write_sram_data, 0x20, &[0; 65]);

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

for_all_ics!(
    can_write_data,
    write_data_test,
    write_sram_data,
    [I2cTrans::write(DEV_ADDR, vec![0x20, 1, 2, 3, 4, 5])],
    0x20,
    [1, 2, 3, 4, 5]
);
