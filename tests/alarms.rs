extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy_mcp7940n, new_mcp7940n, BitFlags, Register, DEVICE_ADDRESS as DEV_ADDR};
extern crate mcp794xx;
use mcp794xx::{
    Alarm, AlarmDateTime, AlarmMatching, AlarmOutputPinPolarity, DateTime, Error, Hours, Rtcc,
};

macro_rules! invalid_dt_test {
    ($name:ident, $month:expr, $day:expr, $weekday:expr,
     $hour:expr, $minute:expr, $second:expr) => {
        mod $name {
            use super::*;
            const ADT: AlarmDateTime = AlarmDateTime {
                month: $month,
                day: $day,
                weekday: $weekday,
                hour: $hour,
                minute: $minute,
                second: $second,
            };
            set_invalid_param_test!(
                $name,
                set_alarm,
                Alarm::Zero,
                ADT,
                AlarmMatching::AllMatch,
                AlarmOutputPinPolarity::High
            );
        }
    };
}

#[macro_export]
macro_rules! call_set_alarm_test {
    ($name:ident, $create_method:ident, $destroy_method:ident, $transactions:expr
    $(, $value:expr)*) => {
        #[test]
        fn $name() {
            const DT:DateTime = DateTime {
        year: 2019,
        month: 11,
        day: 13,
        weekday: 2,
        hour: Hours::H24(23),
        minute: 59,
        second: 58,
    };
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            dev.set_datetime(&DT).unwrap();
            dev.set_alarm($($value),*).unwrap();
            $destroy_method(dev);
        }
    };
}

macro_rules! set_alarm_test {
    ($name:ident, [$($value:expr),*], [$( $binary_value:expr ),+]) => {
        mod $name {
            use super::*;
            const ALM0: Alarm = Alarm::Zero;
            for_all_ics!(
                for_alm0,
                call_set_alarm_test,
                [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::SECONDS,
                        0b0101_1000,
                        0b0101_1001,
                        0b0010_0011,
                        0b0000_0010,
                        0b0001_0011,
                        0b0001_0001,
                        0b0001_1001
                    ]
                ),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::ALM0SEC, $($binary_value),*]
                )],
                ALM0, $($value),*
            );

            const ALM1: Alarm = Alarm::One;
            for_all_ics!(
                for_alm1,
                call_set_alarm_test,
                [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::SECONDS,
                        0b0101_1000,
                        0b0101_1001,
                        0b0010_0011,
                        0b0000_0010,
                        0b0001_0011,
                        0b0001_0001,
                        0b0001_1001
                    ]
                ),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::ALM1SEC, $($binary_value),*]
                )],
                ALM1, $($value),*
            );
        }
    };
}

macro_rules! set_alarm_test_variation {
    ($name:ident, [$($value:expr),*], $weekday_mask:expr) => {
        set_alarm_test!(
        $name,
        [$($value),*],
        [
            0b0100_0001,
            0b0011_0010,
            0b0001_0101,
            0b0000_0100 | $weekday_mask,
            0b0000_0011,
            0b0001_0001
        ]
    );
    };
}

mod set_alarm {
    use super::*;

    const ADT: AlarmDateTime = AlarmDateTime {
        month: 11,
        day: 3,
        weekday: 4,
        hour: Hours::H24(15),
        minute: 32,
        second: 41,
    };

    set_alarm_test_variation!(
        basic,
        [
            ADT,
            AlarmMatching::SecondsMatch,
            AlarmOutputPinPolarity::Low
        ],
        0
    );
    set_alarm_test_variation!(
        minutes_match,
        [
            ADT,
            AlarmMatching::MinutesMatch,
            AlarmOutputPinPolarity::Low
        ],
        0b0001_0000
    );
    set_alarm_test_variation!(
        hours_match,
        [ADT, AlarmMatching::HoursMatch, AlarmOutputPinPolarity::Low],
        0b0010_0000
    );
    set_alarm_test_variation!(
        weekday_matches,
        [
            ADT,
            AlarmMatching::WeekdayMatches,
            AlarmOutputPinPolarity::Low
        ],
        0b0011_0000
    );
    set_alarm_test_variation!(
        day_matches,
        [ADT, AlarmMatching::DayMatches, AlarmOutputPinPolarity::Low],
        0b0100_0000
    );
    set_alarm_test_variation!(
        all_match,
        [ADT, AlarmMatching::AllMatch, AlarmOutputPinPolarity::Low],
        0b0111_0000
    );

    const ADT_3PM: AlarmDateTime = AlarmDateTime {
        month: 11,
        day: 3,
        weekday: 4,
        hour: Hours::PM(3),
        minute: 32,
        second: 41,
    };

    set_alarm_test_variation!(
        hours_is_adapted_to_running_mode,
        [
            ADT_3PM,
            AlarmMatching::SecondsMatch,
            AlarmOutputPinPolarity::Low
        ],
        0
    );

    mod set_high_polarity {
        use super::*;
        const MATCH: AlarmMatching = AlarmMatching::SecondsMatch;
        const POL: AlarmOutputPinPolarity = AlarmOutputPinPolarity::High;
        const ALM0: Alarm = Alarm::Zero;
        for_all_ics!(
            alm0_high_polarity,
            call_set_alarm_test,
            [
                I2cTrans::write(
                    DEV_ADDR,
                    vec![
                        Register::SECONDS,
                        0b0101_1000,
                        0b0101_1001,
                        0b0010_0011,
                        0b0000_0010,
                        0b0001_0011,
                        0b0001_0001,
                        0b0001_1001
                    ]
                ),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![
                        Register::ALM0SEC,
                        0b0100_0001,
                        0b0011_0010,
                        0b0001_0101,
                        0b0000_0100 | BitFlags::ALMPOL,
                        0b0000_0011,
                        0b0001_0001
                    ]
                )
            ],
            ALM0,
            ADT,
            MATCH,
            POL
        );

        const ALM1: Alarm = Alarm::One;
        for_all_ics!(
            alarm1_high_polarity_changes_alarm0_polarity_if_it_does_not_match,
            call_set_alarm_test,
            [
                I2cTrans::write(
                    DEV_ADDR,
                    vec![
                        Register::SECONDS,
                        0b0101_1000,
                        0b0101_1001,
                        0b0010_0011,
                        0b0000_0010,
                        0b0001_0011,
                        0b0001_0001,
                        0b0001_1001
                    ]
                ),
                I2cTrans::write_read(DEV_ADDR, vec![Register::ALM0WKDAY], vec![0b0101_0110]),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::ALM0WKDAY, 0b0101_0110 | BitFlags::ALMPOL]
                ),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![
                        Register::ALM1SEC,
                        0b0100_0001,
                        0b0011_0010,
                        0b0001_0101,
                        0b0000_0100 | BitFlags::ALMPOL,
                        0b0000_0011,
                        0b0001_0001
                    ]
                )
            ],
            ALM1,
            ADT,
            MATCH,
            POL
        );
    }

    invalid_dt_test!(too_small_month, 0, 3, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_month, 13, 3, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_day, 11, 0, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_day, 11, 32, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_wd, 11, 3, 0, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_wd, 11, 3, 8, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_hours, 11, 3, 2, Hours::H24(24), 59, 58);
    invalid_dt_test!(too_big_min, 11, 3, 2, Hours::H24(24), 60, 58);
    invalid_dt_test!(too_big_seconds, 11, 3, 2, Hours::H24(24), 59, 60);
}

set_param_test!(
    en_alm0,
    enable_alarm,
    CONTROL,
    Alarm::Zero,
    [BitFlags::OUT | BitFlags::ALM0EN]
);
set_param_test!(
    en_alm1,
    enable_alarm,
    CONTROL,
    Alarm::One,
    [BitFlags::OUT | BitFlags::ALM1EN]
);
set_param_test!(
    dis_alm0,
    disable_alarm,
    CONTROL,
    Alarm::Zero,
    [BitFlags::OUT]
);
set_param_test!(
    dis_alm1,
    disable_alarm,
    CONTROL,
    Alarm::One,
    [BitFlags::OUT]
);

get_param_test!(
    alm0_not_matched,
    has_alarm_matched,
    Alarm::Zero,
    ALM0WKDAY,
    false,
    [0]
);

get_param_test!(
    alm0_matched,
    has_alarm_matched,
    Alarm::Zero,
    ALM0WKDAY,
    true,
    [BitFlags::ALMIF]
);

get_param_test!(
    alm1_not_matched,
    has_alarm_matched,
    Alarm::One,
    ALM1WKDAY,
    false,
    [0]
);

get_param_test!(
    alm1_matched,
    has_alarm_matched,
    Alarm::One,
    ALM1WKDAY,
    true,
    [BitFlags::ALMIF]
);

macro_rules! call_update_method {
    ($name:ident, $method:ident, $arg:expr, $register:ident, $read:expr, $value:expr) => {
        for_all_ics!(
            $name,
            call_test,
            $method,
            [
                I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$read]),
                I2cTrans::write(DEV_ADDR, vec![Register::$register, $value])
            ],
            $arg
        );
    };
}

call_update_method!(
    clear_alarm0_matched,
    clear_alarm_matched_flag,
    Alarm::Zero,
    ALM0WKDAY,
    BitFlags::ALMIF,
    0
);
call_update_method!(
    clear_alarm1_matched,
    clear_alarm_matched_flag,
    Alarm::One,
    ALM1WKDAY,
    BitFlags::ALMIF,
    0
);
