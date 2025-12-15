mod allocator;
mod factory;
pub mod factory_esp32_wroom;
mod handle;
mod types;

pub use factory_esp32_wroom::Esp32WroomPinResolver;

use allocator::GpioAllocator;
use crate::error::{UrError, UrResult};

pub use handle::{UrGpioIn, UrGpioOut};
pub use types::Pull;

pub use factory::UrPinResolver;

pub struct UrGpioContext {
    alloc: GpioAllocator,
    factory: Box<dyn factory::UrPinResolver>,
}

impl UrGpioContext {
    pub fn new(factory: Box<dyn factory::UrPinResolver>) -> Self {
        Self {
            alloc: GpioAllocator::new(),
            factory,
        }
    }

    pub fn out(&self, gpio: u8) -> UrResult<UrGpioOut> {
        self.alloc.alloc(gpio)?;
        if (34..=39).contains(&gpio) { return Err(UrError::InvalidGpio(gpio)); }
        let pin = self.factory.any_io(gpio).ok_or(UrError::InvalidGpio(gpio))?;
        UrGpioOut::new(pin)
    }

    pub fn input(&self, gpio: u8, pull: Pull) -> UrResult<UrGpioIn> {
        self.alloc.alloc(gpio)?;
        let pin = self.factory.any_io(gpio).ok_or(UrError::InvalidGpio(gpio))?;
        UrGpioIn::new(pin, pull)
    }


    pub fn free(&self, gpio: u8) {
        self.alloc.free(gpio);
    }
}
