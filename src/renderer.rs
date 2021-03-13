use ultralight_sys::{
    ulCreateRenderer, ulDestroyRenderer, ulLogMemoryUsage, ulPurgeMemory, ulRender, ulUpdate,
    ULRenderer,
};

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
    /// You should set up your platform handlers before calling this.
    /// You will also need to define a font loader before calling this, with [platform::enable_font_loader()]
    /// You should not call this if you are creating an App directly,
    /// it creates its own renderer and provides default implementations for various platform handlers automatically.
    pub fn new(config: &Config) -> Self {
        unsafe {
            Renderer {
                raw: ulCreateRenderer(config.raw),
                created: true,
            }
        }
    }

    /// Update timers and dispatch internal callbacks (JavaScript and network).
    pub fn update(&self) {
        unsafe {
            ulUpdate(self.raw);
        }
    }

    /// Render all active Views.
    pub fn render(&self) {
        unsafe {
            ulRender(self.raw);
        }
    }

    /// Print detailed memory usage statistics to the log.
    pub fn log_memory_usage(&self) {
        unsafe {
            ulLogMemoryUsage(self.raw);
        }
    }

    /// Attempt to release as much memory as possible.
    /// Don't call this from any callbacks or driver code.
    pub fn purge_memory(&self) {
        unsafe {
            ulPurgeMemory(self.raw);
        }
    }

    // TODO Events bindings
}

impl From<ULRenderer> for Renderer {
    fn from(raw: ULRenderer) -> Self {
        Renderer {
            raw,
            created: false,
        }
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
