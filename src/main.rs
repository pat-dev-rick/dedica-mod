use anyhow::Result;
use clap::Parser;
use log::{debug, error, info, trace, warn};
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};

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

    // Configure the SPI peripheral. The 24AA1024 clocks in data on the first
    // rising edge of the clock signal (SPI mode 0). At 3.3 V, clock speeds of up
    // to 10 MHz are supported.
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2_000_000, Mode::Mode0)?;

    let mut buffer = [0u8; 2];
    loop {
        spi.transfer_segments(&[Segment::with_read(&mut buffer)])?;
        let b = u16::from_le_bytes(buffer);
        debug!("Temperature-sensor | incomming bytes: {b:#b}");
    }

    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");

    Ok(())
}
