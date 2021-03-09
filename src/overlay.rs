use ultralight_sys::{ulCreateOverlay, ulDestroyOverlay, ulOverlayGetView, ULOverlay};

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
