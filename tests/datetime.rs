use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{
    destroy_mcp79400, destroy_mcp79401, destroy_mcp79402, destroy_mcp7940m, destroy_mcp7940n,
    destroy_mcp79410, destroy_mcp79411, destroy_mcp79412, new_mcp79400, new_mcp79401, new_mcp79402,
    new_mcp7940m, new_mcp7940n, new_mcp79410, new_mcp79411, new_mcp79412, BitFlags, Register,
    DEVICE_ADDRESS as DEV_ADDR,
};
use mcp794xx::{DateTimeAccess, Error, Hours, NaiveDate, NaiveTime, Rtcc};

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
    get_param_test!(get, seconds, SECONDS, 12, [18]);
    set_param_test!(set, set_seconds, SECONDS, 12, [18]);
    set_invalid_param_test!(invalid, set_seconds, 60);
}

mod minutes {
    use super::*;
    get_param_test!(get, minutes, MINUTES, 13, [19]);
    set_param_test!(set, set_minutes, MINUTES, 13, [19]);
    set_invalid_param_test!(invalid, set_minutes, 60);
}

mod hours_24h {
    use super::*;
    get_param_test!(get, hours, HOURS, Hours::H24(21), [0b0010_0001]);
    set_param_test!(set, set_hours, HOURS, Hours::H24(21), [0b0010_0001]);
    set_invalid_param_test!(invalid, set_hours, Hours::H24(24));
}

mod hours_12h_am {
    use super::*;
    get_param_test!(get, hours, HOURS, Hours::AM(12), [0b0101_0010]);
    set_param_test!(set, set_hours, HOURS, Hours::AM(12), [0b0101_0010]);
    set_invalid_param_range_test!(invalid, set_hours, Hours::AM(0), Hours::AM(13));
}

mod hours_12h_pm {
    use super::*;
    get_param_test!(get, hours, HOURS, Hours::PM(12), [0b0111_0010]);
    set_param_test!(set, set_hours, HOURS, Hours::PM(12), [0b0111_0010]);
    set_invalid_param_range_test!(invalid, set_hours, Hours::PM(0), Hours::PM(13));
}

mod weekday {
    use super::*;
    get_param_test!(get, weekday, WEEKDAY, 5, [5]);
    set_param_test!(set, set_weekday, WEEKDAY, 7, [7]);
    set_invalid_param_range_test!(invalid, set_weekday, 0, 8);
}

mod day {
    use super::*;
    get_param_test!(get, day, DAY, 23, [0b0010_0011]);
    set_param_test!(set, set_day, DAY, 31, [0b0011_0001]);
    set_invalid_param_range_test!(invalid, set_day, 0, 32);
}

mod month {
    use super::*;
    get_param_test!(get, month, MONTH, 12, [0b0001_0010]);
    set_param_test!(set, set_month, MONTH, 9, [0b0000_1001]);
    set_invalid_param_range_test!(invalid, set_month, 0, 13);
}

mod year {
    use super::*;
    get_param_test!(get, year, YEAR, 2045, [0b0100_0101]);
    set_param_test!(set, set_year, YEAR, 2098, [0b1001_1000]);
    set_invalid_param_test!(invalid, set_year, 2100);
}

mod date {
    use super::*;
    get_param_test!(
        get,
        date,
        DAY,
        NaiveDate::from_ymd(2018, 8, 13),
        [0b0001_0011, 0b0000_1000, 0b0001_1000]
    );

    set_param_test!(
        set,
        set_date,
        WEEKDAY,
        &NaiveDate::from_ymd(2018, 8, 13),
        [0b0000_0010, 0b0001_0011, 0b0000_1000, 0b0001_1000]
    );
}

mod time {
    use super::*;
    get_param_test!(
        get,
        time,
        SECONDS,
        NaiveTime::from_hms(23, 59, 58),
        [0b0101_1000, 0b0101_1001, 0b0010_0011]
    );

    set_param_test!(
        set,
        set_time,
        SECONDS,
        &NaiveTime::from_hms(23, 59, 58),
        [0b0101_1000, 0b0101_1001, 0b0010_0011]
    );
}

mod datetime {
    use super::*;
    get_param_test!(
        get,
        datetime,
        SECONDS,
        NaiveDate::from_ymd(2018, 8, 13).and_hms(23, 59, 58),
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
        &NaiveDate::from_ymd(2018, 8, 13).and_hms(23, 59, 58),
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

    set_invalid_param_test!(
        too_small_year,
        set_datetime,
        &NaiveDate::from_ymd(1999, 1, 1).and_hms(1, 1, 1)
    );
    set_invalid_param_test!(
        too_big_year,
        set_datetime,
        &NaiveDate::from_ymd(2100, 1, 1).and_hms(1, 1, 1)
    );
}

mod leapyear {
    use super::*;
    get_param_test!(yes, is_leap_year, MONTH, true, [BitFlags::LEAPYEAR]);
    get_param_test!(no, is_leap_year, MONTH, false, [!BitFlags::LEAPYEAR]);
}
