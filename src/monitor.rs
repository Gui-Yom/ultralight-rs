use ultralight_sys::{ulMonitorGetHeight, ulMonitorGetScale, ulMonitorGetWidth, ULMonitor};

pub struct Monitor {
    pub(crate) raw: ULMonitor,
}

impl Monitor {
    /// Get the width of the monitor (in pixels).
    pub fn width(&self) -> u32 {
        unsafe { ulMonitorGetWidth(self.raw) }
    }

    /// Get the height of the monitor (in pixels).
    pub fn height(&self) -> u32 {
        unsafe { ulMonitorGetHeight(self.raw) }
    }

    /// Get the monitor's DPI scale (1.0 = 100%).
    pub fn scale(&self) -> f64 {
        unsafe { ulMonitorGetScale(self.raw) }
    }
}

impl From<ULMonitor> for Monitor {
    fn from(raw: ULMonitor) -> Self {
        Monitor { raw }
    }
}
