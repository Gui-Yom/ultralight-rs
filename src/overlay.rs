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
    pub fn new(window: &Window, width: u32, height: u32, x: i32, y: i32) -> Self {
        unsafe {
            Overlay {
                raw: ulCreateOverlay(window.into(), width, height, x, y),
                created: true,
            }
        }
    }

    pub fn view(&self) -> View {
        unsafe { ulOverlayGetView(self.raw).into() }
    }

    pub fn hide(&self) {
        unsafe {
            ulOverlayHide(self.raw);
        }
    }

    pub fn show(&self) {
        unsafe {
            ulOverlayShow(self.raw);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            ulOverlayResize(self.raw, width, height);
        }
    }

    pub fn move_to(&self, x: i32, y: i32) {
        unsafe {
            ulOverlayMoveTo(self.raw, x, y);
        }
    }

    pub fn focus(&self) {
        unsafe {
            ulOverlayFocus(self.raw);
        }
    }

    pub fn unfocus(&self) {
        unsafe {
            ulOverlayUnfocus(self.raw);
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { ulOverlayGetWidth(self.raw) }
    }

    pub fn height(&self) -> u32 {
        unsafe { ulOverlayGetHeight(self.raw) }
    }

    pub fn x(&self) -> i32 {
        unsafe { ulOverlayGetX(self.raw) }
    }

    pub fn y(&self) -> i32 {
        unsafe { ulOverlayGetY(self.raw) }
    }

    pub fn hidden(&self) -> bool {
        unsafe { ulOverlayIsHidden(self.raw) }
    }

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

impl Into<ULOverlay> for Overlay {
    fn into(self) -> ULOverlay {
        self.raw
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
