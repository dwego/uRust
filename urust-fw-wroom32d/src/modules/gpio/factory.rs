use esp_idf_hal::gpio::AnyIOPin;

pub trait UrPinResolver {
    fn gpio_out(&self, gpio: u8) -> Option<AnyIOPin>;
    fn gpio_in(&self, gpio: u8) -> Option<AnyIOPin>;
}