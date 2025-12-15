use anyhow::Result;

use crate::modules::gpio::{
    UrGpioContext,
    Esp32WroomPinResolver,
};

mod modules;
mod error;

fn main() -> Result<()> {
    let resolver = Esp32WroomPinResolver::new();

    let gpio = UrGpioContext::new(Box::new(resolver));

    let mut led = gpio.out(2)?;

    loop {
        led.set(true);
        std::thread::sleep(std::time::Duration::from_millis(500));

        led.set(false);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
