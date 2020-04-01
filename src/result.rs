use std::fmt::{Debug,Display};

/// Indicates the status of a simulator operation with either a value, error, or
/// result which will be available after a delay. D is the data type, E is the
/// error type.
#[derive(Debug,PartialEq)]
pub enum SimResult<D, E: Display> {
    /// Error if operation failed.
    Err(E),

    /// Indicates result is not yet available but was successful. First field is
    /// the number of simulator cycles before the value will be ready. A value
    /// of 0 indicates the result is ready. The second field is the value.
    Wait(u16, D),
}

impl<D, E: Display> SimResult<D, E> {
    /// Panics if Err, otherwise returns Wait fields.
    pub fn unwrap(self, panic_msg: &str) -> (u16, D) {
        match self {
            SimResult::Err(e) => panic!(format!("{}: {}", panic_msg, e)),
            SimResult::Wait(c, d) => (c, d),
        }
    }

    /// Returns true if wait is 0. If error returns false.
    pub fn ready(self) -> bool {
        match self {
            SimResult::Err(_e) => false,
            SimResult::Wait(wait, _v) => wait == 0,
        }
    }
}
