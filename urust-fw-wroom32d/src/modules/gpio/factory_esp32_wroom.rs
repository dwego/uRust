use esp_idf_hal::gpio::AnyIOPin;
use super::factory::UrPinResolver;

pub struct Esp32WroomPinResolver;

impl Esp32WroomPinResolver {
    pub fn new() -> Self { Self }

    fn is_valid_gpio(gpio: u8) -> bool {
        if gpio > 39 { return false; }
        if (6..=11).contains(&gpio) { return false; } // flash
        true
    }
}

impl UrPinResolver for Esp32WroomPinResolver {
    fn any_io(&self, gpio: u8) -> Option<AnyIOPin> {
        if !Self::is_valid_gpio(gpio) { return None; }
        Some(unsafe { AnyIOPin::new(gpio as i32) })
    }
}
