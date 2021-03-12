use std::ffi::{c_void, CString};

use ultralight_sys::{
    ulCreateWindow, ulDestroyWindow, ulWindowClose, ulWindowDeviceToPixel, ulWindowGetHeight,
    ulWindowGetNativeHandle, ulWindowGetScale, ulWindowGetWidth, ulWindowIsFullscreen,
    ulWindowPixelsToDevice, ulWindowSetCloseCallback, ulWindowSetCursor, ulWindowSetResizeCallback,
    ulWindowSetTitle, ULCursor, ULWindow, ULWindowFlags,
};

use crate::Monitor;

pub struct Window {
    raw: ULWindow,
    created: bool,
}

pub type WindowFlags = ULWindowFlags;
pub type Cursor = ULCursor;

impl Window {
    pub fn new(monitor: &Monitor, width: u32, height: u32, fullscreen: bool, flags: u32) -> Self {
        unsafe {
            Window {
                raw: ulCreateWindow(monitor.raw, width, height, fullscreen, flags),
                created: true,
            }
        }
    }

    pub fn close(&self) {
        unsafe {
            ulWindowClose(self.raw);
        }
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let str = CString::new(title).unwrap();
            ulWindowSetTitle(self.raw, str.as_ptr());
        }
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        unsafe {
            ulWindowSetCursor(self.raw, cursor);
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { ulWindowGetWidth(self.raw) }
    }

    pub fn height(&self) -> u32 {
        unsafe { ulWindowGetHeight(self.raw) }
    }

    pub fn scale(&self) -> f64 {
        unsafe { ulWindowGetScale(self.raw) }
    }

    pub fn is_fullscreen(&self) -> bool {
        unsafe { ulWindowIsFullscreen(self.raw) }
    }

    pub fn native_handle(&self) -> *mut c_void {
        unsafe { ulWindowGetNativeHandle(self.raw) }
    }

    pub fn device_to_pixel(&self, val: i32) -> i32 {
        unsafe { ulWindowDeviceToPixel(self.raw, val) }
    }

    pub fn pixels_to_device(&self, val: i32) -> i32 {
        unsafe { ulWindowPixelsToDevice(self.raw, val) }
    }

    pub fn set_resize_callback<F>(&mut self, cb: &mut F)
    where
        F: FnMut(u32, u32),
    {
        unsafe {
            extern "C" fn trampoline<F>(data: *mut c_void, width: u32, height: u32)
            where
                F: FnMut(u32, u32),
            {
                let closure = unsafe { &mut *(data as *mut F) };
                closure(width, height);
            }
            ulWindowSetResizeCallback(self.raw, Some(trampoline::<F>), cb as *mut F as *mut c_void);
        }
    }

    pub fn set_close_callback<F: FnMut()>(&mut self, cb: &mut F) {
        unsafe {
            extern "C" fn trampoline<F: FnMut()>(data: *mut c_void) {
                let closure = unsafe { &mut *(data as *mut F) };
                closure();
            }
            ulWindowSetCloseCallback(self.raw, Some(trampoline::<F>), cb as *mut F as *mut c_void);
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
