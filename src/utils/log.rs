use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum LogLevel {
    Info,
    Error,
    Success,
}

pub fn log(level: LogLevel, msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    write!(&mut stdout, "[").unwrap();

    let (symbol, color) = match level {
        LogLevel::Info => ("*", Color::Cyan),
        LogLevel::Error => ("!", Color::Red),
        LogLevel::Success => ("+", Color::Green),
    };

    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();
    write!(&mut stdout, "{}", symbol).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();

    writeln!(&mut stdout, "] {}", msg).unwrap();
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::utils::log::log($crate::utils::log::LogLevel::Info, $msg);
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::utils::log::log($crate::utils::log::LogLevel::Error, $msg);
    };
}

#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        $crate::utils::log::log($crate::utils::log::LogLevel::Success, $msg);
    };
}
