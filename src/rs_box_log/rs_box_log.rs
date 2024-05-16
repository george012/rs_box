use chrono::Local;
use std::fs::{File, OpenOptions, create_dir_all, rename, remove_file};
use std::io::Write;
use std::sync::{Mutex, Arc};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use pathdiff::diff_paths;
use backtrace;

#[derive(Clone, Copy, Debug)]
pub enum LogFileSaveType {
    LogFileSaveTypeDays,
    LogFileSaveTypeHours,
}

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    LogLevelInfo,
    LogLevelWarning,
    LogLevelError,
    LogLevelDebug,
    LogLevelTrace,
}

impl LogLevel {
    fn to_str(&self) -> &'static str {
        match self {
            LogLevel::LogLevelInfo => "INFO",
            LogLevel::LogLevelWarning => "WARNING",
            LogLevel::LogLevelError => "ERROR",
            LogLevel::LogLevelDebug => "DEBUG",
            LogLevel::LogLevelTrace => "TRACE",
        }
    }

    fn to_level_filter(self) -> log::LevelFilter {
        match self {
            LogLevel::LogLevelDebug => log::LevelFilter::Debug,
            LogLevel::LogLevelError => log::LevelFilter::Error,
            LogLevel::LogLevelWarning => log::LevelFilter::Warn,
            LogLevel::LogLevelInfo => log::LevelFilter::Info,
            LogLevel::LogLevelTrace => log::LevelFilter::Trace,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogConfig {
    project_name: String,
    enable_save_log_file: bool,
    log_dir: String,
    log_level: LogLevel,
    file_save_days_max: u64,
    file_save_type: LogFileSaveType,
}

impl Default for LogConfig {
    fn default() -> Self {
        let project_name = "default";
        let log_dir = if cfg!(target_os = "linux") {
            format!("/var/log/{}", project_name)
        } else {
            "./logs".to_string()
        };

        LogConfig {
            project_name: project_name.to_string(),
            enable_save_log_file: false,
            log_dir,
            log_level: LogLevel::LogLevelTrace,
            file_save_days_max: 7,
            file_save_type: LogFileSaveType::LogFileSaveTypeDays,
        }
    }
}

pub struct LoggerManager {
    config: Arc<LogConfig>,
    file: Option<Mutex<File>>,
    current_log_path: Mutex<PathBuf>,
}

impl LoggerManager {
    pub fn default() -> Self {
        let config = GLOBAL_LOG_CONFIG.lock().unwrap().clone();
        LoggerManager::new_with_config(config)
    }

    pub fn new(module_name: &str) -> Self {
        let global_config = GLOBAL_LOG_CONFIG.lock().unwrap().clone();
        let mut new_config = (*global_config).clone();
        new_config.project_name = module_name.to_string();
        LoggerManager::new_with_config(Arc::new(new_config))
    }

    fn new_with_config(config: Arc<LogConfig>) -> Self {
        let file_path = LoggerManager::get_log_file_path(&config);
        let log_dir = Path::new(&file_path).parent().unwrap();
        create_dir_all(log_dir).expect("Failed to create log directory");

        let file = if config.enable_save_log_file {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&file_path)
                .expect("Failed to open log file");
            LoggerManager::create_symlink(&file_path, &config.log_dir);
            Some(Mutex::new(file))
        } else {
            None
        };

        LoggerManager {
            config,
            file,
            current_log_path: Mutex::new(PathBuf::from(file_path)),
        }
    }

    fn get_log_file_path(config: &LogConfig) -> String {
        let now = Local::now();
        let date_folder = now.format("%Y-%m-%d").to_string();
        let hour = now.format("%H").to_string();
        format!("{}/{}/{}_{}.log", config.log_dir, date_folder, date_folder, hour)
    }

    fn create_symlink(target: &str, log_dir: &str) {
        let link_path = format!("{}/run.log", log_dir);
        let target_path = Path::new(target);
        let relative_target = diff_paths(target_path, Path::new(log_dir)).unwrap();

        if let Ok(existing_target) = std::fs::read_link(&link_path) {
            if existing_target == relative_target {
                return; // 当前符号链接已指向目标路径，无需重新创建
            }
        }

        let _ = remove_file(&link_path);
        if let Err(e) = std::os::unix::fs::symlink(relative_target, link_path) {
            eprintln!("Failed to create symlink: {}", e);
        }
    }

    fn rotate_files(&self) {
        let now = Local::now();
        let date_folder = now.format("%Y-%m-%d").to_string();
        let hour = now.format("%H").to_string();
        let log_dir = Path::new(&self.config.log_dir).join(&date_folder);

        // Rotate the log files for the current day
        for i in (0..self.config.file_save_days_max).rev() {
            let src = log_dir.join(format!("{}_{}.log", date_folder, i));
            let dst = log_dir.join(format!("{}_{}.log", date_folder, i + 1));
            if src.exists() {
                let _ = rename(src, dst);
            }
        }

        let current_log_path = format!("{}/{}_{}.log", log_dir.display(), date_folder, hour);
        let mut log_path = self.current_log_path.lock().unwrap();
        *log_path = PathBuf::from(&current_log_path);

        // Create a new log file
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&current_log_path)
            .expect("Failed to open log file");

        if let Some(ref file_lock) = self.file {
            let mut file_guard = file_lock.lock().unwrap();
            *file_guard = file;
        }

        LoggerManager::create_symlink(&current_log_path, &self.config.log_dir);
    }

    fn should_rotate(&self) -> bool {
        let now = Local::now();
        let log_path = self.current_log_path.lock().unwrap();
        let log_dir = Path::new(&self.config.log_dir).join(now.format("%Y-%m-%d").to_string());

        if log_path.starts_with(&log_dir) {
            false
        } else {
            true
        }
    }


    fn get_caller_info() -> String {
        let backtrace = backtrace::Backtrace::new();
        for frame in backtrace.frames().iter().skip(1) { // 跳过第一个堆栈帧
            for symbol in frame.symbols() {
                if let Some(name) = symbol.name() {
                    let name_str = name.to_string();
                    if !name_str.contains("log_format") && !name_str.contains("LoggerManager") {
                        return format!(
                            "{}:{}",
                            symbol.filename().unwrap_or_else(|| std::path::Path::new("unknown")).display(),
                            symbol.lineno().unwrap_or(0)
                        );
                    }
                }
            }
        }
        "unknown".to_string()
    }

    fn log_format(&self, level: LogLevel, message: &str) {
        if level as u8 <= self.config.log_level as u8 {
            let now = chrono::Utc::now();
            let color_code = match level {
                LogLevel::LogLevelInfo => "\x1b[32m",    // 绿色
                LogLevel::LogLevelWarning => "\x1b[33m", // 黄色
                LogLevel::LogLevelError => "\x1b[31m",   // 红色
                LogLevel::LogLevelDebug => "\x1b[36m",   // 青色
                LogLevel::LogLevelTrace => "\x1b[34m",   // 蓝色
            };
            let reset_code = "\x1b[0m"; // 重置颜色

            let location_info = LoggerManager::get_caller_info();

            let log_message = format!(
                "[{}] {}[{}]{} [--{}] [location: {}]\n",
                now.format("%Y-%m-%d %H:%M:%S %:z"),
                color_code,
                level.to_str(),
                reset_code,
                message,
                location_info,
            );

            if let Some(ref file) = self.file {
                if self.should_rotate() {
                    self.rotate_files();
                }
                let mut file = file.lock().unwrap();
                file.write_all(log_message.as_bytes()).unwrap();
            } else {
                print!("{}", log_message);
            }
        }
    }

    pub fn log_info_f(&self, message: &str) {
        self.log_format(LogLevel::LogLevelInfo, message);
    }

    pub fn log_warning_f(&self, message: &str) {
        self.log_format(LogLevel::LogLevelWarning, message);
    }

    pub fn log_error_f(&self, message: &str) {
        self.log_format(LogLevel::LogLevelError, message);
    }

    pub fn log_debug_f(&self, message: &str) {
        self.log_format(LogLevel::LogLevelDebug, message);
    }

    pub fn log_trace_f(&self, message: &str) {
        self.log_format(LogLevel::LogLevelTrace, message);
    }
}

static GLOBAL_LOG_CONFIG: Lazy<Mutex<Arc<LogConfig>>> = Lazy::new(|| Mutex::new(Arc::new(LogConfig::default())));
static DEFAULT_LOGGER: Lazy<Mutex<LoggerManager>> = Lazy::new(|| Mutex::new(LoggerManager::default()));

pub fn setup_log_tools(project_name: &str, enable_save_log_file: bool, log_dir: &str, log_level: LogLevel, file_save_days_max: u64, file_save_type: LogFileSaveType, ) {
    let log_dir = if log_dir.is_empty() {
        if cfg!(target_os = "linux") {
            format!("/var/log/{}", project_name)
        } else {
            "./logs".to_string()
        }
    } else {
        log_dir.to_string()
    };

    let new_config = Arc::new(LogConfig {
        project_name: project_name.to_string(),
        enable_save_log_file,
        log_dir,
        log_level,
        file_save_days_max,
        file_save_type,
    });

    {
        let mut config = GLOBAL_LOG_CONFIG.lock().unwrap();
        *config = new_config.clone();
    }

    {
        let mut logger = DEFAULT_LOGGER.lock().unwrap();
        *logger = LoggerManager::new_with_config(new_config);
    }
}

fn with_default_logger<F>(log_function: F)
    where
        F: FnOnce(&LoggerManager),
{
    let logger = DEFAULT_LOGGER.lock().unwrap();
    log_function(&*logger);
}

pub fn log_info(message: &str) {
    with_default_logger(|logger| logger.log_info_f(message));
}

pub fn log_warning(message: &str) {
    with_default_logger(|logger| logger.log_warning_f(message));
}

pub fn log_error(message: &str) {
    with_default_logger(|logger| logger.log_error_f(message));
}

pub fn log_debug(message: &str) {
    with_default_logger(|logger| logger.log_debug_f(message));
}

pub fn log_trace(message: &str) {
    with_default_logger(|logger| logger.log_trace_f(message));
}
