use chrono::Utc;
use std::fmt::Display;

// === LOG LEVEL ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
  Error,
  Debug,
  Info,
  Warn,
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Error => write!(f, "ERROR"),
      Self::Debug => write!(f, "DEBUG"),
      Self::Info => write!(f, "INFO"),
      Self::Warn => write!(f, "WARN"),
    }
  }
}

// === LOG & MACROS ===

pub fn log(level: LogLevel, msg: &str) {
  println!("{} {} {}", Utc::now().format("%Y-%m-%dT%H:%M:%SZ"), level, msg);
}

#[macro_export]
macro_rules! log_error { ($($arg:tt)*) => { $crate::log::log($crate::log::LogLevel::Error, &format!($($arg)*)) }; }
#[macro_export]
macro_rules! log_debug { ($($arg:tt)*) => { $crate::log::log($crate::log::LogLevel::Debug, &format!($($arg)*)) }; }
#[macro_export]
macro_rules! log_info  { ($($arg:tt)*) => { $crate::log::log($crate::log::LogLevel::Info,  &format!($($arg)*)) }; }
#[macro_export]
macro_rules! log_warn  { ($($arg:tt)*) => { $crate::log::log($crate::log::LogLevel::Warn,  &format!($($arg)*)) }; }
