/// Contains the type for the safe BitStream buffer wrapper
pub mod bitstream;
/// Wrapper for the NVEncoder after creating an active session
pub mod encoder;
/// Wrapper around an NVenc input buffer
pub mod input_buffer;
/// Type-state wrapper around session initialization
pub mod session;

#[cfg(windows)]
use crate::sys::result::NVencError;
#[cfg(windows)]
pub use windows::{Win32::Foundation::WIN32_ERROR, core::Error as WindowsError};

#[derive(Debug)]
#[cfg(windows)]
pub enum AsyncNVencError {
    NVenc(NVencError),
    Win32(WIN32_ERROR),
    Windows(WindowsError),
}

#[cfg(windows)]
impl From<NVencError> for AsyncNVencError {
    fn from(value: NVencError) -> Self {
        Self::NVenc(value)
    }
}

#[cfg(windows)]
impl From<WIN32_ERROR> for AsyncNVencError {
    fn from(value: WIN32_ERROR) -> Self {
        Self::Win32(value)
    }
}

#[cfg(windows)]
impl From<WindowsError> for AsyncNVencError {
    fn from(value: WindowsError) -> Self {
        Self::Windows(value)
    }
}
