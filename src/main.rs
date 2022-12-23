use clap::Parser;

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
fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
