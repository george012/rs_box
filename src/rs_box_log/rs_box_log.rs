extern crate slog;

use std::fs;
use slog::{Level, o, Drain};
use slog_term;
use slog_async::Async;
use std::fs::{File, OpenOptions};
use std::sync::{Mutex};
use lazy_static::lazy_static;
use crate::rs_box_log::rs_box_log;
lazy_static! {
    static ref GLOBAL_LOG_CONFIG: Mutex<Option<LogConfig>> = Mutex::new(None);
    static ref DEFAULT_LOGGER: Mutex<Option<LoggerManager>> = Mutex::new(None);
}
static DEFAULT_CHAIN_SIZE: usize = 1024*2;

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

fn create_slog_logger_terminal() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().stdout().build();
    let drain = slog_term::CompactFormat::new(decorator)
        .use_custom_timestamp(custom_timestamp)
        .build()
        .fuse();

    let async_drain = slog_async::Async::new(drain)
        .chan_size(DEFAULT_CHAIN_SIZE)
        .build()
        .fuse();

    slog::Logger::root(async_drain, o!())
}

fn create_slog_logger_write_file(file: File) -> slog::Logger {
    let decorator = slog_term::PlainDecorator::new(file);

    let drain = slog_term::FullFormat::new(decorator)
        .use_custom_timestamp(custom_timestamp)
        .build()
        .fuse();

    let drain_file = Async::new(drain)
        .chan_size(DEFAULT_CHAIN_SIZE)
        .build()
        .fuse();

    slog::Logger::root(drain_file, o!())
}

fn custom_timestamp(w: &mut dyn std::io::Write) -> std::io::Result<()> {
    write!(w, "{}", chrono::prelude::Utc::now().format("UTC %Y-%m-%d_%H:%M:%S"))
}
impl LoggerManager {

    pub fn new(model_name: &str) -> LoggerManager {
        let maybe_log_config = GLOBAL_LOG_CONFIG.lock().unwrap();

        let mut a_log_cfg = LogConfig{
            model_name: model_name.to_string(),
            enable_save_log_file: false,
            log_dir: "".to_string(),
            log_level: LogLevel::LogLevelDebug,
            log_file_save_type: LogFileSaveType::LogFileSaveTypeDays,
            log_file_save_days_max: 0,
        };

        let logger = if let Some(ref cfg) = *maybe_log_config {
            a_log_cfg.log_level                 =cfg.log_level;
            a_log_cfg.enable_save_log_file      =cfg.enable_save_log_file;
            a_log_cfg.log_file_save_type        =cfg.log_file_save_type;
            a_log_cfg.log_file_save_days_max    =cfg.log_file_save_days_max;
            a_log_cfg.log_dir                   =format!("{}/{}", cfg.log_dir, model_name);

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
                create_slog_logger_write_file(file)
            } else {
                create_slog_logger_terminal()
            }
        } else {
            eprintln!("Global log configuration is not initialized, falling back to console logging.");
            create_slog_logger_terminal()
        };

        LoggerManager {
            log_cfg: Option::from(a_log_cfg), // 使用已有配置或 None
            logger
        }
    }
    fn log_format(&self, message: &str,a_log_level: LogLevel) {

        match a_log_level{
            LogLevel::LogLevelDebug => {
                slog::debug!(self.logger, "{}", message);
            }
            LogLevel::LogLevelError => {
                slog::error!(self.logger, "{}", message);
            }
            LogLevel::LogLevelWarning => {
                slog::warn!(self.logger, "{}", message);
            }
            LogLevel::LogLevelInfo => {
                slog::info!(self.logger, "{}", message);
            }
            LogLevel::LogLevelTrace => {
                slog::trace!(self.logger, "{}", message);
            }
        }
    }
    pub fn log_info_f(&self, message: &str) {
        self.log_format(message,self::LogLevel::LogLevelInfo);
    }
    pub fn log_warning_f(&self, message: &str) {
        self.log_format(message,self::LogLevel::LogLevelWarning);
    }
    pub fn log_error_f(&self, message: &str) {
        self.log_format(message,self::LogLevel::LogLevelError);
    }
    pub fn log_debug_f(&self, message: &str) {
        self.log_format(message,self::LogLevel::LogLevelDebug);
    }
    pub fn log_trace_f(&self, message: &str) {
        self.log_format(message,self::LogLevel::LogLevelTrace);
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