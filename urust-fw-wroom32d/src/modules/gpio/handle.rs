use esp_idf_hal::gpio::{
    AnyIOPin,
    Input,
    Output,
    PinDriver,
    Pull as HalPull,
};

use crate::error::{UrError, UrResult};
use super::types::Pull;

pub struct UrGpioOut {
    pin: PinDriver<'static, AnyIOPin, Output>,
}

pub struct UrGpioIn {
    pin: PinDriver<'static, AnyIOPin, Input>,
}

impl UrGpioOut {
    pub fn new(pin: AnyIOPin) -> UrResult<Self> {
        let driver = PinDriver::output(pin).map_err(|_| UrError::HalError)?;
        Ok(Self { pin: driver })
    }

    pub fn set(&mut self, high: bool) {
        if high { let _ = self.pin.set_high(); }
        else { let _ = self.pin.set_low(); }
    }

    pub fn toggle(&mut self) {
        let _ = self.pin.toggle();
    }
}

impl UrGpioIn {
    pub fn new(pin: AnyIOPin, pull: Pull) -> UrResult<Self> {
        let mut driver = PinDriver::input(pin).map_err(|_| UrError::HalError)?;

        let hal_pull = match pull {
            Pull::None => HalPull::Floating,
            Pull::Up => HalPull::Up,
            Pull::Down => HalPull::Down,
        };

        driver.set_pull(hal_pull).map_err(|_| UrError::HalError)?;
        Ok(Self { pin: driver })
    }

    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }
}
