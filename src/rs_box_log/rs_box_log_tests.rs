use log::{LevelFilter};
use crate::rs_box_log::rs_box_log;

#[test]
fn test_logs_with_write_logfile() {
    rs_box_log::setup_log_tools(true, LevelFilter::Trace, Some("".to_string()));
    rs_box_log::log_info("This is an info message");
    rs_box_log::log_error("This is an error message");
    rs_box_log::log_warning("This is an warning message");
    rs_box_log::log_debug("This is an debug message");
    rs_box_log::log_trace("This is an trace message");
}

#[test]
fn test_logs_with_terminal_show() {
    rs_box_log::setup_log_tools(false, LevelFilter::Trace, Some("".to_string()));
    rs_box_log::log_info("This is an info message");
    rs_box_log::log_error("This is an error message");
    rs_box_log::log_warning("This is an warning message");
    rs_box_log::log_debug("This is an debug message");
    rs_box_log::log_trace("This is an trace message");
}