use std::ffi::c_void;
use std::ptr::null_mut;

use ultralight_sys::{
    ulAppGetMainMonitor, ulAppGetRenderer, ulAppGetWindow, ulAppIsRunning, ulAppQuit, ulAppRun,
    ulAppSetUpdateCallback, ulAppSetWindow, ulCreateApp, ulDestroyApp, ULApp,
};

use crate::{Config, Monitor, Renderer, Settings, Window};

pub struct App {
    pub raw: ULApp,
}

impl App {
    pub fn new(settings: &Settings, config: &Config) -> Self {
        unsafe {
            App {
                raw: ulCreateApp(settings.into(), config.into()),
            }
        }
    }

    pub fn run(&self) {
        unsafe {
            ulAppRun(self.raw);
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe { ulAppIsRunning(self.raw) }
    }

    pub fn main_monitor(&self) -> Monitor {
        unsafe { ulAppGetMainMonitor(self.raw).into() }
    }
    pub fn renderer(&self) -> Renderer {
        unsafe { ulAppGetRenderer(self.raw).into() }
    }

    pub fn quit(&self) {
        unsafe { ulAppQuit(self.raw) }
    }

    pub fn window(&self) -> Option<Window> {
        unsafe {
            let ptr = ulAppGetWindow(self.raw);
            if ptr != null_mut() {
                Some(ptr.into())
            } else {
                None
            }
        }
    }

    pub fn set_window(&mut self, window: &Window) {
        unsafe {
            ulAppSetWindow(self.raw, window.into());
        }
    }

    pub fn set_update_callback<F: FnMut()>(&mut self, cb: &mut F) {
        unsafe {
            extern "C" fn trampoline<F: FnMut()>(data: *mut c_void) {
                let closure = unsafe { &mut *(data as *mut F) };
                closure();
            }
            ulAppSetUpdateCallback(self.raw, Some(trampoline::<F>), cb as *mut F as *mut c_void);
        }
    }
}

impl Into<ULApp> for App {
    fn into(self) -> ULApp {
        self.raw
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            ulDestroyApp(self.raw);
        }
    }
}
