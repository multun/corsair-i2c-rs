use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use i2cdev::{
    core::I2CDevice,
    linux::{LinuxI2CDevice, LinuxI2CError},
};

#[derive(Debug)]
pub struct SpdData {
    pub part_number: [u8; 20],
}

#[derive(Debug)]
pub struct Module {
    pub addr: u16,
    pub spd_data: SpdData,
}

enum ModuleProbe {
    Success,
    Busy,
    Empty,
}

fn probe_ee1004_slot(bus: u16, addr: u16) -> Result<ModuleProbe, LinuxI2CError> {
    use nix::Error;
    use ModuleProbe::*;
    let bus_path = PathBuf::from(format!("/dev/i2c-{0}", bus));

    // get a handle for this address
    let mut i2c_dev = match LinuxI2CDevice::new(bus_path, addr) {
        Err(LinuxI2CError::Nix(Error::EBUSY)) => return Ok(Busy),
        other => other?,
    };

    // read a byte
    match i2c_dev.smbus_read_byte() {
        Ok(_) => Ok(Success),
        Err(LinuxI2CError::Nix(Error::ENXIO)) => Ok(Empty),
        Err(err) => Err(err),
    }
}

fn parse_spd(buf: &[u8]) -> Option<SpdData> {
    if buf.len() != 512 {
        println!("eeprom has invalid size: {0}", buf.len());
        return None;
    }

    // TODO: properly parse the manufacturer

    let part_number = buf[329..349].try_into().unwrap();
    Some(SpdData { part_number })
}

fn read_ee1004(bus: u16, addr: u16) -> Result<Option<Module>, LinuxI2CError> {
    use ModuleProbe::*;
    let bus_path = format!("/sys/bus/i2c/devices/i2c-{bus}");
    let dev_path = format!("/sys/bus/i2c/devices/{bus}-{addr:04x}");

    // read the at this i2c address and see what happens
    match probe_ee1004_slot(bus, addr)? {
        // if an ENXIO occured, return None
        Empty => return Ok(None),
        // if the probe succeeded, load the driver
        Success => {
            let mut file = File::create(format!("{bus_path}/new_device"))?;
            let data = format!("ee1004 0x{addr:x}\n");
            file.write_all(data.as_bytes())?;
        }
        // if an EBUSY occured, check that the proper driver is loaded
        Busy => {
            let content = std::fs::read_to_string(format!("{dev_path}/name"))?;
            if content != "ee1004\n" {
                println!("invalid driver loaded for bus {bus} addr 0x{addr}");
                return Ok(None);
            }
        }
    };

    // read the chip eeprom
    let mut file = File::open(format!("{dev_path}/eeprom"))?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    let spd_data = match parse_spd(buf.as_slice()) {
        None => return Ok(None),
        Some(spd_data) => spd_data,
    };

    Ok(Some(Module { addr, spd_data }))
}

pub fn probe_bus_ee1004(bus: u16) -> Result<Vec<Module>, LinuxI2CError> {
    let mut res = vec![];
    for addr in 0x50u16..0x58 {
        println!("probing {addr:x}...");
        if let Some(module) = read_ee1004(bus, addr)? {
            res.push(module)
        }
    }
    Ok(res)
}
