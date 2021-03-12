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
    /// Create the App singleton.
    ///
    /// You should only create one of these per application lifetime.
    /// Certain Config options may be overridden during App creation,
    /// most commonly Config::face_winding and Config::device_scale_hint.
    ///
    /// - `settings` Settings to customize App runtime behavior.
    /// - `config` Config options for the Ultralight renderer.
    pub fn new(settings: &Settings, config: &Config) -> Self {
        unsafe {
            App {
                raw: ulCreateApp(settings.raw, config.raw),
            }
        }
    }

    pub fn new_with_defaults() -> Self {
        unsafe {
            App {
                raw: ulCreateApp(null_mut(), null_mut()),
            }
        }
    }

    /// Run the main loop.
    pub fn run(&self) {
        unsafe {
            ulAppRun(self.raw);
        }
    }

    /// Whether or not the App is running.
    pub fn is_running(&self) -> bool {
        unsafe { ulAppIsRunning(self.raw) }
    }

    /// Get the main monitor (this is never NULL).
    ///
    /// We'll add monitor enumeration later.
    pub fn main_monitor(&self) -> Monitor {
        unsafe { ulAppGetMainMonitor(self.raw).into() }
    }

    /// Get the underlying Renderer instance.
    pub fn renderer(&self) -> Renderer {
        unsafe { ulAppGetRenderer(self.raw).into() }
    }

    /// Quit the application.
    pub fn quit(&self) {
        unsafe { ulAppQuit(self.raw) }
    }

    /// Get the main window.
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

    /// Set the main window, you must set this before calling run.
    ///
    /// We currently only support one Window per App, this will change later once we add support for multiple driver instances.
    ///
    /// - `window` The window to use for all rendering.
    pub fn set_window(&mut self, window: &Window) {
        unsafe {
            ulAppSetWindow(self.raw, window.raw);
        }
    }

    /// Set a callback for whenever the App updates. You should update all app logic here.
    ///
    /// This event is fired right before the run loop calls Renderer::Update and Renderer::Render.
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

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            ulDestroyApp(self.raw);
        }
    }
}
