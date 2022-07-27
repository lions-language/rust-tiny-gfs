pub fn init_file_log(name: &str, path: &str, level: log::LevelFilter) {
    // log
    let log = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "{h} {d} {l} {f}:{L} - {m}{n}",
        )))
        .build(path)
        .unwrap();

    let log_name = name;
    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build(log_name, Box::new(log)))
        .logger(
            log4rs::config::Logger::builder()
                .appender(log_name)
                .additive(false)
                .build("app::log", level),
        )
        .build(
            log4rs::config::Root::builder()
                .appender(log_name)
                .build(level),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
