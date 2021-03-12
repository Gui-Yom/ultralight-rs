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
    ULIntRect, ULRenderTarget, ULScrollEventType, ULSurface, ULView,
};

use crate::internal::{
    log_forward_cb, unpack_closure_view_0, unpack_closure_view_1, unpack_closure_view_create_child,
    unpack_closure_view_cursor, unpack_closure_view_fail_loading, unpack_closure_view_history,
};
use crate::jsc::{JSString, JSValue};
use crate::{Cursor, Renderer, Session, ULString};

pub struct View {
    pub(crate) raw: ULView,
    created: bool,
}

impl View {
    /// Create a View with certain size (in pixels).
    /// You can pass null to 'session' to use the default session.
    pub fn new(
        renderer: &Renderer,
        width: u32,
        height: u32,
        transparent: bool,
        session: Session,
        force_cpu_renderer: bool,
    ) -> Self {
        unsafe {
            View {
                raw: ulCreateView(
                    renderer.raw,
                    width,
                    height,
                    transparent,
                    session.raw,
                    force_cpu_renderer,
                ),
                created: true,
            }
        }
    }

    /// Reload current page.
    pub fn reload(&self) {
        unsafe {
            ulViewReload(self.raw);
        }
    }

    /// Stop all page loads.
    pub fn stop(&self) {
        unsafe {
            ulViewStop(self.raw);
        }
    }

    /// Get the RenderTarget for the View.
    /// Only valid when the GPU renderer is enabled in Config.
    pub fn render_target(&self) -> ULRenderTarget {
        unsafe { ulViewGetRenderTarget(self.raw) }
    }

    /// Get the Surface for the View (native pixel buffer container).
    /// Only valid when the GPU renderer is disabled in Config.
    /// (Will return a nullptr when the GPU renderer is enabled.)
    ///
    /// The default Surface is BitmapSurface but you can provide your
    /// own Surface implementation via ulPlatformSetSurfaceDefinition.
    ///
    /// When using the default Surface, you can retrieve the underlying
    /// bitmap by casting ULSurface to ULBitmapSurface and calling ulBitmapSurfaceGetBitmap().
    pub fn surface(&self) -> ULSurface {
        unsafe { ulViewGetSurface(self.raw) }
    }

    /// Load a raw string of HTML.
    pub fn load_html(&self, html: &str) {
        unsafe {
            ulViewLoadHTML(self.raw, ULString::from(html).into());
        }
    }

    /// Load a URL into main frame.
    pub fn load_url(&self, url: &str) {
        unsafe {
            ulViewLoadURL(self.raw, ULString::from(url).into());
        }
    }

    /// Resize view to a certain width and height (in pixels).
    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            ulViewResize(self.raw, width, height);
        }
    }

    /// Get the width, in pixels.
    pub fn width(&self) -> u32 {
        unsafe { ulViewGetWidth(self.raw) }
    }

    /// Get the height, in pixels.
    pub fn height(&self) -> u32 {
        unsafe { ulViewGetHeight(self.raw) }
    }

    /// Get current title.
    pub fn title(&self) -> ULString {
        unsafe { ulViewGetTitle(self.raw).into() }
    }

    /// Get current URL.
    pub fn url(&self) -> ULString {
        unsafe { ulViewGetURL(self.raw).into() }
    }

    /// Whether or not a view should be painted during the next call to ulRender.
    pub fn needs_repaint(&self) -> bool {
        unsafe { ulViewGetNeedsPaint(self.raw) }
    }

    /// Set whether or not a view should be repainted during the next call to ulRender.
    /// This flag is automatically set whenever the page content changes but you can set it directly
    /// in case you need to force a repaint.
    pub fn set_needs_repaint(&mut self, needs_repaint: bool) {
        unsafe {
            ulViewSetNeedsPaint(self.raw, needs_repaint);
        }
    }

    /// Scroll the page.
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

    /// Evaluates a string of JavaScript.
    ///
    /// - `script` A JSString containing the script to evaluate.
    pub fn evaluate_script(&mut self, script: &str) -> Result<JSValue> {
        unsafe {
            let jsctx = self.lock_js_ctx();
            let result: JSValueRef = JSEvaluateScript(
                jsctx.ctx,
                JSString::from(script).raw,
                null_mut(),
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

    /// Retrieve the JS context for the current scope. See [Mutex] and [MutexGuard].
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

    /// Set callback for when the page finishes loading a URL into a frame.
    pub fn on_finish_loading<T>(&mut self, cb: &mut T)
    where
        T: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetFinishLoadingCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when all JavaScript has been parsed and the document is ready.
    /// This is the best time to make any JavaScript calls that are dependent on DOM elements or scripts on the page.
    pub fn on_dom_ready<T>(&mut self, cb: &mut T)
    where
        T: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetDOMReadyCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the page begins loading a new URL into a frame.
    pub fn on_begin_loading<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetBeginLoadingCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the page title changes.
    pub fn on_change_title<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_1(cb);
            ulViewSetChangeTitleCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the mouse cursor changes.
    pub fn on_change_cursor<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, Cursor),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_cursor(cb);
            ulViewSetChangeCursorCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the page URL changes.
    pub fn on_change_url<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_1(cb);
            ulViewSetChangeURLCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the tooltip changes (usually result of a mouse hover).
    pub fn on_change_tooltip<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_1(cb);
            ulViewSetChangeTooltipCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the JavaScript window object is reset for a new page load.
    /// This is called before any scripts are executed on the page and is the earliest time to setup
    /// any initial JavaScript state or bindings.
    /// The document is not guaranteed to be loaded/parsed at this point.
    /// If you need to make any JavaScript calls that are dependent on DOM elements or scripts on the page, use DOMReady instead.
    /// The window object is lazily initialized (this will not be called on pages with no scripts).
    pub fn on_window_ready<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, u64, bool, ULString),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_0(cb);
            ulViewSetWindowObjectReadyCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the page wants to create a new View.
    /// This is usually the result of a user clicking a link with target="_blank"
    /// or by JavaScript calling window.open(url).
    /// To allow creation of these new Views, you should create a new View in this callback,
    /// resize it to your container, and return it. You are responsible for displaying the returned View.
    /// You should return None if you want to block the action.
    pub fn on_create_child_view<F>(&mut self, handler: &mut F)
    where
        F: FnMut(View, ULString, ULString, bool, ULIntRect) -> Option<View>,
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_create_child(handler);
            ulViewSetCreateChildViewCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when an error occurs while loading a URL into a frame.
    pub fn on_fail_loading<F>(&mut self, cb: &mut F)
    where
        F: FnMut(View, u64, bool, ULString, ULString, ULString, i32),
    {
        unsafe {
            let (user_data, callback) = unpack_closure_view_fail_loading(cb);
            ulViewSetFailLoadingCallback(self.raw, Some(callback), user_data);
        }
    }

    /// Set callback for when the history (back/forward state) is modified.
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
