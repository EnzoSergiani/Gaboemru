use std::fs::{self, File};
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use gaboemru::GameBoy;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("logs")?;
    let file = File::create("logs/emulator.log")?;

    let (file_writer, _guard) = tracing_appender::non_blocking(file);

    tracing_subscriber::registry()
        .with(LevelFilter::TRACE)
        .with(fmt::layer())
        .with(fmt::layer().with_writer(file_writer).with_ansi(false))
        .init();

    info!("Emulator started");
    info!("Emulator ended");

    Ok(())
}
