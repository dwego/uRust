use esp_idf_hal::gpio::{Output, OutputPin, PinDriver};

pub struct UrPinsContext<PIN: OutputPin> {
    pin: PinDriver<'static, PIN, Output>,
    is_on: bool,
}

impl<PIN: OutputPin> UrPinsContext<PIN> {
    pub fn new(pin: PIN) -> Self {
        let driver = PinDriver::output(pin).unwrap();
        Self { pin: driver, is_on: false }
    }

    pub fn set(&mut self, is_on: bool) {
        self.is_on = is_on;
        self.apply();
    }

    fn apply(&mut self) {
        if self.is_on {
            self.pin.set_high().unwrap();
        } else {
            self.pin.set_low().unwrap();
        }
    }
}
