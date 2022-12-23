use anyhow::Result;
use clap::Parser;
use log::{debug, error, info, trace, warn};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    // Temperature Poll Intervall
    #[arg(value_parser = parse_duration, long, default_value = "100")]
    temperature_poll_ms: std::time::Duration,

    // Temperature SPI Interface
    #[arg(long, default_value = "/dev/spidev0.0")]
    temperature_spi: String,
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let ms = arg.parse()?;
    Ok(std::time::Duration::from_millis(ms))
}
fn main() -> Result<()> {
    let args = Args::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.verbose.is_silent())
        .verbosity(args.verbose.log_level_filter())
        .init()?;

    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");

    Ok(())
}
