use std::{ffi::c_void, sync::Arc};

#[cfg(windows)]
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE, WAIT_OBJECT_0},
    System::Threading::*,
};

#[cfg(windows)]
use crate::AsyncNVencError;
use crate::{
    safe::encoder::EncoderInternal,
    sys::{enums::NVencPicType, result::NVencError},
};

pub struct BitStream {
    pub(crate) buffer: *mut c_void,
    pub(crate) encoder: Arc<EncoderInternal>,
}

impl Drop for BitStream {
    fn drop(&mut self) {
        println!("Dropping bitstream buffer");
        unsafe {
            self.encoder.destroy_bitstream_buffer(self.buffer).unwrap();
        }
    }
}

unsafe impl Send for BitStream {}

impl BitStream {
    /// Attempts to lock the bit stream, if `wait` is true it will wait
    /// otherwise a `LockBusy` Error may be returned, in which case the
    /// client should retry in a few milliseconds
    pub fn try_lock(&self, wait: bool) -> Result<BitStreamLockGuard<'_>, NVencError> {
        let lock = unsafe { self.encoder.lock_bit_stream_buffer(self.buffer, wait)? };
        Ok(BitStreamLockGuard {
            buffer: self,
            data_ptr: lock.bitstream_buffer,
            data_len: lock.bitstream_size_in_bytes,
            picture_type: lock.picture_type,
        })
    }
}

/// Holds a reference to the `BitStream` and holds the data and associated fields
pub struct BitStreamLockGuard<'a> {
    buffer: &'a BitStream,
    data_ptr: *mut c_void,
    data_len: u32,
    picture_type: NVencPicType,
}

impl BitStreamLockGuard<'_> {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data_ptr as _, self.data_len as _) }
    }

    pub fn picture_type(&self) -> NVencPicType {
        self.picture_type
    }
}

impl<'a> Drop for BitStreamLockGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            self.buffer
                .encoder
                .unlock_bit_stream_buffer(self.buffer.buffer)
                .unwrap();
        }
    }
}

#[cfg(windows)]
pub struct AsyncBitStream {
    pub(crate) bitstream: BitStream,
    pub(crate) event: HANDLE,
}

#[cfg(windows)]
impl AsyncBitStream {
    pub fn try_lock(&self) -> Result<BitStreamLockGuard<'_>, AsyncNVencError> {
        unsafe {
            let result = WaitForSingleObject(self.event, INFINITE);
            if result != WAIT_OBJECT_0 {
                // Error occurred
                return Err(GetLastError().into());
            }
        }
        Ok(self.bitstream.try_lock(false)?)
    }
}

#[cfg(windows)]
impl Drop for AsyncBitStream {
    fn drop(&mut self) {
        unsafe {
            self.bitstream
                .encoder
                .unregister_async_event(self.event)
                .unwrap();
            CloseHandle(self.event).unwrap();
        }
    }
}

#[cfg(windows)]
unsafe impl Send for AsyncBitStream {}
