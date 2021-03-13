use std::ffi::{c_void, CString};

use ultralight_sys::{
    ulBitmapErase, ulBitmapGetBpp, ulBitmapGetFormat, ulBitmapGetHeight, ulBitmapGetRowBytes,
    ulBitmapGetSize, ulBitmapGetWidth, ulBitmapIsEmpty, ulBitmapLockPixels, ulBitmapOwnsPixels,
    ulBitmapSwapRedBlueChannels, ulBitmapUnlockPixels, ulBitmapWritePNG, ulCreateBitmap,
    ulCreateBitmapFromCopy, ulCreateBitmapFromPixels, ulCreateEmptyBitmap, ulDestroyBitmap,
    ULBitmap, ULBitmapFormat,
};

pub struct Bitmap {
    pub raw: ULBitmap,
    created: bool,
}

pub type BitmapFormat = ULBitmapFormat;

impl Bitmap {
    /// Create bitmap with certain dimensions and pixel format.
    pub fn new(width: u32, height: u32, format: BitmapFormat) -> Self {
        unsafe {
            Bitmap {
                raw: ulCreateBitmap(width, height, format),
                created: true,
            }
        }
    }

    /// Create bitmap from existing pixel buffer.
    pub fn new_from_pixels(
        width: u32,
        height: u32,
        format: BitmapFormat,
        row_bytes: u32,
        pixels: *mut c_void,
        size: u64,
        should_copy: bool,
    ) -> Self {
        unsafe {
            Bitmap {
                raw: ulCreateBitmapFromPixels(
                    width,
                    height,
                    format,
                    row_bytes,
                    pixels,
                    size,
                    should_copy,
                ),
                created: true,
            }
        }
    }

    /// Create empty bitmap.
    pub fn new_empty() -> Self {
        unsafe {
            Bitmap {
                raw: ulCreateEmptyBitmap(),
                created: true,
            }
        }
    }

    /// Get the width in pixels.
    pub fn width(&self) -> u32 {
        unsafe { ulBitmapGetWidth(self.raw) }
    }

    /// Get the height in pixels.
    pub fn height(&self) -> u32 {
        unsafe { ulBitmapGetHeight(self.raw) }
    }

    /// Get the pixel format.
    pub fn format(&self) -> BitmapFormat {
        unsafe { ulBitmapGetFormat(self.raw) }
    }

    /// Get the number of bytes per row.
    pub fn row_bytes(&self) -> u32 {
        unsafe { ulBitmapGetRowBytes(self.raw) }
    }

    /// Get the size in bytes of the underlying pixel buffer.
    pub fn size(&self) -> u64 {
        unsafe { ulBitmapGetSize(self.raw) }
    }

    /// Whether or not this bitmap is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { ulBitmapIsEmpty(self.raw) }
    }

    /// Get the bytes per pixel.
    pub fn bpp(&self) -> u32 {
        unsafe { ulBitmapGetBpp(self.raw) }
    }

    /// Whether or not this bitmap owns its own pixel buffer.
    pub fn owns_pixels(&self) -> bool {
        unsafe { ulBitmapOwnsPixels(self.raw) }
    }

    /// Lock pixels for reading/writing for the current scope.
    pub fn lock_pixels(&self) -> BitmapPixelsGuard {
        unsafe {
            BitmapPixelsGuard {
                pixels: ulBitmapLockPixels(self.raw),
                bitmap: self,
            }
        }
    }

    /// Write bitmap to a PNG on disk.
    pub fn write_to_png(&self, path: &str) -> bool {
        unsafe {
            let cstr = CString::new(path).unwrap();
            ulBitmapWritePNG(self.raw, cstr.as_ptr())
        }
    }

    /// Reset bitmap pixels to 0.
    pub fn erase(&mut self) {
        unsafe {
            ulBitmapErase(self.raw);
        }
    }

    /// This converts a BGRA bitmap to RGBA bitmap and vice-versa by swapping the red and blue channels.
    pub fn swap_red_blue(&mut self) {
        unsafe {
            ulBitmapSwapRedBlueChannels(self.raw);
        }
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        unsafe {
            Bitmap {
                raw: ulCreateBitmapFromCopy(self.raw),
                created: true,
            }
        }
    }
}

impl From<ULBitmap> for Bitmap {
    fn from(raw: ULBitmap) -> Self {
        Bitmap {
            raw,
            created: false,
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroyBitmap(self.raw);
            }
        }
    }
}

pub struct BitmapPixelsGuard<'a> {
    pub pixels: *mut c_void,
    bitmap: &'a Bitmap,
}

impl Drop for BitmapPixelsGuard<'_> {
    fn drop(&mut self) {
        unsafe { ulBitmapUnlockPixels(self.bitmap.raw) }
    }
}
