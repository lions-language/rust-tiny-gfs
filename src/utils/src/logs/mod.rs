pub struct SimpleFileLog<'a> {
    name: &'a str,
    app_name: &'a str,
    path: &'a str,
    level: log::LevelFilter,
}

pub fn init_simple_file_log<'a>(info: SimpleFileLog<'a>) {
    // log
    let log = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "{h} {d} {l} {f}:{L} - {m}{n}",
        )))
        .build(info.path)
        .unwrap();

    let log_name = info.name;
    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build(log_name, Box::new(log)))
        .logger(
            log4rs::config::Logger::builder()
                .appender(log_name)
                .additive(false)
                .build(info.app_name, info.level),
        )
        .build(
            log4rs::config::Root::builder()
                .appender(log_name)
                .build(info.level),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
