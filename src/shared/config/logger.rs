use dotenvy::from_filename;
use env_logger::Env;

pub fn setup_logger() {
    from_filename(".env.local").ok();
    let env = Env::default().filter_or("RUST_LOG", "trace");
    env_logger::Builder::from_env(env)
        .format_timestamp_secs()
        .init();
}
