extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::{OutputPinLevel, SqWFreq};
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

set_param_test!(
    set_out_high,
    set_output_pin,
    CONTROL,
    OutputPinLevel::High,
    [BitFlags::OUT]
);

set_param_test!(
    set_out_low,
    set_output_pin,
    CONTROL,
    OutputPinLevel::Low,
    [0]
);

set_control_test!(
    en_trim,
    enable_coarse_trim,
    BitFlags::OUT | BitFlags::CRSTRIM
);
set_control_test!(dis_trim, disable_coarse_trim, BitFlags::OUT);

set_param_test!(set_trim_0, set_trimming, OSCTRIM, 0, [0]);
set_param_test!(set_trim_254, set_trimming, OSCTRIM, 127, [127]);
set_param_test!(set_trim_m128, set_trimming, OSCTRIM, -128, [0b1000_0000]);
set_param_test!(set_trim_m252, set_trimming, OSCTRIM, -127, [0b1111_1111]);
set_param_test!(set_trim_m4, set_trimming, OSCTRIM, -2, [0b1000_0010]);
set_param_test!(set_trim_m2, set_trimming, OSCTRIM, -1, [0b1000_0001]);
