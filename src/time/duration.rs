/// A Duration represents the elapsed time between two instants
/// as an int64 nanosecond count. The representation limits the
/// largest representable duration to approximately 290 years.
type Duration = i64;

/// Common durations. There is no definition for units of Day or larger
/// to avoid confusion across daylight savings time zone transitions.
pub const NANOSECOND: Duration = 1;
pub const MICROSECOND: Duration = 1000 * NANOSECOND;
pub const MILLISECOND: Duration = 1000 * MICROSECOND;
pub const SECOND: Duration = 1000 * MILLISECOND;
pub const MINUTE: Duration = 60 * SECOND;
pub const HOUR: Duration = 60 * MINUTE;
