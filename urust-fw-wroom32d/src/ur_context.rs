use crate::ur_led::UrLed;
use esp_idf_hal::gpio::OutputPin;

pub struct UrContext<PIN: OutputPin> {
    pub led: UrLed<PIN>,
}
