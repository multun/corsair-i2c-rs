use std::path::PathBuf;

use clap::{Parser, Subcommand};

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

mod drivers;
mod ee1004;

use drivers::single_led::*;
use ee1004::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The linux i2c bus number
    #[clap(short, long)]
    bus: u16,

    #[command(subcommand)]
    command: ArgCommands,
}

#[derive(Subcommand, Debug)]
enum ArgCommands {
    /// Sets the brightness of a single LED device
    SetBrightness {
        /// A brightness percentage intensity
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        value: u8,
    },
    /// Turn off all lights
    Disable,
}

fn main() -> Result<(), LinuxI2CError> {
    let args = Args::parse();

    let modules = probe_bus_ee1004(args.bus)?;
    for module in &modules {
        println!("found module: {0:?}", module);
        if !module.spd_data.part_number.starts_with(b"CMU") {
            continue;
        }

        let bus_path = PathBuf::from(format!("/dev/i2c-{0}", args.bus));
        let i2c_dev = LinuxI2CDevice::new(bus_path, module.addr + 8)?;
        let mut led_dev = CorsairSingleColor::from(i2c_dev);

        match args.command {
            ArgCommands::SetBrightness { value } => led_dev.set_brightness(value)?,
            ArgCommands::Disable => led_dev.disable()?,
        };
    }

    Ok(())
}
