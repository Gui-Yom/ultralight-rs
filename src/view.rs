use std::os::raw::c_void;
use std::ptr::null_mut;

use anyhow::Result;

use ultralight_sys::{
    ulCreateScrollEvent, ulCreateView, ulDestroyScrollEvent, ulDestroyView,
    ulViewCreateInspectorView, ulViewFireScrollEvent, ulViewGetHeight, ulViewGetNeedsPaint,
    ulViewGetRenderTarget, ulViewGetSurface, ulViewGetTitle, ulViewGetURL, ulViewGetWidth,
    ulViewLoadHTML, ulViewLoadURL, ulViewLockJSContext, ulViewReload, ulViewResize,
    ulViewSetAddConsoleMessageCallback, ulViewSetBeginLoadingCallback,
    ulViewSetChangeCursorCallback, ulViewSetChangeTitleCallback, ulViewSetChangeTooltipCallback,
    ulViewSetChangeURLCallback, ulViewSetCreateChildViewCallback, ulViewSetDOMReadyCallback,
    ulViewSetFailLoadingCallback, ulViewSetFinishLoadingCallback, ulViewSetNeedsPaint,
    ulViewSetUpdateHistoryCallback, ulViewSetWindowObjectReadyCallback, ulViewStop,
    ulViewUnlockJSContext, JSContextGetGlobalObject, JSContextRef, JSEvaluateScript, JSValueRef,
    ULIntRect, ULRenderTarget, ULScrollEventType, ULSession, ULSurface, ULView,
};

use crate::internal::{
    log_forward_cb, unpack_closure_view_0, unpack_closure_view_1, unpack_closure_view_create_child,
    unpack_closure_view_cursor, unpack_closure_view_fail_loading, unpack_closure_view_history,
};
use crate::jsc::{JSString, JSValue};
use crate::{Cursor, Renderer, ULString};

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

    pub fn reload(&self) {
        unsafe {
            ulViewReload(self.raw);
        }
    }

    pub fn stop(&self) {
        unsafe {
            ulViewStop(self.raw);
        }
    }

    pub fn render_target(&self) -> ULRenderTarget {
        unsafe { ulViewGetRenderTarget(self.raw) }
    }

    pub fn surface(&self) -> ULSurface {
        unsafe { ulViewGetSurface(self.raw) }
    }

    pub fn load_html(&self, html: &str) {
        unsafe {
            ulViewLoadHTML(self.raw, ULString::from(html).into());
        }
    }

    pub fn load_url(&self, url: &str) {
        unsafe {
            ulViewLoadURL(self.raw, ULString::from(url).into());
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            ulViewResize(self.raw, width, height);
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { ulViewGetWidth(self.raw) }
    }

    pub fn height(&self) -> u32 {
        unsafe { ulViewGetHeight(self.raw) }
    }

    pub fn title(&self) -> ULString {
        unsafe { ulViewGetTitle(self.raw).into() }
    }

    pub fn url(&self) -> ULString {
        unsafe { ulViewGetURL(self.raw).into() }
    }

    pub fn needs_repaint(&self) -> bool {
        unsafe { ulViewGetNeedsPaint(self.raw) }
    }

    pub fn set_needs_repaint(&mut self, needs_repaint: bool) {
        unsafe {
            ulViewSetNeedsPaint(self.raw, needs_repaint);
        }
    }

    pub fn scroll(&mut self, delta_x: i32, delta_y: i32) {
        unsafe {
            let scroll_event = ulCreateScrollEvent(
                ULScrollEventType::kScrollEventType_ScrollByPixel,
                delta_x,
                delta_y,
            );

            ulViewFireScrollEvent(self.raw, scroll_event);

            ulDestroyScrollEvent(scroll_event);
        }
    }

    pub fn get_scroll_height(&mut self) -> Result<f64> {
        self.evaluate_script("document.body.scrollHeight")
            .map(|v| v.as_number().unwrap())
    }

    pub fn evaluate_script(&mut self, script: &str) -> Result<JSValue> {
        unsafe {
            let jsctx = self.lock_js_ctx();
            let result: JSValueRef = JSEvaluateScript(
                jsctx.ctx,
                JSString::from(script).raw,
                JSContextGetGlobalObject(jsctx.ctx),
                null_mut(),
                0,
                null_mut(),
            );
            Ok(JSValue {
                raw: result,
                ctx: jsctx.ctx,
            })
        }
    }

    pub fn enable_default_logger(&mut self) {
        unsafe {
            ulViewSetAddConsoleMessageCallback(
                self.raw,
                Some(log_forward_cb),
                std::ptr::null_mut() as *mut c_void,
            );
        }
    }

    pub fn use_js_ctx<F, R>(&self, consumer: F) -> R
    where
        F: Fn(ultralight_sys::JSContextRef, ultralight_sys::JSObjectRef) -> R,
    {
        unsafe {
            // TODO replaces types with their high-level bindings
            let jsctx = self.lock_js_ctx();
            let jsroot = JSContextGetGlobalObject(jsctx.ctx);
            let r = consumer(jsctx.ctx, jsroot);
            r
        }
    }

    /// Retrieve the JS context for the current scope
    pub fn lock_js_ctx(&self) -> JSCtxGuard {
        unsafe {
            JSCtxGuard {
                ctx: ulViewLockJSContext(self.raw),
                view: self,
            }
        }
    }

    /// Create an inspector for this View, this is useful for debugging and inspecting pages locally.
    /// This will only succeed if you have the inspector assets in your filesystem
    /// the inspector will look for file:///inspector/Main.html when it loads.
    /// The initial dimensions of the returned View are 10x10, you should call ulViewResize on the
    /// returned View to resize it to your desired dimensions.
    /// You will need to call ulDestroyView on the returned instance when you're done using it.
    pub fn create_inspector_view(&self) -> View {
        unsafe {
            // Special because we need to destroy it ourselves
            View {
                raw: ulViewCreateInspectorView(self.raw),
                created: true,
            }
        }
    }

    pub fn on_finish_loading<T>(&mut self, cb: &mut T)
    where
        T: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetFinishLoadingCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_dom_ready<T>(&mut self, cb: &mut T)
    where
        T: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetDOMReadyCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_begin_loading<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetBeginLoadingCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_change_title<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_1(cb);
            ulViewSetChangeTitleCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_change_cursor<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, Cursor),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_cursor(cb);
            ulViewSetChangeCursorCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_change_url<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_1(cb);
            ulViewSetChangeURLCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_change_tooltip<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_1(cb);
            ulViewSetChangeTooltipCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_window_ready<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetWindowObjectReadyCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_create_child_view<F>(&mut self, handler: &mut F)
    where
        F: FnMut(View, ULString, ULString, bool, ULIntRect) -> View,
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_create_child(handler);
            ulViewSetCreateChildViewCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_fail_loading<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, u64, bool, ULString, ULString, ULString, i32),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_fail_loading(cb);
            ulViewSetFailLoadingCallback(self.raw, Some(callback), user_data);
        }
    }

    pub fn on_update_history<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_history(cb);
            ulViewSetUpdateHistoryCallback(self.raw, Some(callback), user_data);
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

pub struct JSCtxGuard<'a> {
    pub ctx: JSContextRef,
    view: &'a View,
}

impl Drop for JSCtxGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            ulViewUnlockJSContext(self.view.raw);
        }
    }
}
