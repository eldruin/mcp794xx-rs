extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy_mcp7940n, new_mcp7940n, BitFlags, Register, DEVICE_ADDRESS as DEV_ADDR};

macro_rules! call_update_method {
    ($name:ident, $method:ident, $register:ident, $value:expr) => {
        for_all_ics!(
            $name,
            call_test,
            $method,
            [
                I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![0]),
                I2cTrans::write(DEV_ADDR, vec![Register::$register, $value])
            ]
        );
    };
}

call_update_method!(can_enable, enable, SECONDS, BitFlags::ST);
call_update_method!(can_disable, disable, SECONDS, 0);
