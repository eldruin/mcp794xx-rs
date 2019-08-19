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
call_update_method!(can_clear_pwr_fail, clear_power_failed, WEEKDAY, 0);

call_update_method!(
    can_enable_vbat,
    enable_backup_battery_power,
    WEEKDAY,
    BitFlags::VBATEN
);

call_update_method!(can_disable_vbat, disable_backup_battery_power, WEEKDAY, 0);

get_param_test!(
    osc_running,
    is_oscillator_running,
    WEEKDAY,
    true,
    [BitFlags::OSCRUN]
);
get_param_test!(
    osc_not_running,
    is_oscillator_running,
    WEEKDAY,
    false,
    [!BitFlags::OSCRUN]
);

get_param_test!(
    power_failed,
    has_power_failed,
    WEEKDAY,
    true,
    [BitFlags::PWRFAIL]
);
get_param_test!(
    power_not_failed,
    has_power_failed,
    WEEKDAY,
    false,
    [!BitFlags::PWRFAIL]
);
