/* Logger module from Shared
  [X] done
  [X] refactor
*/
use std::fmt;

use colored::Colorize;

#[allow(dead_code)]
pub enum Icons {
    Penguin,
    Medal,
    Error,
    Rocket,
    Download,
}

#[allow(dead_code)]
impl fmt::Display for Icons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Icons::Penguin => {
                write!(f, "\u{1F427}")
            }
            Icons::Medal => {
                write!(f, "\u{1F3C5}")
            }
            Icons::Rocket => {
                write!(f, "\u{1F680}")
            }
            Icons::Download => {
                write!(f, "\u{2B73}")
            }
            Icons::Error => {
                write!(f, "\u{274C}")
            }
        }
    }
}

#[allow(dead_code)]
pub enum Level {
    Error,
    Warn,
    Info,
    Trace,
    Debug,
}

#[allow(dead_code)]
impl Level {
    pub fn to_log_str(&self) -> String {
        match self {
            Level::Error => " [ERROR] ".red().to_string(),
            Level::Warn => " [WARN] ".yellow().to_string(),
            Level::Info => " [INFO] ".green().to_string(),
            Level::Trace => " [TRACE] ".white().to_string(),
            Level::Debug => " [DEBUG] ".blue().to_string(),
        }
    }
}

#[macro_export]
macro_rules! logger_summary {
    ($summary:expr) => {
        println!("\n {}", $summary.to_uppercase().bold().purple())
    };
}

#[macro_export]
macro_rules! logger_cmd {
    ($summary:expr, $cmd:expr, $descr:expr) => {
        println!("  {} {} {}", $summary.bold(), $cmd, $descr.purple())
    };
}

#[macro_export]
macro_rules! logger {
    ($color:expr, $log:expr) => {
        println!(
            "{}{}{}",
            chrono::prelude::Local::now().format("%H:%M"),
            $color.to_log_str(),
            $log
        )
    };
}

#[macro_export]
macro_rules! logger_warn {
    ($log:expr) => {
        logger!(Level::Warn, $log)
    };
    ($icon:expr, $log:expr) => {
        logger!(Level::Warn, $log)
    };
}

#[macro_export]
macro_rules! logger_info {
    ($log:expr) => {
        logger!(Level::Info, $log)
    };
    ($icon:expr, $log:expr) => {
        logger!(Level::Info, $log)
    };
}

#[macro_export]
macro_rules! logger_error {
    ($log:expr) => {
        logger!(Level::Error, $log)
    };
    ($icon:expr, $log:expr) => {
        logger!(Level::Error, $log)
    };
}

#[macro_export]
macro_rules! logger_trace {
    ($log:expr) => {
        logger!(Level::Trace, $log)
    };
    ($icon:expr, $log:expr) => {
        logger!(Level::Trace, $log)
    };
}

#[macro_export]
macro_rules! logger_debug {
    ($log:expr) => {
        logger!(Level::Debug, $log)
    };
    ($icon:expr, $log:expr) => {
        logger!(Level::Debug, $log)
    };
}
