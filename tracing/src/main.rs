use tracing::{Level, info};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_attributes::instrument;

#[instrument(level = "trace")]
pub fn a_unit_of_work(first_parameter: u64) {
    info!(excited = "true", "Tracing is quite cool!");
}

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

    // bunyan format logger
    let formatting_layer = BunyanFormattingLayer::new("tracing_demo".into(), std::io::stdout);
    let subscriber_bunyan = tracing_subscriber::registry::Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);

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

    // log bunyan format
    tracing::subscriber::with_default(subscriber_bunyan, || {
        info!("Orphan event without a parent span");
        a_unit_of_work(2);
    });

}

