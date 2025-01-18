use chrono::{DateTime, Local, Utc};
use env_logger;
use std::io::Write;

pub fn init() {
    // Inisialisasi logger dengan format yang menyertakan file dan baris
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record: &log::Record<'_>| {
            writeln!(
                buf,
                "{} {} [Caller::{}:{}] -> {}",
                Local::now(),
                record.level(),
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

#[macro_export]
macro_rules! debug_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}
