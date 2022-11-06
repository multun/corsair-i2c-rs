use i2cdev::core::I2CDevice;

pub struct CorsairSingleColor<DevType: I2CDevice> {
    dev: DevType,
}

impl<DeviceT: I2CDevice> CorsairSingleColor<DeviceT> {
    pub fn from(dev: DeviceT) -> CorsairSingleColor<DeviceT> {
        CorsairSingleColor { dev }
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), DeviceT::Error> {
        println!("setting brightness to {brightness}");
        let value = ((100 - brightness as u32) * 63 / 100) as u8;
        self.dev.smbus_write_byte_data(160, value)?;
        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), DeviceT::Error> {
        self.set_brightness(0)
    }
}
