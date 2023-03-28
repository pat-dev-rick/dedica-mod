use std::time::SystemTime;
use ads1x1x::{Ads1x1x, ModeChangeError, SlaveAddr};
use anyhow::{bail, Result};
use clap::Parser;
use linux_embedded_hal::I2cdev;

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

    // Temperature I2C Interface (ADC - ADS 1115)
    #[arg(long, default_value = "/dev/i2c-1")]
    temperature_i2c: String,

    // Temperature I2C ADC Channel
    #[arg(long, default_value = "1")]
    temperature_i2c_channel: String,
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

    let dev = I2cdev::new(args.temperature_i2c)?;
    let address = SlaveAddr::default();
    let adc = Ads1x1x::new_ads1115(dev, address);

    match adc.into_continuous() {
        Err(ModeChangeError::I2C(e, adc)) => {
            let _dev = adc.destroy_ads1115();
            bail!("{e}");
        }
        Ok(mut adc) => loop {
            let start = SystemTime::now();
            let measurement = adc.read().unwrap();
            println!("Value: {measurement}");
            std::thread::sleep(args.temperature_poll_ms.saturating_sub(start.elapsed()?))
        },
    }
}
