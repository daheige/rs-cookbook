fn main() {
    // let logfile = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    //     .build("log/output.log")
    //     .unwrap();
    //
    // let config = Config::builder()
    //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
    //     .build(Root::builder().appender("logfile").build(LevelFilter::Info))
    //     .unwrap();
    //
    // log4rs::init_config(config).unwrap();

    log4rs::init_file("app.yaml", Default::default()).unwrap();
    log::info!("Hello, world!");
}

#[test]
fn log_test() {
    use log::LevelFilter;
    use log4rs::{
        append::file::FileAppender,
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder,
    };

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
