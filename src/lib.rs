use std::ffi::CString;

use ultralight_sys::{ulVersionMajor, ulVersionMinor, ulVersionPatch, ulVersionString};

pub use crate::app::*;
pub use crate::config::*;
pub use crate::monitor::*;
pub use crate::overlay::*;
pub use crate::renderer::*;
pub use crate::session::*;
pub use crate::settings::*;
pub use crate::string::*;
pub use crate::view::*;
pub use crate::window::*;

mod app;
mod config;
pub mod helpers;
mod internal;
/// JavascriptCore bindings.
pub mod jsc;
mod monitor;
mod overlay;
/// Functions that control Ultralight environment like filesystem and clipboard.
pub mod platform;
mod renderer;
mod session;
mod settings;
mod string;
mod view;
mod window;

/// Get the numeric major version of the library.
pub fn version_major() -> u32 {
    unsafe { ulVersionMajor() }
}

/// Get the numeric minor version of the library.
pub fn version_minor() -> u32 {
    unsafe { ulVersionMinor() }
}

/// Get the numeric patch version of the library.
pub fn version_patch() -> u32 {
    unsafe { ulVersionPatch() }
}

/// Get the version string of the library in MAJOR.MINOR.PATCH format.
pub fn version_string() -> String {
    unsafe {
        CString::from_raw(ulVersionString() as *mut i8)
            .into_string()
            .unwrap()
    }
}
