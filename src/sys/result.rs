#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum NVencResult {
    Success,
    NoDevice,
    UnsupportedDevice,
    InvalidEncoderDevice,
    InvalidDevice,
    DeviceNotExist,
    InvalidPtr,
    InvalidParam,
    InvalidEvent,
    InvalidCall,
    OutOfMemory,
    NotInitialized,
    UnsupportedParam,
    LockBusy,
    NotEnoughBuffer,
    InvalidVersion,
    MapFailed,
    NeedMoreInput,
    EncoderBusy,
    EventNotRegisterd,
    Generic,
    IncompatibleClientKey,
    Unimplemented,
    ResourceRegisterFailed,
    ResourceNotRegistered,
    ResourceNotMapped,
    NeedMoreOutput,
}

impl NVencResult {
    pub fn into_error(self) -> Result<(), NVencError> {
        NVencError::from_result(self)
    }
}

impl NVencError {
    pub fn from_result(value: NVencResult) -> Result<(), Self> {
        if value == NVencResult::Success {
            Ok(())
        } else {
            // Safety: They are one case different, so they always align
            unsafe { Err(std::mem::transmute::<NVencResult, NVencError>(value)) }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum NVencError {
    NoDevice = NVencResult::NoDevice as isize,
    UnsupportedDevice,
    InvalidEncoderDevice,
    InvalidDevice,
    DeviceNotExist,
    InvalidPtr,
    InvalidEvent,
    InvalidParam,
    InvalidCall,
    OutOfMemory,
    NotInitialized,
    UnsupportedParam,
    LockBusy,
    NotEnoughBuffer,
    InvalidVersion,
    MapFailed,
    NeedMoreInput,
    EncoderBusy,
    EventNotRegisterd,
    Generic,
    IncompatibleClientKey,
    Unimplemented,
    ResourceRegisterFailed,
    ResourceNotRegistered,
    ResourceNotMapped,
    NeedMoreOutput,
}

impl std::fmt::Display for NVencError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::NoDevice => "No device",
            Self::UnsupportedDevice => "Unsupported device",
            Self::InvalidEncoderDevice => "Invalid encoder device",
            Self::InvalidDevice => "Invalid device",
            Self::DeviceNotExist => "Device does not exist",
            Self::InvalidPtr => "Invalid pointer",
            Self::InvalidEvent => "Invalid event",
            Self::InvalidParam => "Invalid parameter",
            Self::InvalidCall => "Invalid call",
            Self::OutOfMemory => "Out-of-memory",
            Self::NotInitialized => "Not initialized",
            Self::UnsupportedParam => "Unsupported parameter",
            Self::LockBusy => "Buffer lock is busy",
            Self::NotEnoughBuffer => "Not enough buffer",
            Self::InvalidVersion => "Invalid version",
            Self::MapFailed => "Memory map failed",
            Self::NeedMoreInput => "Need more input",
            Self::EncoderBusy => "Encoder is busy",
            Self::EventNotRegisterd => "Event not registered",
            Self::Generic => "Generic error",
            Self::IncompatibleClientKey => "Incompatible client key",
            Self::Unimplemented => "Unimplemented",
            Self::ResourceRegisterFailed => "Resource registration failed",
            Self::ResourceNotRegistered => "Resource not registered",
            Self::ResourceNotMapped => "Resource not mapped",
            Self::NeedMoreOutput => "Need more output",
        };
        f.write_str(message)
    }
}

impl std::error::Error for NVencError {}

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
