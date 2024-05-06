extern crate slog;

use std::env::var;
use slog::*;
use slog_async::Async;
use slog_term::TermDecorator;
use slog_json::Json;
use std::fs::OpenOptions;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_LOG_CONFIG: Mutex<Option<LogConfig>> = Mutex::new(None);
    static ref DEFAULT_LOGGER: Mutex<Option<LoggerManager>> = Mutex::new(None);
}

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    LogLevelDebug,
    LogLevelError,
    LogLevelWarning,
    LogLevelInfo,
    LogLevelTrace,
}

impl LogLevel {
    fn to_slog_level(self) -> Level {
        match self {
            LogLevel::LogLevelDebug => Level::Debug,
            LogLevel::LogLevelError => Level::Error,
            LogLevel::LogLevelWarning => Level::Warning,
            LogLevel::LogLevelInfo => Level::Info,
            LogLevel::LogLevelTrace => Level::Trace,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LogFileSaveType {
    LogFileSaveTypeDays,
    LogFileSaveTypeHours,
}
#[derive(Clone, Debug)]
pub struct LogConfig {
    model_name:             String,
    enable_save_log_file:   bool,
    log_dir:                String,
    log_level:              LogLevel,
    log_file_save_type:     LogFileSaveType,
    log_file_save_days_max: u64,
}

pub struct LoggerManager {
    log_cfg:    Option<LogConfig>,
    logger:     slog::Logger,

}

fn default_slog_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().stdout().build();
    let drain = slog_term::CompactFormat::new(decorator)
        .use_custom_timestamp(custom_timestamp)
        .build()
        .fuse();

    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn custom_timestamp(w: &mut dyn std::io::Write) -> std::io::Result<()> {
    write!(w, "{}", chrono::prelude::Utc::now().format("utc_%Y-%m-%d_%H:%M:%S"))
}
impl LoggerManager {

    pub fn new(model_name: &str) -> LoggerManager {
        let maybe_log_config = GLOBAL_LOG_CONFIG.lock().unwrap();

        let logger = if let Some(ref cfg) = *maybe_log_config {
            if cfg.enable_save_log_file == true{
                let file_path = format!("{}/{}/run.log", cfg.log_dir, model_name);
                // 尝试创建目录，如果目录不存在
                if let Some(parent) = std::path::Path::new(&file_path).parent() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|err| {
                        eprintln!("Failed to create log directory {}: {}", parent.display(), err);
                    });
                }

                let file = OpenOptions::new()
                    .read(true)
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(file_path)
                    .unwrap();

                let decorator = slog_term::PlainDecorator::new(file);

                let drain = slog_term::FullFormat::new(decorator)
                    .use_custom_timestamp(custom_timestamp)
                    .build()
                    .fuse();

                let drain_file = slog_async::Async::new(drain).build().fuse();

                slog::Logger::root(drain_file, o!())
            } else {
                default_slog_logger()
            }
        } else {
            eprintln!("Global log configuration is not initialized, falling back to console logging.");
            default_slog_logger()
        };

        LoggerManager {
            log_cfg: None, // 使用已有配置或 None
            logger
        }
    }

    pub fn log_info_f(&self, message: &str) {
        slog::info!(self.logger, "{}", message);
    }
    pub fn log_warning_f(&self, message: &str) {
        slog::warn!(self.logger, "{}", message);
    }
    pub fn log_error_f(&self, message: &str) {
        slog::error!(self.logger, "{}", message);
    }
    pub fn log_debug_f(&self, message: &str) {
        slog::debug!(self.logger, "{}", message);
    }
    pub fn log_trace_f(&self, message: &str) {
        slog::trace!(self.logger, "{}", message);
    }
}

pub fn setup_log_tools(product_name: &str, enable_save_log_file: bool, log_dir: &str, log_level: LogLevel, log_file_save_days_max: u64, log_file_save_type: LogFileSaveType) {
    let mut log_config = LogConfig {
        model_name: product_name.to_string(),
        enable_save_log_file,
        log_dir: log_dir.to_string(),
        log_level,
        log_file_save_type,
        log_file_save_days_max,
    };

    if log_dir == "" {
        if cfg!(target_os = "linux") {
            log_config.log_dir = format!("/var/log/{}",product_name);
        } else {
            log_config.log_dir = format!("./logs");
        }
    }

    {
        let mut config_guard = GLOBAL_LOG_CONFIG.lock().unwrap();
        *config_guard = Some(log_config);
    }

    let logger_manager = LoggerManager::new("main");
    let mut logger_guard = DEFAULT_LOGGER.lock().unwrap();
    *logger_guard = Some(logger_manager);
}

pub fn log_info(message: &str) {
    if let Some(logger) = DEFAULT_LOGGER.lock().unwrap().as_ref() {
        logger.log_info_f(message)
    }
}

pub fn log_warning(message: &str) {
    if let Some(logger) = DEFAULT_LOGGER.lock().unwrap().as_ref() {
        logger.log_warning_f(message)
    }
}

pub fn log_error(message: &str) {
    if let Some(logger) = DEFAULT_LOGGER.lock().unwrap().as_ref() {
        logger.log_error_f(message)
    }
}
pub fn log_debug(message: &str) {
    if let Some(logger) = DEFAULT_LOGGER.lock().unwrap().as_ref() {
        logger.log_debug_f(message)
    }
}

pub fn log_trace(message: &str) {
    if let Some(logger) = DEFAULT_LOGGER.lock().unwrap().as_ref() {
        logger.log_trace_f(message)
    }
}