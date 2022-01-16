use tracing_appender::rolling::{RollingFileAppender, Rotation};

fn main() {
    let out_file = RollingFileAppender::new(Rotation::NEVER, "./logs", "debug.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(out_file);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
        .init();

    tracing::info!("sleeping for a minute...");

    std::thread::sleep(std::time::Duration::from_secs(1));

    tracing::info!("bye!");
}
