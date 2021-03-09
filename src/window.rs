use std::ffi::CString;
use std::os::raw::c_void;

use ultralight_sys::{
    ulCreateWindow, ulDestroyWindow, ulWindowClose, ulWindowSetCloseCallback, ulWindowSetCursor,
    ulWindowSetResizeCallback, ulWindowSetTitle, ULCursor, ULWindow, ULWindowFlags,
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

    pub fn title(&mut self, title: &str) {
        unsafe {
            let str = CString::new(title).unwrap();
            ulWindowSetTitle(self.raw, str.as_ptr());
        }
    }

    pub fn close(&self) {
        unsafe {
            ulWindowClose(self.raw);
        }
    }

    pub fn cursor(&self, cursor: Cursor) {
        unsafe {
            ulWindowSetCursor(self.raw, cursor);
        }
    }

    pub fn resize_callback<F>(&self, cb: &mut F)
    where
        F: FnMut(u32, u32),
    {
        unsafe {
            let (closure, func) = {
                extern "C" fn trampoline<F>(data: *mut c_void, width: u32, height: u32)
                where
                    F: FnMut(u32, u32),
                {
                    let closure: &mut F = unsafe { &mut *(data as *mut F) };
                    (*closure)(width, height);
                }

                (cb as *mut F as *mut c_void, trampoline::<F>)
            };
            ulWindowSetResizeCallback(self.raw, Some(func), closure);
        }
    }

    pub fn close_callback<F: FnMut()>(&self, cb: &mut F) {
        unsafe {
            let (closure, func) = {
                extern "C" fn trampoline<F: FnMut()>(data: *mut c_void) {
                    let closure: &mut F = unsafe { &mut *(data as *mut F) };
                    (*closure)();
                }

                (cb as *mut F as *mut c_void, trampoline::<F>)
            };
            ulWindowSetCloseCallback(self.raw, Some(func), closure);
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
