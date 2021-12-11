#[macro_export]
macro_rules! log_init {
    () => {
        // pretty_env_logger::init_timed();
        pretty_env_logger::formatted_timed_builder()
            .write_style(pretty_env_logger::env_logger::WriteStyle::Auto)
            .filter(
                Some(&env!("CARGO_PKG_NAME").replace("-", "_")),
                log::LevelFilter::Trace,
            )
            .format_timestamp_secs()
            .init();
    };
}
