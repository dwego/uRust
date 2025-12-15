use anyhow::Result;

use crate::modules::gpio::{
    UrGpioContext,
    Esp32Wroom32dPinResolver,
};

mod modules;
mod error;

fn main() -> Result<()> {
    let resolver = Esp32Wroom32dPinResolver::new();

    let gpio = UrGpioContext::new(Box::new(resolver));

    let mut led = gpio.out(2)?;
    let mut led2 = gpio.out(13)?;

    loop {
        led.set(true);
        led2.set(false);
        std::thread::sleep(std::time::Duration::from_millis(500));


        led2.set(true);
        led.set(false);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
