//! Data types
use rtcc::Hours;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C/SPI bus error
    Comm(E),
    /// Invalid input data provided
    InvalidInputData,
    /// Invalid time set in device: (hours, minutes, seconds)
    InvalidTimeData(u32, u32, u32),
    /// Invalid date set in device: (years, months, days)
    InvalidDateData(u32, u32, u32),
}

/// Square-wave output frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqWFreq {
    /// 1 Hz (default)
    Hz1,
    /// 4.096 Hz
    Hz4_096,
    /// 8.192 Hz
    Hz8_192,
    /// 32.768 Hz
    Hz32_768,
}

/// General purpose output pin logic level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputPinLevel {
    /// High
    High,
    /// Low
    Low,
}

/// Alarm interrupt output pin polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlarmOutputPinPolarity {
    /// High logic level when alarm asserted
    High,
    /// Low logic level when alarm asserted
    Low,
}

/// Alarm trigger rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlarmMatching {
    /// Alarm triggers when seconds match.
    SecondsMatch,
    /// Alarm triggers when minutes match.
    MinutesMatch,
    /// Alarm triggers when hours match.
    HoursMatch,
    /// Alarm triggers when weekday matches.
    WeekdayMatches,
    /// Alarm triggers when day (date/day of month) matches.
    DayMatches,
    /// Alarm triggers when seconds, minutes, hours, weekday, day and month match.
    AllMatch,
}

/// Alarm selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alarm {
    /// Alarm 0
    Zero,
    /// Alarm 1
    One,
}

/// Alarm date/time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlarmDateTime {
    /// Month [1-12]
    pub month: u8,
    /// Day [1-31]
    pub day: u8,
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour in 24h/12h format (format matches RTC)
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8,
}

/// Power fail date/time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerFailDateTime {
    /// Month [1-12]
    pub month: u8,
    /// Day [1-31]
    pub day: u8,
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour in 24h/12h format (format matches RTC)
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
}

/// EEPROM block write protection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EepromWriteProtection {
    /// None of the addresses is write-protected (default)
    None,
    /// Upper 1/4 of the addresses are write-protected: `[0x60-0x7F]`
    UpperQuarter,
    /// Upper 1/2 of the addresses are write-protected: `[0x40-0x7F]`
    UpperHalf,
    /// All of the addresses are write-protected: `[0x00-0x7F]`
    All,
}
