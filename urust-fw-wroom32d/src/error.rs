use thiserror::Error;

#[derive(Debug, Error)]
pub enum UrError {
    #[error("invalid gpio: {0}")]
    InvalidGpio(u8),

    #[error("gpio already allocated: {0}")]
    GpioAlreadyAllocated(u8),

    #[error("hal error")]
    HalError,
}

pub type UrResult<T> = Result<T, UrError>;
