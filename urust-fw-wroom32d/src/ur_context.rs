use crate::modules::ur_pins_context::UrPinsContext;
use esp_idf_hal::gpio::OutputPin;

pub struct UrContext<PIN: OutputPin> {
    pub led: UrPinsContext<PIN>,
}
