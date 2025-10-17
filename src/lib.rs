//! Daku time types
//!
//! This crate contains the time types defined by the Daku spec, with conversion
//! logic.  It is not required to use this crate with Daku.
//!
//! <https://ardaku.org/daku>

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg"
)]
#![no_std]
#![forbid(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::unescaped_backticks,
    rustdoc::redundant_explicit_links
)]

mod date;
mod date_time;
mod femtos;
#[allow(dead_code)] // FIXME
mod leap_second;
mod micros;
mod millis;
mod naive_days;
mod naive_hours;
mod naive_minutes;
mod naive_weeks;
mod nanos;
mod picos;
mod secs;
mod time;
mod time_adjustment;
mod time_designation;
mod time_zone;
mod timestamp;

pub use self::{
    date::Date, date_time::DateTime, femtos::Femtos, micros::Micros,
    millis::Millis, naive_days::NaïveDays, naive_hours::NaïveHours,
    naive_minutes::NaïveMinutes, naive_weeks::NaïveWeeks, nanos::Nanos,
    picos::Picos, secs::Secs, time::Time, time_adjustment::TimeAdjustment,
    time_designation::TimeDesignation, time_zone::TimeZone,
    timestamp::Timestamp,
};
pub use crate::{
    NaïveDays as NaiveDays, NaïveHours as NaiveHours,
    NaïveMinutes as NaiveMinutes, NaïveWeeks as NaiveWeeks,
};
