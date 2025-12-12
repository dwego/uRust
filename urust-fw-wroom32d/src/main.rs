mod ur_context;
mod ur_led;

use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::{Gpio2};
use crate::ur_led::UrLed;

fn main() -> anyhow::Result<()> {
    println!("uRust: Hello from ESP32-WROOM-32D!");

    let peripherals: Peripherals = Peripherals::take().unwrap();
    let mut led: UrLed<Gpio2> = UrLed::new(peripherals.pins.gpio2);

    loop {
        led.set(true);
        std::thread::sleep(std::time::Duration::from_millis(500));
        led.set(false);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
