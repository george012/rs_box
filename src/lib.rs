#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::arc_with_non_send_sync)]

mod lib_tests;
pub mod rs_box_log;

use std::cmp::PartialEq;
use std::fs;
use toml::Value;

#[derive(Clone, Copy, Debug)]
pub enum RunMode {
    Unknown,
    Debug,
    Release,
    Test,
}

impl RunMode {
    pub fn to_string(&self) -> &'static str {
        match *self {
            RunMode::Debug => "Debug",
            RunMode::Release => "Release",
            RunMode::Test => "Test",
            RunMode::Unknown => "Unknown",
        }
    }
}

static mut CURRENT_RUN_MODE: RunMode = RunMode::Unknown;

pub fn get_current_run_mode() -> RunMode {
    unsafe { CURRENT_RUN_MODE }
}

impl PartialEq for RunMode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

pub fn rs_box_setup(project_name: &str, run_mode: RunMode, product_log_dir: Option<String>, log_max_save_days: i64, http_request_timeout: u32) {
    let log_dir = product_log_dir.unwrap_or_else(|| format!("/usr/logs/{}", project_name));
    let enable_save_log_file = run_mode != RunMode::Debug;
    let log_level = match run_mode {
        RunMode::Debug | RunMode::Test => crate::rs_box_log::rs_box_log::LogLevel::LogLevelDebug,
        RunMode::Release => crate::rs_box_log::rs_box_log::LogLevel::LogLevelInfo,
        _ => crate::rs_box_log::rs_box_log::LogLevel::LogLevelDebug,
    };

    crate::rs_box_log::rs_box_log::setup_log_tools("test_project",enable_save_log_file,"",rs_box_log::rs_box_log::LogLevel::LogLevelDebug,7,rs_box_log::rs_box_log::LogFileSaveType::LogFileSaveTypeDays);

    unsafe {
        CURRENT_RUN_MODE = run_mode;
    }

    println!("GTBox Tools Setup End");
    println!("ProjectName=[{}]", project_name);
    println!("RunMode=[{}]", run_mode.to_string());
    println!("LogLevel=[{:?}]", log_level);
    println!("Product main logdir=[{}]", log_dir);
    println!("LogSaveDays=[{}]", log_max_save_days);
    println!("HttpRequestTimeout=[{} Second]", http_request_timeout);
}

pub fn get_version() -> Result<String, Box<dyn std::error::Error>>{
    // 读取当前目录下的 Cargo.toml 文件
    let contents = fs::read_to_string("Cargo.toml")?;

    // 解析 TOML 内容
    let parsed = contents.parse::<Value>()?;

    // 尝试从解析后的 TOML 数据中获取版本号
    let version = parsed
        .get("package")
        .and_then(|pkg| pkg.get("version"))
        .and_then(|v| v.as_str())
        .ok_or("Version not found or is not a string")?
        .to_string();

    Ok(version)
}




