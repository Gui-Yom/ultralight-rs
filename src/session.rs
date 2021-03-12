use ultralight_sys::{
    ulCreateSession, ulDefaultSession, ulDestroySession, ulSessionGetDiskPath, ulSessionGetId,
    ulSessionGetName, ulSessionIsPersistent, ULSession,
};

use crate::{Renderer, ULString};

pub struct Session {
    pub raw: ULSession,
    created: bool,
}

impl Session {
    /// Create a Session to store local data in (such as cookies, local storage, application cache, indexed db, etc).
    pub fn new(renderer: Renderer, is_persistent: bool, name: &str) -> Self {
        unsafe {
            Session {
                raw: ulCreateSession(renderer.raw, is_persistent, ULString::from(name).into()),
                created: true,
            }
        }
    }

    /// Get the default session (persistent session named "default").
    pub fn default(renderer: Renderer) -> Self {
        unsafe {
            Session {
                raw: ulDefaultSession(renderer.raw),
                created: false,
            }
        }
    }

    /// Unique numeric Id for the session.
    pub fn id(&self) -> u64 {
        unsafe { ulSessionGetId(self.raw) }
    }

    /// Unique name identifying the session (used for unique disk path).
    pub fn name(&self) -> ULString {
        unsafe { ulSessionGetName(self.raw).into() }
    }

    /// Whether or not is persistent (backed to disk).
    pub fn is_persistent(&self) -> bool {
        unsafe { ulSessionIsPersistent(self.raw) }
    }

    /// The disk path to write to (used by persistent sessions only).
    pub fn disk_path(&self) -> ULString {
        unsafe { ulSessionGetDiskPath(self.raw).into() }
    }
}

impl From<ULSession> for Session {
    fn from(raw: ULSession) -> Self {
        Session {
            raw,
            created: false,
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroySession(self.raw);
            }
        }
    }
}
