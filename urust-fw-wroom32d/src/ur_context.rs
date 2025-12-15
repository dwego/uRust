use crate::modules::ur_pins_context::{UrGpioContext, UrPinsContext};
use crate::modules::ur_timer_context::UrTimerContext;
use esp_idf_hal::gpio::OutputPin;

pub struct UrContext {
    pub pin: UrGpioContext,
    pub time: UrTimerContext,
}
