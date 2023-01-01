pub(crate) mod common;
pub mod prelude;
pub mod protocol;
pub mod recipient; // or client
pub mod sender; // or server

/// Init logger for **tests**
#[cfg(test)]
pub fn init_logger_for_test() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Warn)
        .filter_module("snwf", log::LevelFilter::Debug)
        .try_init();
}
