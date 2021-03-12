use std::ffi::{c_void, CString};

use ultralight_sys::{
    ulCreateWindow, ulDestroyWindow, ulWindowClose, ulWindowDeviceToPixel, ulWindowGetHeight,
    ulWindowGetNativeHandle, ulWindowGetScale, ulWindowGetWidth, ulWindowIsFullscreen,
    ulWindowPixelsToDevice, ulWindowSetCloseCallback, ulWindowSetCursor, ulWindowSetResizeCallback,
    ulWindowSetTitle, ULCursor, ULWindow, ULWindowFlags,
};

use crate::Monitor;

pub struct Window {
    pub raw: ULWindow,
    created: bool,
}

pub type WindowFlags = ULWindowFlags;
pub type Cursor = ULCursor;

impl Window {
    /// Create a new Window.
    ///
    /// - `monitor` The monitor to create the Window on.
    /// - `width` The width (in device coordinates).
    /// - `height` The height (in device coordinates).
    /// - `fullscreen` Whether or not the window is fullscreen.
    /// - `flags` Various window flags.
    pub fn new(monitor: &Monitor, width: u32, height: u32, fullscreen: bool, flags: u32) -> Self {
        unsafe {
            Window {
                raw: ulCreateWindow(monitor.raw, width, height, fullscreen, flags),
                created: true,
            }
        }
    }

    /// Close a window.
    pub fn close(&self) {
        unsafe {
            ulWindowClose(self.raw);
        }
    }

    /// Set the window title.
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let str = CString::new(title).unwrap();
            ulWindowSetTitle(self.raw, str.as_ptr());
        }
    }

    /// Set the cursor for a window.
    pub fn set_cursor(&self, cursor: Cursor) {
        unsafe {
            ulWindowSetCursor(self.raw, cursor);
        }
    }

    /// Get window width (in pixels).
    pub fn width(&self) -> u32 {
        unsafe { ulWindowGetWidth(self.raw) }
    }

    /// Get window height (in pixels).
    pub fn height(&self) -> u32 {
        unsafe { ulWindowGetHeight(self.raw) }
    }

    /// Get the DPI scale of a window.
    pub fn scale(&self) -> f64 {
        unsafe { ulWindowGetScale(self.raw) }
    }

    /// Get whether or not a window is fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        unsafe { ulWindowIsFullscreen(self.raw) }
    }

    /// Get the underlying native window handle. This is:
    /// - HWND on Windows
    /// - NSWindow* on macOS
    /// - GLFWwindow* on Linux
    pub fn native_handle(&self) -> *mut c_void {
        unsafe { ulWindowGetNativeHandle(self.raw) }
    }

    /// Convert device coordinates to pixels using the current DPI scale.
    pub fn device_to_pixel(&self, val: i32) -> i32 {
        unsafe { ulWindowDeviceToPixel(self.raw, val) }
    }

    /// Convert pixels to device coordinates using the current DPI scale.
    pub fn pixels_to_device(&self, val: i32) -> i32 {
        unsafe { ulWindowPixelsToDevice(self.raw, val) }
    }

    /// Set a callback to be notified when a window resizes (parameters are passed back in pixels).
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

    /// Set a callback to be notified when a window closes.
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

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyWindow(self.raw)
            }
        }
    }
}
