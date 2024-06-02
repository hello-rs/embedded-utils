//! The time library provides the function of processing time.

mod datetime;
#[allow(dead_code)]
mod duration;
mod timezone;

pub use datetime::DateTime;
pub use timezone::TimeZone;
