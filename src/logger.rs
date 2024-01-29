use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::SystemTime;

// Access log
const ACCESS_LOG: &str = "access.log";
// Error log
const ERROR_LOG: &str = "access.log";

fn default_property_logging_accesslog() -> String {
    ACCESS_LOG.to_string()
}
fn default_property_logging_errorlog() -> String {
    ERROR_LOG.to_string()
}

pub fn default_property_logging() -> LoggingProperties {
    LoggingProperties {
        accesslog: default_property_logging_accesslog(),
        errorlog: default_property_logging_errorlog(),
    }
}

pub struct Logger {
    properties: LoggingProperties,
}

impl Logger {
    pub fn new(properties: LoggingProperties) -> Logger {
        Logger {
            properties: properties,
        }
    }

    pub fn access(&self, msg: &str) {
        self.write(msg, LogType::Access);
    }

    pub fn error(&self, msg: &str) {
        self.write(msg, LogType::Error);
    }

    pub fn write(&self, msg: &str, log_type: LogType) {
        let log_file = match log_type {
            LogType::Access => &self.properties.accesslog,
            LogType::Error => &self.properties.errorlog,
        };

        // Append the log message to the appropriate log file
        if let Err(e) = self.append_to_log(log_file, msg) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }

    fn append_to_log(&self, log_file: &str, msg: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        writeln!(file, "[{}] {}", timestamp, msg)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggingProperties {
    #[serde(default = "default_property_logging_accesslog")]
    accesslog: String,
    #[serde(default = "default_property_logging_errorlog")]
    errorlog: String,
}

#[derive(Debug)]
enum LogType {
    Access,
    Error,
}
