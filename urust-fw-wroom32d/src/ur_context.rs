use crate::modules::ur_pins_context::UrPinsContext;
use esp_idf_hal::gpio::OutputPin;
use crate::modules::ur_timer_context::UrTimerContext;

pub struct UrContext<PIN: OutputPin> {
    pub pin: UrPinsContext<PIN>,
    pub timer: UrTimerContext,
}
