use ultralight_sys::{
    ulCreateOverlay, ulDestroyOverlay, ulOverlayFocus, ulOverlayGetHeight, ulOverlayGetView,
    ulOverlayGetWidth, ulOverlayGetX, ulOverlayGetY, ulOverlayHasFocus, ulOverlayHide,
    ulOverlayIsHidden, ulOverlayMoveTo, ulOverlayResize, ulOverlayShow, ulOverlayUnfocus,
    ULOverlay,
};

use crate::{View, Window};

pub struct Overlay {
    raw: ULOverlay,
    created: bool,
}

impl Overlay {
    /// Create a new [Overlay].
    ///
    /// Each Overlay is essentially a View and an on-screen quad.
    /// You should create the Overlay then load content into the underlying View.
    ///
    /// - `window` The window to create the Overlay in. (we currently only support one window per application)
    /// - `width` The width in device coordinates.
    /// - `height` The height in device coordinates.
    /// - `x` The x-position (offset from the left of the Window), in pixels.
    /// - `y` The y-position (offset from the top of the Window), in pixels.
    pub fn new(window: &Window, width: u32, height: u32, x: i32, y: i32) -> Self {
        unsafe {
            Overlay {
                raw: ulCreateOverlay(window.raw, width, height, x, y),
                created: true,
            }
        }
    }

    /// Get the underlying View.
    pub fn view(&self) -> View {
        unsafe { ulOverlayGetView(self.raw).into() }
    }

    /// Hide the overlay (will no longer be drawn).
    pub fn hide(&self) {
        unsafe {
            ulOverlayHide(self.raw);
        }
    }

    /// Show the overlay.
    pub fn show(&self) {
        unsafe {
            ulOverlayShow(self.raw);
        }
    }

    /// Resize the overlay (and underlying View), dimensions should be specified in pixels.
    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            ulOverlayResize(self.raw, width, height);
        }
    }

    /// Move the overlay to a new position (in pixels).
    pub fn move_to(&mut self, x: i32, y: i32) {
        unsafe {
            ulOverlayMoveTo(self.raw, x, y);
        }
    }

    /// Grant this overlay exclusive keyboard focus.
    pub fn focus(&self) {
        unsafe {
            ulOverlayFocus(self.raw);
        }
    }

    /// Remove keyboard focus.
    pub fn unfocus(&self) {
        unsafe {
            ulOverlayUnfocus(self.raw);
        }
    }

    /// Get the width (in pixels).
    pub fn width(&self) -> u32 {
        unsafe { ulOverlayGetWidth(self.raw) }
    }

    /// Get the height (in pixels).
    pub fn height(&self) -> u32 {
        unsafe { ulOverlayGetHeight(self.raw) }
    }

    /// Get the x-position (offset from the left of the Window), in pixels.
    pub fn x(&self) -> i32 {
        unsafe { ulOverlayGetX(self.raw) }
    }

    /// Get the y-position (offset from the top of the Window), in pixels.
    pub fn y(&self) -> i32 {
        unsafe { ulOverlayGetY(self.raw) }
    }

    /// Whether or not the overlay is hidden (not drawn).
    pub fn is_hidden(&self) -> bool {
        unsafe { ulOverlayIsHidden(self.raw) }
    }

    /// Whether or not an overlay has keyboard focus.
    pub fn has_focus(&self) -> bool {
        unsafe { ulOverlayHasFocus(self.raw) }
    }
}

impl From<ULOverlay> for Overlay {
    fn from(raw: ULOverlay) -> Self {
        Overlay {
            raw,
            created: false,
        }
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyOverlay(self.raw)
            }
        }
    }
}
