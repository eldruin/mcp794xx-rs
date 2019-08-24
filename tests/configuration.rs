extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::SqWFreq;
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

#[macro_export]
macro_rules! set_control_test {
    ($name:ident, $method:ident, $binary_value:expr) => {
        for_all_ics!(
            $name,
            call_test,
            $method,
            [I2cTrans::write(
                DEV_ADDR,
                vec![Register::CONTROL, $binary_value]
            )]
        );
    };
}

set_control_test!(
    en_extosc,
    enable_external_oscillator,
    BitFlags::OUT | BitFlags::EXTOSC
);
set_control_test!(dis_extosc, disable_external_oscillator, BitFlags::OUT);
set_param_test!(
    set_sqw_1hz,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::Hz1,
    [BitFlags::OUT]
);
set_param_test!(
    set_sqw_4hz,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::Hz4_096,
    [BitFlags::OUT | 1]
);
set_param_test!(
    set_sqw_8hz,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::Hz8_192,
    [BitFlags::OUT | 2]
);
set_param_test!(
    set_sqw_32hz,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::Hz32_768,
    [BitFlags::OUT | 3]
);
set_control_test!(en_sqw, enable_square_wave, BitFlags::OUT | BitFlags::SQWEN);
set_control_test!(dis_sqw, disable_square_wave, BitFlags::OUT);
