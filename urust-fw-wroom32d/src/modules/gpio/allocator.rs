use core::cell::RefCell;
use heapless::Vec;

use crate::error::{UrError, UrResult};

const MAX_GPIO: usize = 40;

pub struct GpioAllocator {
    used: RefCell<Vec<u8, MAX_GPIO>>,
}

impl GpioAllocator {
    pub fn new() -> Self {
        Self {
            used: RefCell::new(Vec::new()),
        }
    }

    pub fn alloc(&self, gpio: u8) -> UrResult<()> {
        let mut used = self.used.borrow_mut();
        if used.iter().any(|&g| g == gpio) {
            return Err(UrError::GpioAlreadyAllocated(gpio));
        }
        used.push(gpio).map_err(|_| UrError::HalError)?;
        Ok(())
    }

    pub fn free(&self, gpio: u8) {
        let mut used = self.used.borrow_mut();
        used.retain(|&g| g != gpio);
    }
}
