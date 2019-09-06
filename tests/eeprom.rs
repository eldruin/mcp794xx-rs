extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::Error;
mod common;
use common::{
    destroy_mcp79400, destroy_mcp79401, destroy_mcp79402, destroy_mcp79410, destroy_mcp79411,
    destroy_mcp79412, new_mcp79400, new_mcp79401, new_mcp79402, new_mcp79410, new_mcp79411,
    new_mcp79412, EEPROM_ADDRESS,
};

for_all_ics_with_protected_eeprom!(
    read_eeprom_byte_too_small_address,
    set_invalid_test,
    read_eeprom_byte,
    0xEF
);
for_all_ics_with_protected_eeprom!(
    read_eeprom_byte_too_big_address,
    set_invalid_test,
    read_eeprom_byte,
    0xF8
);

for_all_ics_with_protected_eeprom!(
    can_read_byte,
    get_test,
    read_eeprom_byte,
    [I2cTrans::write_read(EEPROM_ADDRESS, vec![0xF0], vec![15])],
    15,
    0xF0
);
