use std::os::raw::c_void;

use anyhow::Result;

use ultralight_sys::{ulCreateView, ulDestroyView, ulViewLoadHTML, ULSession, ULView};

use crate::helpers_internal::{log_forward_cb, unpack_closure_view_cb};
use crate::jsc::JSValue;
use crate::{helpers, Renderer, ULString};

pub struct View {
    pub(crate) raw: ULView,
    created: bool,
}

impl View {
    pub fn new(
        renderer: &Renderer,
        width: u32,
        height: u32,
        transparent: bool,
        session: ULSession,
        force_cpu_renderer: bool,
    ) -> Self {
        unsafe {
            View {
                raw: ulCreateView(
                    renderer.into(),
                    width,
                    height,
                    transparent,
                    session,
                    force_cpu_renderer,
                ),
                created: true,
            }
        }
    }

    pub fn load_html(&self, html: &str) {
        unsafe { ulViewLoadHTML(self.raw, ULString::from(html).into()) }
    }

    pub fn scroll(&mut self, delta_x: i32, delta_y: i32) {
        unsafe {
            let scroll_event = ultralight_sys::ulCreateScrollEvent(
                ultralight_sys::ULScrollEventType::kScrollEventType_ScrollByPixel,
                delta_x,
                delta_y,
            );

            ultralight_sys::ulViewFireScrollEvent(self.raw, scroll_event);

            ultralight_sys::ulDestroyScrollEvent(scroll_event);
        }
    }

    pub fn get_scroll_height(&mut self) -> Result<f64> {
        self.evaluate_script("document.body.scrollHeight")
            .map(|v| v.as_number().unwrap())
    }

    pub fn evaluate_script(&mut self, script: &'static str) -> Result<JSValue> {
        helpers::evaluate_script(self, script)
    }

    pub fn log_to_stdout(&mut self) {
        unsafe {
            ultralight_sys::ulViewSetAddConsoleMessageCallback(
                self.raw,
                Some(log_forward_cb),
                std::ptr::null_mut() as *mut c_void,
            );
        }
    }

    pub fn set_finish_loading_callback<T>(&mut self, cb: &mut T)
    where
        T: FnMut(View, u64, bool, &str),
    {
        unsafe {
            let (cb_closure, cb_function) = unpack_closure_view_cb(cb);

            ultralight_sys::ulViewSetFinishLoadingCallback(self.raw, Some(cb_function), cb_closure);
        }
    }

    pub fn set_dom_ready_callback<T>(&mut self, cb: &mut T)
    where
        T: FnMut(View, u64, bool, &str),
    {
        unsafe {
            let (cb_closure, cb_function) = unpack_closure_view_cb(cb);

            ultralight_sys::ulViewSetDOMReadyCallback(self.raw, Some(cb_function), cb_closure);
        }
    }

    pub fn use_js_ctx<F, R>(&self, consumer: F) -> R
    where
        F: Fn(ultralight_sys::JSContextRef, ultralight_sys::JSObjectRef) -> R,
    {
        unsafe {
            // TODO replaces types with their high-level bindings
            let jsctx = ultralight_sys::ulViewLockJSContext(self.raw);
            let jsroot = ultralight_sys::JSContextGetGlobalObject(jsctx);
            let r = consumer(jsctx, jsroot);
            ultralight_sys::ulViewUnlockJSContext(self.raw);
            r
        }
    }
}

impl From<ULView> for View {
    fn from(raw: ULView) -> Self {
        View {
            raw,
            created: false,
        }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyView(self.raw)
            }
        }
    }
}
