use std::ffi::CStr;
use std::string::FromUtf16Error;

use ultralight_sys::{
    ulCreateString, ulCreateStringUTF8, ulDestroyString, ulStringAssignString, ulStringGetData,
    ulStringGetLength, ulStringIsEmpty,
};

/// A handle to an ultralight string.
pub struct ULString {
    raw: ultralight_sys::ULString,
    created: bool,
}

impl ULString {
    pub fn new(value: &str) -> Self {
        unsafe {
            ULString {
                raw: ulCreateStringUTF8(value.as_ptr() as *const i8, value.len() as u64),
                created: true,
            }
        }
    }

    pub fn from_cstring(value: &CStr) -> Self {
        unsafe {
            ULString {
                raw: ulCreateString(value.as_ptr()),
                created: true,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { ulStringIsEmpty(self.raw) }
    }

    pub fn length(&self) -> u64 {
        unsafe { ulStringGetLength(self.raw) }
    }

    /// Warning ! This copies the string to Rust.
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        unsafe {
            String::from_utf16(std::slice::from_raw_parts_mut(
                ulStringGetData(self.raw),
                ulStringGetLength(self.raw) as usize,
            ))
        }
    }

    pub fn set(&mut self, other: &ULString) {
        unsafe {
            ulStringAssignString(self.raw, other.raw);
        }
    }
}

// FIXME produces a link error
/*
impl Clone for ULString {
    fn clone(&self) -> Self {
        unsafe {
            ULString {
                raw: ulCreateStringFromCopy(self.raw),
                created: true,
            }
        }
    }
}*/

impl From<&str> for ULString {
    fn from(value: &str) -> Self {
        ULString::new(value)
    }
}

impl From<ultralight_sys::ULString> for ULString {
    fn from(value: ultralight_sys::ULString) -> Self {
        ULString {
            raw: value,
            created: false,
        }
    }
}

impl Into<String> for ULString {
    fn into(self) -> String {
        self.to_string().unwrap()
    }
}

impl Into<ultralight_sys::ULString> for ULString {
    fn into(self) -> ultralight_sys::ULString {
        self.raw
    }
}

impl Drop for ULString {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyString(self.raw);
            }
        }
    }
}
