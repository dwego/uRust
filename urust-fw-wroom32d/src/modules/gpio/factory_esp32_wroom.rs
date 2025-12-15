use esp_idf_hal::gpio::{AnyIOPin};
use super::factory::UrPinResolver;

pub struct Esp32Wroom32dPinResolver;

impl Esp32Wroom32dPinResolver {
    pub fn new() -> Self {
        Self
    }

    fn is_valid_gpio(gpio: u8) -> bool {
        if gpio > 39 { return false; }

        if (6..=11).contains(&gpio) { return false; }

        true
    }

    fn is_input_only(gpio: u8) -> bool {
        (34..=39).contains(&gpio)
    }
}

impl UrPinResolver for Esp32Wroom32dPinResolver {
    fn gpio_out(&self, gpio: u8) -> Option<AnyIOPin> {
        if !Self::is_valid_gpio(gpio) { return None; }
        if Self::is_input_only(gpio) { return None; }
        Some(unsafe { AnyIOPin::new(gpio as i32) })
    }

    fn gpio_in(&self, gpio: u8) -> Option<AnyIOPin> {
        if !Self::is_valid_gpio(gpio) { return None; }
        Some(unsafe { AnyIOPin::new(gpio as i32) })
    }
}
