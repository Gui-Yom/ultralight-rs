use ultralight_sys::{ulCreateRenderer, ulDestroyRenderer, ULRenderer};

use crate::config::Config;

pub struct Renderer {
    pub raw: ULRenderer,
    created: bool,
}

impl Renderer {
    /// Create the Ultralight Renderer directly.
    /// This does not use any native windows for drawing and allows you to manage your own runloop and painting.
    /// This method is recommended for those wishing to integrate the library into a game.
    /// This singleton manages the lifetime of all Views and coordinates all painting, rendering,
    /// network requests, and event dispatch.
    /// You should only call this once per process lifetime.
    /// You should set up your platform handlers (eg, ulPlatformSetLogger, ulPlatformSetFileSystem, etc.) before calling this.
    /// You will also need to define a font loader before calling this, with [platform::enable_font_loader()]
    /// You should not call this if you are creating an App directly,
    /// it creates its own renderer and provides default implementations for various platform handlers automatically.
    pub fn new(config: &Config) -> Self {
        unsafe {
            Renderer {
                raw: ulCreateRenderer(config.into()),
                created: true,
            }
        }
    }

    // TODO renderer methods
    // TODO ULSession bindings
    // TODO ULBitmap bindings
    // TODO Events bindings
    // TODO ULSurface bindings
}

impl From<ULRenderer> for Renderer {
    fn from(raw: ULRenderer) -> Self {
        Renderer {
            raw,
            created: false,
        }
    }
}

impl Into<ULRenderer> for &Renderer {
    fn into(self) -> ULRenderer {
        self.raw
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyRenderer(self.raw);
            }
        }
    }
}
