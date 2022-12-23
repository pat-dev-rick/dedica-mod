use anyhow::Result;
use clap::Parser;
use log::debug;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
use std::time::SystemTime;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    // Temperature Poll Intervall
    #[arg(value_parser = parse_duration, long, default_value = "500")]
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

    // Configure the SPI peripheral.
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2_000_000, Mode::Mode0)?;

    let mut buffer = [0u8; 2];
    loop {
        let start = SystemTime::now();
        spi.transfer_segments(&[Segment::with_read(&mut buffer)])?;
        let buffer_u16 = u16::from_be_bytes(buffer);
        debug!(
            "Temperature-Sensor | incomming byte 1: {:#010b} | byte 2: {:#010b} | as u16: {:#018b}",
            buffer[0], buffer[1], buffer_u16
        );

        buffer = [0, 0];

        std::thread::sleep(args.temperature_poll_ms.saturating_sub(start.elapsed()?))
    }
}

/*
fn print_max6675_output(input: [u8;2]) {
    let DUMMY_SIGN_BIT =
    let TEMPERATURE_READING =
    let THERMOCOUPLE_INPUT =
    let DEVICE_ID =
    let STATE =
}
*/
