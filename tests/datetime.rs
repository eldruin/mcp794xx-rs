extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy_mcp7940n, new_mcp7940n, BitFlags, Register, DEVICE_ADDRESS as DEV_ADDR};
extern crate mcp794xx;
use mcp794xx::{DateTime, Error, Hours, Rtcc};

macro_rules! set_invalid_param_range_test {
    ($name:ident, $method:ident, $too_small_value:expr, $too_big_value:expr) => {
        mod $name {
            use super::*;
            for_all_ics!(too_small, set_invalid_test, $method, $too_small_value);
            for_all_ics!(too_big, set_invalid_test, $method, $too_big_value);
        }
    };
}

mod seconds {
    use super::*;
    get_param_test!(get, get_seconds, SECONDS, 12, [18]);
    set_param_test!(set, set_seconds, SECONDS, 12, [18]);
    set_invalid_param_test!(invalid, set_seconds, 60);
}

mod minutes {
    use super::*;
    get_param_test!(get, get_minutes, MINUTES, 13, [19]);
    set_param_test!(set, set_minutes, MINUTES, 13, [19]);
    set_invalid_param_test!(invalid, set_minutes, 60);
}

mod hours_24h {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::H24(21), [0b0010_0001]);
    set_param_test!(set, set_hours, HOURS, Hours::H24(21), [0b0010_0001]);
    set_invalid_param_test!(invalid, set_hours, Hours::H24(24));
}

mod hours_12h_am {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::AM(12), [0b0101_0010]);
    set_param_test!(set, set_hours, HOURS, Hours::AM(12), [0b0101_0010]);
    set_invalid_param_range_test!(invalid, set_hours, Hours::AM(0), Hours::AM(13));
}

mod hours_12h_pm {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::PM(12), [0b0111_0010]);
    set_param_test!(set, set_hours, HOURS, Hours::PM(12), [0b0111_0010]);
    set_invalid_param_range_test!(invalid, set_hours, Hours::PM(0), Hours::PM(13));
}

mod weekday {
    use super::*;
    get_param_test!(get, get_weekday, WEEKDAY, 5, [5]);
    set_param_test!(set, set_weekday, WEEKDAY, 7, [7]);
    set_invalid_param_range_test!(invalid, set_weekday, 0, 8);
}

mod day {
    use super::*;
    get_param_test!(get, get_day, DAY, 23, [0b0010_0011]);
    set_param_test!(set, set_day, DAY, 31, [0b0011_0001]);
    set_invalid_param_range_test!(invalid, set_day, 0, 32);
}

mod month {
    use super::*;
    get_param_test!(get, get_month, MONTH, 12, [0b0001_0010]);
    set_param_test!(set, set_month, MONTH, 9, [0b0000_1001]);
    set_invalid_param_range_test!(invalid, set_month, 0, 13);
}

mod year {
    use super::*;
    get_param_test!(get, get_year, YEAR, 2045, [0b0100_0101]);
    set_param_test!(set, set_year, YEAR, 2098, [0b1001_1000]);
    set_invalid_param_test!(invalid, set_year, 2100);
}

macro_rules! invalid_dt_test {
    ($name:ident, $year:expr, $month:expr, $day:expr, $weekday:expr,
     $hour:expr, $minute:expr, $second:expr) => {
        mod $name {
            use super::*;
            const DT: DateTime = DateTime {
                year: $year,
                month: $month,
                day: $day,
                weekday: $weekday,
                hour: $hour,
                minute: $minute,
                second: $second,
            };
            set_invalid_param_test!($name, set_datetime, &DT);
        }
    };
}

mod datetime {
    use super::*;
    const DT: DateTime = DateTime {
        year: 2018,
        month: 8,
        day: 13,
        weekday: 2,
        hour: Hours::H24(23),
        minute: 59,
        second: 58,
    };
    get_param_test!(
        get,
        get_datetime,
        SECONDS,
        DT,
        [
            0b0101_1000,
            0b0101_1001,
            0b0010_0011,
            0b0000_0010,
            0b0001_0011,
            0b0000_1000,
            0b0001_1000
        ]
    );

    set_param_test!(
        set,
        set_datetime,
        SECONDS,
        &DT,
        [
            0b0101_1000,
            0b0101_1001,
            0b0010_0011,
            0b0000_0010,
            0b0001_0011,
            0b0000_1000,
            0b0001_1000
        ]
    );

    invalid_dt_test!(too_small_year, 1999, 8, 13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_year, 2100, 8, 13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_month, 2018, 0, 13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_month, 2018, 13, 13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_day, 2018, 8, 0, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_day, 2018, 8, 32, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_wd, 2018, 8, 13, 0, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_wd, 2018, 8, 13, 8, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_hours, 2018, 8, 13, 2, Hours::H24(24), 59, 58);
    invalid_dt_test!(too_big_min, 2018, 8, 13, 2, Hours::H24(24), 60, 58);
    invalid_dt_test!(too_big_seconds, 2018, 8, 13, 2, Hours::H24(24), 59, 60);
}

mod leapyear {
    use super::*;
    get_param_test!(yes, is_leap_year, MONTH, true, [BitFlags::LEAPYEAR]);
    get_param_test!(no, is_leap_year, MONTH, false, [!BitFlags::LEAPYEAR]);
}
