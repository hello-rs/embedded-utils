/// Timezone enum.
#[derive(Clone, Copy, Default, core::fmt::Debug)]
pub enum TimeZone {
    /// Default UTC.
    #[default]
    UTC,
    /// Asia/Shanghai timezone.
    AsiaShanghai,
}

impl TimeZone {
    /// Get the offset of the timezone (in seconds)
    pub fn get_offset(&self) -> i32 {
        match self {
            TimeZone::UTC => 0,
            TimeZone::AsiaShanghai => 8 * 3600,
        }
    }
}
