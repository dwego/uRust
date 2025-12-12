mod ur_context;
mod modules;

use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::{Gpio2};
use crate::modules::ur_pins_context::{UrPinsContext};

fn main() -> anyhow::Result<()> {
    println!("uRust: Hello from ESP32-WROOM-32D!");

    let peripherals: Peripherals = Peripherals::take().unwrap();
    let mut led: UrPinsContext<Gpio2> = UrPinsContext::new(peripherals.pins.gpio2);
    let _timer = modules::ur_timer_context::UrTimerContext;
    loop {
        led.set(true);
        _timer.sleep_ms(500);
        led.set(false);
        _timer.sleep_ms(500);
    }
}
