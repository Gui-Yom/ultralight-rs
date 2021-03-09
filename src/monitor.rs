use ultralight_sys::{ulMonitorGetHeight, ulMonitorGetScale, ulMonitorGetWidth, ULMonitor};

pub struct Monitor {
    pub(crate) raw: ULMonitor,
}

impl Monitor {
    pub fn width(&self) -> u32 {
        unsafe { ulMonitorGetWidth(self.raw) }
    }

    pub fn height(&self) -> u32 {
        unsafe { ulMonitorGetHeight(self.raw) }
    }

    pub fn scale(&self) -> f64 {
        unsafe { ulMonitorGetScale(self.raw) }
    }
}

impl From<ULMonitor> for Monitor {
    fn from(raw: ULMonitor) -> Self {
        Monitor { raw }
    }
}
