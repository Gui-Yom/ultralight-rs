use ultralight_sys::{
    ulConfigSetAnimationTimerDelay, ulConfigSetCachePath, ulConfigSetDeviceScale,
    ulConfigSetEnableImages, ulConfigSetEnableJavaScript, ulConfigSetFaceWinding,
    ulConfigSetFontFamilyFixed, ulConfigSetFontFamilySansSerif, ulConfigSetFontFamilySerif,
    ulConfigSetFontFamilyStandard, ulConfigSetFontGamma, ulConfigSetFontHinting,
    ulConfigSetForceRepaint, ulConfigSetMemoryCacheSize, ulConfigSetMinLargeHeapSize,
    ulConfigSetMinSmallHeapSize, ulConfigSetOverrideRAMSize, ulConfigSetPageCacheSize,
    ulConfigSetRecycleDelay, ulConfigSetResourcePath, ulConfigSetScrollTimerDelay,
    ulConfigSetUseGPURenderer, ulConfigSetUserAgent, ulConfigSetUserStylesheet, ulCreateConfig,
    ulDestroyConfig, ULConfig, ULFaceWinding, ULFontHinting,
};

use crate::string::ULString;

#[derive(Clone)]
pub struct Config {
    pub raw: ULConfig,
    created: bool,
}

pub type FaceWinding = ULFaceWinding;
pub type FontHinting = ULFontHinting;

impl Config {
    /// Create config with default values
    pub fn new() -> Self {
        unsafe {
            Config {
                raw: ulCreateConfig(),
                created: true,
            }
        }
    }

    /// The file path to the directory that contains Ultralight's bundled
    /// resources (eg, cacert.pem and other localized resources).
    pub fn resource_path(&self, resource_path: &str) {
        unsafe {
            let ulstr: ULString = resource_path.into();
            ulConfigSetResourcePath(self.raw, ulstr.into());
        }
    }

    /// The file path to a writable directory that will be used to store cookies,
    /// cached resources, and other persistent data.
    pub fn cache_path(&self, cache_path: &str) {
        unsafe {
            let ulstr: ULString = cache_path.into();
            ulConfigSetCachePath(self.raw, ulstr.into());
        }
    }

    /// When enabled, each View will be rendered to an offscreen GPU texture
    /// using the GPU driver set in Platform::set_gpu_driver. You can fetch
    /// details for the texture via View::render_target.
    ///
    /// When disabled (the default), each View will be rendered to an offscreen
    /// pixel buffer. This pixel buffer can optionally be provided by the user--
    /// for more info see <Ultralight/platform/Surface.h> and View::surface.
    pub fn use_gpu_renderer(&self, use_gpu: bool) {
        unsafe {
            ulConfigSetUseGPURenderer(self.raw, use_gpu);
        }
    }

    /// The amount that the application DPI has been scaled (200% = 2.0).
    /// This should match the device scale set for the current monitor.
    ///
    /// Note: Device scales are rounded to nearest 1/8th (eg, 0.125).
    pub fn device_scale(&self, device_scale: f64) {
        unsafe {
            ulConfigSetDeviceScale(self.raw, device_scale);
        }
    }

    /// The winding order for front-facing triangles.
    ///
    /// Note: This is only used when the GPU renderer is enabled.
    pub fn face_winding(&self, face_winding: FaceWinding) {
        unsafe {
            ulConfigSetFaceWinding(self.raw, face_winding);
        }
    }

    /// Whether or not images should be enabled.
    pub fn enable_images(&self, enable_images: bool) {
        unsafe {
            ulConfigSetEnableImages(self.raw, enable_images);
        }
    }

    /// Whether or not JavaScript should be enabled.
    pub fn enable_javascript(&self, enable_javascript: bool) {
        unsafe {
            ulConfigSetEnableJavaScript(self.raw, enable_javascript);
        }
    }

    /// The hinting algorithm to use when rendering fonts.
    pub fn font_hinting(&self, font_hinting: FontHinting) {
        unsafe {
            ulConfigSetFontHinting(self.raw, font_hinting);
        }
    }

    /// The gamma to use when compositing font glyphs, change this value to
    /// adjust contrast (Adobe and Apple prefer 1.8, others may prefer 2.2).
    pub fn font_gamma(&self, font_gamma: f64) {
        unsafe {
            ulConfigSetFontGamma(self.raw, font_gamma);
        }
    }

    /// Default font-family to use.
    pub fn font_family_standard(&self, font_family_standard: &str) {
        unsafe {
            let ulstr: ULString = font_family_standard.into();
            ulConfigSetFontFamilyStandard(self.raw, ulstr.into());
        }
    }

    /// Default font-family to use for fixed fonts. (pre/code)
    pub fn font_family_fixed(&self, font_family_fixed: &str) {
        unsafe {
            let ulstr: ULString = font_family_fixed.into();
            ulConfigSetFontFamilyFixed(self.raw, ulstr.into());
        }
    }

    /// Default font-family to use for serif fonts.
    pub fn font_family_serif(&self, font_family_serif: &str) {
        unsafe {
            let ulstr: ULString = font_family_serif.into();
            ulConfigSetFontFamilySerif(self.raw, ulstr.into());
        }
    }

    /// Default font-family to use for sans-serif fonts.
    pub fn font_family_sans_serif(&self, font_family_sans_serif: &str) {
        unsafe {
            let ulstr: ULString = font_family_sans_serif.into();
            ulConfigSetFontFamilySansSerif(self.raw, ulstr.into());
        }
    }

    /// Default user-agent string.
    pub fn user_agent(&self, user_agent: &str) {
        unsafe {
            let ulstr: ULString = user_agent.into();
            ulConfigSetUserAgent(self.raw, ulstr.into());
        }
    }

    /// Default user stylesheet. You should set this to your own custom CSS
    /// string to define default styles for various DOM elements, scrollbars,
    /// and platform input widgets.
    pub fn user_stylesheet(&self, user_stylesheet: &str) {
        unsafe {
            let ulstr: ULString = user_stylesheet.into();
            ulConfigSetUserStylesheet(self.raw, ulstr.into());
        }
    }

    /// Whether or not we should continuously repaint any Views or compositor
    /// layers, regardless if they are dirty or not. This is mainly used to
    /// diagnose painting/shader issues.
    pub fn force_repaint(&self, force_repaint: bool) {
        unsafe {
            ulConfigSetForceRepaint(self.raw, force_repaint);
        }
    }

    /// When a CSS animation is active, the amount of time (in seconds) to wait
    /// before triggering another repaint. Default is 60 Hz.
    pub fn animation_timer_delay(&self, animation_timer_delay: f64) {
        unsafe {
            ulConfigSetAnimationTimerDelay(self.raw, animation_timer_delay);
        }
    }

    /// When a smooth scroll animation is active, the amount of time (in seconds)
    /// to wait before triggering another repaint. Default is 60 Hz.
    pub fn scroll_timer_delay(&self, scroll_timer_delay: f64) {
        unsafe {
            ulConfigSetScrollTimerDelay(self.raw, scroll_timer_delay);
        }
    }

    /// The amount of time (in seconds) to wait before running the recycler (will
    /// attempt to return excess memory back to the system).
    pub fn recycle_delay(&self, recycle_delay: f64) {
        unsafe {
            ulConfigSetRecycleDelay(self.raw, recycle_delay);
        }
    }

    /// Size of WebCore's memory cache in bytes.
    ///
    /// You should increase this if you anticipate handling pages with
    /// large resources, Safari typically uses 128+ MiB for its cache.
    pub fn memory_cache_size(&self, memory_cache_size: u32) {
        unsafe {
            ulConfigSetMemoryCacheSize(self.raw, memory_cache_size);
        }
    }

    /// Number of pages to keep in the cache. Defaults to 0 (none).
    ///
    /// Safari typically caches about 5 pages and maintains an on-disk
    /// cache to support typical web-browsing activities. If you increase
    /// this, you should probably increase the memory cache size as well.
    pub fn page_cache_size(&self, page_cache_size: u32) {
        unsafe {
            ulConfigSetPageCacheSize(self.raw, page_cache_size);
        }
    }

    /// JavaScriptCore tries to detect the system's physical RAM size to set
    /// reasonable allocation limits. Set this to anything other than 0 to
    /// override the detected value. Size is in bytes.
    ///
    /// This can be used to force JavaScriptCore to be more conservative with
    /// its allocation strategy (at the cost of some performance).
    pub fn override_ram_size(&self, override_ram_size: u32) {
        unsafe {
            ulConfigSetOverrideRAMSize(self.raw, override_ram_size);
        }
    }

    /// The minimum size of large VM heaps in JavaScriptCore. Set this to a
    /// lower value to make these heaps start with a smaller initial value.
    pub fn min_large_heap_size(&self, min_large_heap_size: u32) {
        unsafe {
            ulConfigSetMinLargeHeapSize(self.raw, min_large_heap_size);
        }
    }

    /// The minimum size of small VM heaps in JavaScriptCore. Set this to a
    /// lower value to make these heaps start with a smaller initial value.
    pub fn min_small_heap_size(&self, min_small_heap_size: u32) {
        unsafe {
            ulConfigSetMinSmallHeapSize(self.raw, min_small_heap_size);
        }
    }
}

impl From<ULConfig> for Config {
    fn from(raw: ULConfig) -> Self {
        Config {
            raw,
            created: false,
        }
    }
}

impl Into<ULConfig> for &Config {
    fn into(self) -> ULConfig {
        self.raw
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyConfig(self.raw);
            }
        }
    }
}
