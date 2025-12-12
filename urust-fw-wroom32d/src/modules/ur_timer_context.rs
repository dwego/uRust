use esp_idf_hal::delay::FreeRtos;

pub struct UrTimerContext;

impl UrTimerContext {
    pub fn sleep_ms(&self, time_ms: u32) {
        FreeRtos::delay_ms(time_ms);
    }
}