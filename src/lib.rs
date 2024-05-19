#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::arc_with_non_send_sync)]

mod lib_tests;
pub mod rs_box_log;
pub mod block_chain;

use std::fs;
use toml::Value;

const LIB_VERSION: &str = "0.0.25";

#[derive(Clone, Copy, Debug,PartialEq)]
pub enum RunMode {
    RunModeUnknown,
    RunModeDebug,
    RunModeRelease,
    RunModeTest,
}
impl RunMode {
    pub fn to_string(&self) -> &'static str {
        match *self {
            RunMode::RunModeDebug => "Debug",
            RunMode::RunModeRelease => "Release",
            RunMode::RunModeTest => "Test",
            RunMode::RunModeUnknown => "Unknown",
        }
    }
}

static mut CURRENT_RUN_MODE: RunMode = RunMode::RunModeUnknown;

pub fn get_current_run_mode() -> RunMode {
    unsafe { CURRENT_RUN_MODE }
}

pub fn rs_box_setup(project_name: &str, run_mode: RunMode, product_log_dir: &str, log_max_save_days: u64, http_request_timeout: u64) {

    let enable_save_log_file = match run_mode {
        RunMode::RunModeRelease | RunMode::RunModeTest => true,
        _ => false,
    };

    let log_level = match run_mode {
        RunMode::RunModeDebug | RunMode::RunModeTest => crate::rs_box_log::rs_box_log::LogLevel::LogLevelDebug,
        RunMode::RunModeRelease => crate::rs_box_log::rs_box_log::LogLevel::LogLevelInfo,
        _ => crate::rs_box_log::rs_box_log::LogLevel::LogLevelDebug,
    };

    crate::rs_box_log::rs_box_log::setup_log_tools("test_project",enable_save_log_file,product_log_dir,rs_box_log::rs_box_log::LogLevel::LogLevelDebug,7);

    unsafe {
        CURRENT_RUN_MODE = run_mode;
    }

    println!("Rust Developer Tool Box Setup End");
    println!("project    name  =[{}]", project_name);
    println!("rs_box  version  =[v{}]", LIB_VERSION);
    println!("Run        Mode  =[{}]", run_mode.to_string());
    println!("Log       Level  =[{}]", log_level.to_str());
    println!("default log dir  =[{}]", product_log_dir);
    println!("log save days max=[{}]", log_max_save_days);
    println!("http req timeout =[{} Second]", http_request_timeout);
}





