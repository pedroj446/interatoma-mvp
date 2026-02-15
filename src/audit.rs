use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "error" => Some(Self::Error),
            "warn" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" => Some(Self::Debug),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
        }
    }
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if level <= self.level {
            eprintln!("[{}] {}", level.as_str(), message);
        }
    }
}

#[derive(Serialize)]
struct AuditEvent<'a, T: Serialize> {
    ts_unix_ms: u128,
    run_id: &'a str,
    event: &'a str,
    payload: T,
}

pub struct AuditWriter {
    run_id: String,
    writer: Option<BufWriter<std::fs::File>>,
}

impl AuditWriter {
    pub fn disabled(run_id: String) -> Self {
        Self {
            run_id,
            writer: None,
        }
    }

    pub fn enabled(path: impl AsRef<Path>, run_id: String) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            run_id,
            writer: Some(BufWriter::new(file)),
        })
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn emit<T: Serialize>(&mut self, event: &str, payload: T) -> std::io::Result<()> {
        if let Some(writer) = self.writer.as_mut() {
            let row = AuditEvent {
                ts_unix_ms: unix_ms_now(),
                run_id: &self.run_id,
                event,
                payload,
            };
            serde_json::to_writer(&mut *writer, &row)?;
            writer.write_all(b"\n")?;
            writer.flush()?;
        }
        Ok(())
    }
}

pub fn generate_run_id() -> String {
    format!("run-{}", unix_ms_now())
}

fn unix_ms_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default()
}
