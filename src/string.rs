use ultralight_sys::{ulCreateStringUTF8, ulDestroyString, ulStringGetData, ulStringGetLength};

pub struct ULString {
    raw: ultralight_sys::ULString,
    created: bool,
}

impl From<&str> for ULString {
    fn from(value: &str) -> Self {
        unsafe {
            ULString {
                raw: ulCreateStringUTF8(value.as_ptr() as *const i8, value.len() as u64),
                created: true,
            }
        }
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
        unsafe {
            String::from_utf16(std::slice::from_raw_parts_mut(
                ulStringGetData(self.raw),
                ulStringGetLength(self.raw) as usize,
            ))
            .unwrap()
        }
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
