use time::{macros::format_description, UtcOffset};
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan, time::OffsetTime},
    layer::SubscriberExt,
    Layer, Registry,
};

const LOG_FILE_NAME: &str = "tiny";
const LOG_DIR: &str = "logs";

pub fn logger_init() {
    // 定义日志时间格式、时区.
    let local_offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    let timer = OffsetTime::new(
        local_offset,
        format_description!("[year]/[month]/[day] [hour]:[minute]:[second]"),
    );

    // 文件日志
    let log_file_layout = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(LOG_FILE_NAME)
        .filename_suffix("log")
        .build(LOG_DIR)
        .unwrap();
    let file_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_timer(timer.clone())
        .with_ansi(false)
        .with_span_events(FmtSpan::FULL)
        .with_target(false)
        .with_writer(log_file_layout)
        .with_filter(LevelFilter::INFO);

    // 终端日志
    let stdout_layer = fmt::layer()
        // 显示日志级别
        .with_level(true)
        // 显示行号
        .with_line_number(true)
        // 显示文件名
        .with_file(false)
        // 显示线程ID
        .with_thread_ids(false)
        // 显示目标
        .with_target(false)
        // 日志时间格式
        .with_timer(timer)
        // 日志输出最大级别
        .with_filter(LevelFilter::INFO);

    // 合并 appender
    let subscriber = Registry::default().with(file_layer).with(stdout_layer);
    let _ = tracing::subscriber::set_global_default(subscriber);
}
