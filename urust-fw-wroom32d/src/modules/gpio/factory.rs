use esp_idf_hal::gpio::AnyIOPin;

pub trait UrPinResolver {
    fn any_io(&self, gpio: u8) -> Option<AnyIOPin>;
}
