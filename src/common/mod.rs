use super::Config;
pub mod conversion;
pub mod datetime;

impl Config {
    pub(crate) fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    pub(crate) fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}
