use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::PinDriver;

fn main() -> anyhow::Result<()> {
    println!("uRust: Hello from ESP32-WROOM-32D!");

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    loop {
        led.set_high()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        led.set_low()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
