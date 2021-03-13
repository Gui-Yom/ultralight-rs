use std::ffi::c_void;

use ultralight_sys::{
    ulBitmapSurfaceGetBitmap, ulSurfaceClearDirtyBounds, ulSurfaceGetDirtyBounds,
    ulSurfaceGetHeight, ulSurfaceGetRowBytes, ulSurfaceGetSize, ulSurfaceGetUserData,
    ulSurfaceGetWidth, ulSurfaceLockPixels, ulSurfaceResize, ulSurfaceSetDirtyBounds,
    ulSurfaceUnlockPixels, ULIntRect, ULSurface,
};

use crate::Bitmap;

pub struct Surface {
    pub raw: ULSurface,
}

impl Surface {
    /// Width (in pixels).
    pub fn width(&self) -> u32 {
        unsafe { ulSurfaceGetWidth(self.raw) }
    }

    /// Height (in pixels).
    pub fn height(&self) -> u32 {
        unsafe { ulSurfaceGetHeight(self.raw) }
    }

    /// Number of bytes between rows (usually width * 4)
    pub fn row_bytes(&self) -> u32 {
        unsafe { ulSurfaceGetRowBytes(self.raw) }
    }

    /// Size in bytes.
    pub fn size(&self) -> u64 {
        unsafe { ulSurfaceGetSize(self.raw) }
    }

    /// Get the underlying user data pointer (this is only valid if you have set a custom surface implementation via ulPlatformSetSurfaceDefinition).
    pub fn user_data(&self) -> *mut c_void {
        unsafe { ulSurfaceGetUserData(self.raw) }
    }

    /// Get the underlying Bitmap from the default Surface.
    pub fn bitmap(&self) -> Bitmap {
        unsafe { ulBitmapSurfaceGetBitmap(self.raw).into() }
    }

    /// Resize the pixel buffer to a certain width and height (both in pixels).
    /// This should never be called while pixels are locked.
    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            ulSurfaceResize(self.raw, width, height);
        }
    }

    /// Lock the pixel buffer for the current scope and get a pointer to the beginning of the data for reading/writing.
    /// Native pixel format is premultiplied BGRA 32-bit (8 bits per channel).
    pub fn lock_pixels(&self) -> SurfacePixelsGuard {
        unsafe {
            SurfacePixelsGuard {
                pixels: ulSurfaceLockPixels(self.raw),
                surface: self,
            }
        }
    }

    /// Get the dirty bounds.
    /// This value can be used to determine which portion of the pixel buffer has been updated since the last call to ulSurfaceClearDirtyBounds().
    pub fn get_dirty_bounds(&self) -> ULIntRect {
        unsafe { ulSurfaceGetDirtyBounds(self.raw) }
    }

    /// Set the dirty bounds to a certain value.
    /// This is called after the Renderer paints to an area of the pixel buffer. (The new value will be joined with the existing dirty_bounds())
    pub fn set_dirty_bounds(&mut self, bounds: ULIntRect) {
        unsafe {
            ulSurfaceSetDirtyBounds(self.raw, bounds);
        }
    }

    /// Clear the dirty bounds.
    /// You should call this after you're done displaying the Surface.
    pub fn clear_dirty_bounds(&mut self) {
        unsafe {
            ulSurfaceClearDirtyBounds(self.raw);
        }
    }
}

impl From<ULSurface> for Surface {
    fn from(raw: ULSurface) -> Self {
        Surface { raw }
    }
}

pub struct SurfacePixelsGuard<'a> {
    pub pixels: *mut c_void,
    surface: &'a Surface,
}

impl Drop for SurfacePixelsGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            ulSurfaceUnlockPixels(self.surface.raw);
        }
    }
}
