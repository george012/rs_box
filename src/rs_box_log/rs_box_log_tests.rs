use crate::rs_box_log::rs_box_log;

#[test]
fn test_logs_with_write_logfile() {

    rs_box_log::setup_log_tools("test_project",true,"",rs_box_log::LogLevel::LogLevelDebug,7,rs_box_log::LogFileSaveType::LogFileSaveTypeDays);
    rs_box_log::log_info("This is an info message");
    rs_box_log::log_error("This is an error message");
    rs_box_log::log_warning("This is an warning message");
    rs_box_log::log_debug("This is an debug message");
    rs_box_log::log_trace("This is an trace message");


   let a_sub_log_mg = rs_box_log::LoggerManager::new("sub_write_file");
    a_sub_log_mg.log_info_f("a sub write file info");
    a_sub_log_mg.log_warning_f("a sub write file warning");
    a_sub_log_mg.log_error_f("a sub write file error");
    a_sub_log_mg.log_debug_f("a sub write file debug");
    a_sub_log_mg.log_trace_f("a sub write file trace");

}

#[test]
fn test_logs_with_terminal_show() {
    rs_box_log::setup_log_tools("test_terminal_show",false,"",rs_box_log::LogLevel::LogLevelDebug,7,rs_box_log::LogFileSaveType::LogFileSaveTypeDays);
    rs_box_log::log_info("This is an info message");
    rs_box_log::log_error("This is an error message");
    rs_box_log::log_warning("This is an warning message");
    rs_box_log::log_debug("This is an debug message");
    rs_box_log::log_trace("This is an trace message");


    let a_sub_log_mg = rs_box_log::LoggerManager::new("sub_test_terminal_show");
    a_sub_log_mg.log_info_f("a sub write file info");
    a_sub_log_mg.log_warning_f("a sub write file warning");
    a_sub_log_mg.log_error_f("a sub write file error");
    a_sub_log_mg.log_debug_f("a sub write file debug");
    a_sub_log_mg.log_trace_f("a sub write file trace");
}

#[test]
fn test_simple_log_output() {
    rs_box_log::setup_log_tools("test_simple", false, "./logs", rs_box_log::LogLevel::LogLevelDebug, 7, rs_box_log::LogFileSaveType::LogFileSaveTypeDays);
    rs_box_log::log_debug("This is a debug message");
}