use chrono::Local;
use env_logger::{Builder, Target};
use log;
use std::env;
use std::io::Write;

fn main() {
    // 日志level 优先级  error > warn > info > debug > trace
    // 设置日志级别环境变量
    env::set_var("RUST_LOG", "debug");
    // env_logger::init(); // 初始化

    // 自定义的方式初始化
    // Builder::from_default_env()
    //     .target(Target::Stdout)
    //     .format(|buf, record| {
    //         writeln!(
    //             buf,
    //             "{} [{}] - {}",
    //             Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //             record.level(),
    //             record.args()
    //         )
    //     })
    //     .init();
    Builder::from_default_env()
        .target(Target::Stdout)
        .format(|buf, record| {
            let file = record.file().unwrap_or("??"); // 文件名
            let line = record.line().unwrap_or(0); // 行号
            writeln!(
                buf,
                "{} [{}] - file:{}#{} {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"), // 本地时间格式
                record.level(),
                file,
                line,
                record.args()
            )
        })
        .init();

    log::info!("abc");
    log::error!("server inner error");
}
