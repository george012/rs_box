use std::fs;
use lazy_static::lazy_static;
use log::{info, warn, error, debug, trace, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::sync::Mutex;
use std::path::Path;

use std::sync::Once;
static INIT: Once = Once::new();


lazy_static! {
    // static ref LOGGER: Mutex<LoggerManager> = Mutex::new(LoggerManager::new());
    static ref GLOBAL_LOGGER: Mutex<Option<LoggerManager>> = Mutex::new(None);
}

struct LoggerManager {
    enable_file_log: bool,
    log_level: log::LevelFilter,
    log_dir: Option<String>,
}

fn default_log_path() -> String {
    if cfg!(target_os = "linux") {
        "/var/log/my_app/run.log".to_string()
    } else {
        "./logs/run.log".to_string()
    }
}

impl LoggerManager {
    pub fn new(enable_file_log: bool, log_level: LevelFilter, log_dir: Option<String>) -> Self {
        let mut manager = Self {
            enable_file_log,
            log_level,
            log_dir,
        };
        manager.setup();  // Configure logging according to the settings
        manager
    }

    fn setup(&mut self) {
        let mut config_builder = Config::builder();
        let mut root_builder = Root::builder();

        if self.enable_file_log {
            // Configure logging to file
            let log_file_path = self.log_dir.as_ref()
                .map(|dir| {
                    if !dir.is_empty() {
                        format!("{}/run.log", dir)
                    } else {
                        default_log_path()
                    }
                })
                .unwrap_or_else(default_log_path);

            if let Some(parent) = Path::new(&log_file_path).parent() {
                fs::create_dir_all(parent).expect("Failed to create log directory");
            }

            let logfile = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n")))
                .build(&log_file_path)
                .expect("Failed to create file appender");

            config_builder = config_builder.appender(Appender::builder().build("logfile", Box::new(logfile)));
            root_builder = root_builder.appender("logfile");
        } else {
            // Configure logging to stdout only
            let stdout_appender = ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
                .build();

            config_builder = config_builder.appender(Appender::builder().build("stdout", Box::new(stdout_appender)));
            root_builder = root_builder.appender("stdout");
        }

        // Build and initialize the logging configuration outside the if/else block
        let root_config = root_builder.build(self.log_level);
        let config = config_builder.build(root_config)
            .expect("Failed to build logger configuration");

        // Initialize logger configuration
        if let Err(err) = log4rs::init_config(config) {
            eprintln!("Logger initialization failed: {}", err);
        }
    }
}

pub fn setup_log_tools(enable_file_log: bool, log_level: LevelFilter, log_dir: Option<String>) {
    let mut global_logger = GLOBAL_LOGGER.lock().unwrap();
    *global_logger = Some(LoggerManager::new(enable_file_log, log_level, log_dir));
}
pub fn log_info(message: &str) {
    log::info!("{}", message);
}

pub fn log_warning(message: &str) {
    log::warn!("{}", message);
}

pub fn log_error(message: &str) {
    log::error!("{}",message);
}

pub fn log_debug(message: &str) {
    log::debug!("{}", message);
}

/// 打印[trace]级别日志。
/// # 示例
/// ```
/// crate::rs_box::rs_box_log::rs_box_log::log_trace("trace log message");
/// ```
pub fn log_trace(message: &str) {
    trace!("{}", message);
}
