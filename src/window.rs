use std::ffi::CString;

use ultralight_sys::{ulCreateWindow, ulDestroyWindow, ulWindowSetTitle, ULWindow, ULWindowFlags};

use crate::monitor::Monitor;

pub struct Window {
    raw: ULWindow,
    created: bool,
}

pub type WindowFlags = ULWindowFlags;

impl Window {
    pub fn new(monitor: &Monitor, width: u32, height: u32, fullscreen: bool, flags: u32) -> Self {
        unsafe {
            Window {
                raw: ulCreateWindow(monitor.raw, width, height, fullscreen, flags),
                created: true,
            }
        }
    }

    pub fn title(&mut self, title: &str) {
        unsafe {
            let str = CString::new(title).unwrap();
            ulWindowSetTitle(self.raw, str.as_ptr());
        }
    }
}

impl From<ULWindow> for Window {
    fn from(raw: ULWindow) -> Self {
        Window {
            raw,
            created: false,
        }
    }
}

impl Into<ULWindow> for &Window {
    fn into(self) -> ULWindow {
        self.raw
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyWindow(self.raw)
            }
        }
    }
}
