use std::fs::File;

use time::{macros::format_description, UtcOffset};
use tracing_subscriber::{fmt::{self, format::FmtSpan, time::OffsetTime}, layer::SubscriberExt, Registry};

pub fn logger_init() {
    // 定义日志时间格式、时区.
    let local_offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    let timer = OffsetTime::new(
        local_offset,
        format_description!("[year]/[month]/[day] [hour]:[minute]:[second]"),
    );

    // 日志文件 appender
    let file_appender = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_timer(timer.clone())
        .with_ansi(false)
        .with_span_events(FmtSpan::FULL)
        .with_target(false)
        .with_writer(File::create("tiny.log").unwrap());

    // 标准输出 appender
    let stdout_appender = fmt::layer()
        // 显示日志级别
        .with_level(true)
        // 显示行号
        .with_line_number(true)
        // 显示文件名 
        .with_file(true) 
        // 显示线程ID
        .with_thread_ids(false)
        // 显示目标 
        .with_target(false) 
        // 日志时间格式
        .with_timer(timer);
    
    let subscriber = Registry::default()
        .with(file_appender)
        .with(stdout_appender);
    let _ = tracing::subscriber::set_global_default(subscriber);
    // tracing_subscriber::fmt()
    //     // 显示日志级别
    //     .with_level(true)
    //     // 日志最大显示级别 
    //     .with_max_level(Level::DEBUG)
    //     // 设置日志输出文件
    //     // 显示行号
    //     .with_line_number(true)
    //     // 显示文件名 
    //     .with_file(false) 
    //     // 显示线程ID
    //     .with_thread_ids(false)
    //     // 显示目标 
    //     .with_target(false) 
    //     // 日志时间格式
    //     .with_timer(timer).init();
}
