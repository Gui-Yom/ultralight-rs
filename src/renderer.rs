use ultralight_sys::{ulCreateRenderer, ulDestroyRenderer, ULRenderer};

use crate::config::Config;

pub struct Renderer {
    pub raw: ULRenderer,
    created: bool,
}

impl Renderer {
    pub fn new(config: &Config) -> Self {
        unsafe {
            Renderer {
                raw: ulCreateRenderer(config.into()),
                created: true,
            }
        }
    }
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
