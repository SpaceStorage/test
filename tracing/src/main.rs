use tracing::{info, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber;

fn main() {

    // stdout logger
    let collector = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    // file logger
    let out_file = RollingFileAppender::new(Rotation::NEVER, "./logs", "debug.log");
    let (file_logger, _guard) = tracing_appender::non_blocking(out_file);
    let file_outer = tracing_subscriber::fmt()
            .with_writer(file_logger)
            .with_ansi(true)
            .json()
            .with_max_level(tracing::Level::TRACE)
            .finish();

    // log to stdout
    tracing::subscriber::with_default(collector, || {
        info!("This will be logged to stdout");
    });

    // log to file
    tracing::subscriber::with_default(file_outer, || {
        info!("This will be logged to file");
    });

    // log to devnull
    info!("This will _not_ be logged to stdout");
}

