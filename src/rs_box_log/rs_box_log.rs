use std::fs;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use slog::Drain;

lazy_static! {
    static ref GLOBAL_LOG_CONFIG: Mutex<LogConfig> = Mutex::new(LogConfig::default());
    static ref DEFAULT_LOGGER: Mutex<LoggerManager> = Mutex::new(LoggerManager::new("main"));
}
static DEFAULT_CHAIN_SIZE: usize = 1024;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LogLevel {
    LogLevelDebug,
    LogLevelError,
    LogLevelWarning,
    LogLevelInfo,
    LogLevelTrace,
}

impl LogLevel {
    fn to_slog_level(self) -> slog::Level {
        match self {
            LogLevel::LogLevelDebug =>  slog::Level::Debug,
            LogLevel::LogLevelError =>  slog::Level::Error,
            LogLevel::LogLevelWarning =>  slog::Level::Warning,
            LogLevel::LogLevelInfo =>  slog::Level::Info,
            LogLevel::LogLevelTrace =>  slog::Level::Trace,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LogFileSaveType {
    LogFileSaveTypeDays,
    LogFileSaveTypeHours,
}
#[derive(Clone, Debug)]
pub struct LogConfig {
    project_name:           String,
    enable_save_log_file:   bool,
    log_dir:                String,
    log_level:              LogLevel,
    log_file_save_type:     LogFileSaveType,
    log_file_save_days_max: u64,
}

impl Default for LogConfig {
    fn default() -> Self {
        let mut project_name: &str = "default";
        let mut log_dir: String = String::new();

        if cfg!(target_os = "linux") {
            log_dir = format!("/var/log/{}",project_name);
        } else {
            log_dir = format!("./logs");
        }

        LogConfig {
            project_name: project_name.to_string(),
            enable_save_log_file: false,
            log_dir: log_dir.to_string(),
            log_level: LogLevel::LogLevelInfo,
            log_file_save_type: LogFileSaveType::LogFileSaveTypeDays,
            log_file_save_days_max: 7,
        }
    }
}


pub struct LoggerManager {
    logger:     slog::Logger,
}


fn create_slog_logger_terminal() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().stdout().build();
    let drain = slog_term::CompactFormat::new(decorator)
        .use_custom_timestamp(custom_timestamp)
        .build()
        .fuse();

    let sync_drain = slog_async::Async::new(drain)
        .build()
        .fuse();

    slog::Logger::root(sync_drain,  slog::o!())
}

fn create_slog_logger_write_file(file: std::fs::File) -> slog::Logger {
    let decorator = slog_term::PlainDecorator::new(file);

    let drain = slog_term::FullFormat::new(decorator)
        .use_custom_timestamp(custom_timestamp)
        .build()
        .fuse();

    let drain_file =  slog_async::Async::new(drain)
        .build()
        .fuse();

    slog::Logger::root(drain_file, slog::o!())
}

fn custom_timestamp(w: &mut dyn std::io::Write) -> std::io::Result<()> {
    write!(w, "{}", chrono::prelude::Utc::now().format("UTC %Y-%m-%d_%H:%M:%S"))
}
impl LoggerManager {

    // Use a shared global configuration
    pub fn new(model_name: &str) -> Self {
        Self::new_with_config(GLOBAL_LOG_CONFIG.lock().unwrap().clone(),model_name)
    }

    // Allow independent configuration
    fn new_with_config(config: LogConfig,model_name: &str) -> Self {
        let logger = if config.enable_save_log_file {
            let file_path = format!("{}/{}/run.log", config.log_dir, model_name);

            // 确保日志目录存在
            if let Some(parent) = std::path::Path::new(&file_path).parent() {
                if std::fs::create_dir_all(parent).is_err() {
                    eprintln!("Failed to create log directory: {}", parent.display());
                }
            }

            // 尝试打开文件
            match std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&file_path) {
                Ok(file) => create_slog_logger_write_file(file),
                Err(e) => {
                    eprintln!("Failed to open log file '{}': {}", file_path, e);
                    create_slog_logger_terminal() // Fallback to terminal logging
                }
            }
        } else {
            create_slog_logger_terminal()
        };

        LoggerManager {
            logger,
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
    let a_log_dir = if log_dir.is_empty() {
        if cfg!(target_os = "linux") {
            format!("/var/log/{}", product_name)
        } else {
            "./logs".to_string()
        }
    } else {
        log_dir.to_string()
    };

    // 更新 GLOBAL_LOG_CONFIG 中的配置
    let mut config_global = GLOBAL_LOG_CONFIG.lock().unwrap();
    config_global.project_name = product_name.to_string();
    config_global.log_dir = a_log_dir.to_string();
    config_global.enable_save_log_file = enable_save_log_file;
    config_global.log_level = log_level;
    config_global.log_file_save_type = log_file_save_type;
    config_global.log_file_save_days_max = log_file_save_days_max;
}

pub fn log_info(message: &str) {
    if let logger = DEFAULT_LOGGER.lock().unwrap() {
        logger.log_info_f(message)
    }
}

pub fn log_warning(message: &str) {
    if let logger = DEFAULT_LOGGER.lock().unwrap() {
        logger.log_warning_f(message)
    }
}

pub fn log_error(message: &str) {
    if let logger = DEFAULT_LOGGER.lock().unwrap() {
        logger.log_error_f(message)
    }
}
pub fn log_debug(message: &str) {
    if let logger = DEFAULT_LOGGER.lock().unwrap() {
        logger.log_debug_f(message)
    }
}

pub fn log_trace(message: &str) {
    if let logger = DEFAULT_LOGGER.lock().unwrap() {
        logger.log_trace_f(message)
    }
}