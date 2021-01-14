use log::LevelFilter;

pub fn setup_logging(debug: bool) {
    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder.format_timestamp_secs();
    let level = if debug { LevelFilter::Debug } else { LevelFilter::Info };
    builder.filter_level(level);
    builder.init()
}
