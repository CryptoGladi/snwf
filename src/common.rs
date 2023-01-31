//! Module for general things.
//!
//! **Not for user code!**

pub(crate) mod constant;
pub(crate) mod macros;

pub(crate) use constant::*;
pub(crate) use macros::*;

/// Init logger for **tests**
#[cfg(test)]
pub fn init_logger_for_test() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Warn)
        .filter_module("snwf", log::LevelFilter::Debug)
        .try_init();
}
