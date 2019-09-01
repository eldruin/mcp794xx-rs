extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::Error;
mod common;
use common::{destroy_mcp7940n, new_mcp7940n, DEVICE_ADDRESS as DEV_ADDR};

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
