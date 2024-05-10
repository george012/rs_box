use super::rs_box_log;
#[test]
fn test_logs_with_write_logfile() {

    rs_box_log::setup_log_tools("test_project",true,"",rs_box_log::LogLevel::LogLevelDebug,7,rs_box_log::LogFileSaveType::LogFileSaveTypeDays);
    rs_box_log::log_info("This is an info message");
    rs_box_log::log_error("This is an error message");
    rs_box_log::log_warning("This is an warning message");
    rs_box_log::log_debug("This is an debug message");
    rs_box_log::log_trace("This is an trace message");

    // 创建 100 个线程
    let mut handles = vec![];
    for i in 0..100 {
        let handle = std::thread::spawn(move || {
            rs_box_log::log_info(format!("main log test at thread {}", i).as_str(), );

            let a_sub_log_mg = rs_box_log::LoggerManager::new(format!("test_thread_{}",i).as_str());
            a_sub_log_mg.log_info_f(format!("a sub write file warning {}",i).as_str());
            a_sub_log_mg.log_warning_f(format!("a sub write file warning {}",i).as_str());
            a_sub_log_mg.log_error_f(format!("a sub write file error {}",i).as_str());
            a_sub_log_mg.log_debug_f(format!("a sub write file debug {}",i).as_str());
            a_sub_log_mg.log_trace_f(format!("a sub write file trace {}",i).as_str());
        });
        handles.push(handle);
    }
    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_logs_with_terminal_show() {
    rs_box_log::setup_log_tools("test_terminal_show",false,"",rs_box_log::LogLevel::LogLevelDebug,7,rs_box_log::LogFileSaveType::LogFileSaveTypeDays);
    rs_box_log::log_info("This is an info message", );
    rs_box_log::log_error("This is an error message");
    rs_box_log::log_warning("This is an warning message");
    rs_box_log::log_debug("This is an debug message");
    rs_box_log::log_trace("This is an trace message");


    // 创建 100 个线程
    let mut handles = vec![];
    for i in 0..100 {
        let handle = std::thread::spawn(move || {
            rs_box_log::log_info(format!("main-show log test at thread {}", i).as_str(), );

            let a_sub_log_mg = rs_box_log::LoggerManager::new(format!("sub_test_terminal_show_{}",i).as_str());
            a_sub_log_mg.log_info_f("a sub write file info");
            a_sub_log_mg.log_warning_f("a sub write file warning");
            a_sub_log_mg.log_error_f("a sub write file error");
            a_sub_log_mg.log_debug_f("a sub write file debug");
            a_sub_log_mg.log_trace_f("a sub write file trace");
        });
        handles.push(handle);
    }
    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_simple_log_output() {
    rs_box_log::setup_log_tools("test_simple", false, "./logs", rs_box_log::LogLevel::LogLevelDebug, 7, rs_box_log::LogFileSaveType::LogFileSaveTypeDays);
    rs_box_log::log_debug("This is a debug message");
}