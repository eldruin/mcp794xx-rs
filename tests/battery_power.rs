extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::{Hours, PowerFailDateTime};
mod common;
use common::{
    destroy_mcp79400, destroy_mcp79401, destroy_mcp79402, destroy_mcp7940n, destroy_mcp79410,
    destroy_mcp79411, destroy_mcp79412, new_mcp79400, new_mcp79401, new_mcp79402, new_mcp7940n,
    new_mcp79410, new_mcp79411, new_mcp79412, BitFlags, Register, DEVICE_ADDRESS as DEV_ADDR,
};

const PFDT: PowerFailDateTime = PowerFailDateTime {
    minute: 59,
    hour: Hours::H24(23),
    day: 26,
    weekday: 5,
    month: 12,
};

for_all_ics_with_bat_power!(
    get_power_down_date_time,
    get_test,
    get_power_down_datetime,
    [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::PWRDNMIN],
        vec![0b0101_1001, 0b0010_0011, 0b0010_0110, 0b1011_0010]
    )],
    PFDT
);

for_all_ics_with_bat_power!(
    get_power_up_date_time,
    get_test,
    get_power_up_datetime,
    [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::PWRUPMIN],
        vec![0b0101_1001, 0b0010_0011, 0b0010_0110, 0b1011_0010]
    )],
    PFDT
);

macro_rules! call_update_method {
    ($name:ident, $method:ident, $register:ident, $value:expr) => {
        for_all_ics_with_bat_power!(
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

call_update_method!(can_clear_pwr_fail, clear_power_failed, WEEKDAY, 0);

call_update_method!(
    can_enable_vbat,
    enable_backup_battery_power,
    WEEKDAY,
    BitFlags::VBATEN
);

call_update_method!(can_disable_vbat, disable_backup_battery_power, WEEKDAY, 0);

for_all_ics_with_bat_power!(
    power_failed,
    get_test,
    has_power_failed,
    [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::WEEKDAY],
        vec![BitFlags::PWRFAIL]
    )],
    true
);

for_all_ics_with_bat_power!(
    power_not_failed,
    get_test,
    has_power_failed,
    [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::WEEKDAY],
        vec![!BitFlags::PWRFAIL]
    )],
    false
);
