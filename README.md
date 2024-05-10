# rs_box
```ignore
cargo fix --lib -p rs_box --allow-dirty

```


## 线程 用法
```
#[test]
fn test_logs_with_write_logfile() {
    // 假设已经在某处设置了全局配置
    setup_log_tools("test_project", true, "./logs", LogLevel::LogLevelDebug, 7, LogFileSaveType::LogFileSaveTypeDays);

    let num_threads = 100;
    let handles: Vec<_> = (0..num_threads).map(|i| {
        std::thread::spawn(move || {
            // 每个线程创建自己的日志管理器
            let thread_logger = LoggerManager::new(&format!("thread_{}", i));
            thread_logger.log_info_f(&format!("This is an info message from thread {}", i));
            // 其他日志调用...
        })
    }).collect();

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}

```
