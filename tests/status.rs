extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate mcp794xx;
use mcp794xx::{Hours, PowerFailDateTime};
mod common;
use common::{destroy_mcp7940n, new_mcp7940n, Register, DEVICE_ADDRESS as DEV_ADDR};

get_param_test!(
    get_power_down_date_time,
    get_power_down_datetime,
    PWRDNMIN,
    PowerFailDateTime {
        minute: 59,
        hour: Hours::H24(23),
        day: 26,
        weekday: 5,
        month: 12,
    },
    [0b0101_1001, 0b0010_0011, 0b0010_0110, 0b1011_0010]
);

get_param_test!(
    get_power_up_date_time,
    get_power_up_datetime,
    PWRUPMIN,
    PowerFailDateTime {
        minute: 59,
        hour: Hours::H24(23),
        day: 26,
        weekday: 5,
        month: 12,
    },
    [0b0101_1001, 0b0010_0011, 0b0010_0110, 0b1011_0010]
);
