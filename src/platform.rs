use std::ffi::c_void;
use std::ptr::slice_from_raw_parts_mut;

use ultralight_sys::{
    ulEnablePlatformFileSystem, ulEnablePlatformFontLoader, ulPlatformSetClipboard,
    ulPlatformSetFileSystem, ulPlatformSetGPUDriver, ulPlatformSetLogger,
    ulPlatformSetSurfaceDefinition, ulStringAssignString, ULClipboard, ULFileHandle, ULFileSystem,
    ULGPUDriver, ULLogLevel, ULLogger, ULSurfaceDefinition,
};

use crate::ULString;

pub fn enable_fontloader() {
    unsafe {
        ulEnablePlatformFontLoader();
    }
}

pub fn enable_default_filesystem(path: &str) {
    unsafe {
        ulEnablePlatformFileSystem(ULString::from(path).into());
    }
}

pub trait SurfaceDefinition<T>
where
    T: SurfaceDefinition<T>,
    T: 'static,
{
    unsafe extern "C" fn create_(width: u32, height: u32) -> *mut c_void {
        let boxed = Box::new(T::create(width, height));
        (Box::leak(boxed) as &'static mut T) as *mut T as *mut c_void
    }

    unsafe extern "C" fn destroy_(user_data: *mut c_void) {
        T::destroy(&mut *(user_data as *mut T));
    }

    unsafe extern "C" fn get_width_(user_data: *mut c_void) -> u32 {
        T::get_width(&*(user_data as *mut T))
    }

    unsafe extern "C" fn get_height_(user_data: *mut c_void) -> u32 {
        T::get_height(&*(user_data as *mut T))
    }

    unsafe extern "C" fn get_row_bytes_(user_data: *mut c_void) -> u32 {
        T::get_row_bytes(&*(user_data as *mut T))
    }

    unsafe extern "C" fn get_size_(user_data: *mut c_void) -> u64 {
        T::get_size(&*(user_data as *mut T))
    }

    unsafe extern "C" fn lock_pixels_(user_data: *mut c_void) -> *mut c_void {
        T::lock_pixels(&mut *(user_data as *mut T))
    }

    unsafe extern "C" fn unlock_pixels_(user_data: *mut c_void) {
        T::unlock_pixels(&mut *(user_data as *mut T));
    }

    unsafe extern "C" fn resize_(user_data: *mut c_void, width: u32, height: u32) {
        T::resize(&mut *(user_data as *mut T), width, height);
    }

    fn create(width: u32, height: u32) -> Self;
    fn destroy(&mut self);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_row_bytes(&self) -> u32;
    fn get_size(&self) -> u64;
    fn lock_pixels(&mut self) -> *mut c_void;
    fn unlock_pixels(&mut self);
    fn resize(&mut self, width: u32, height: u32);
}

pub fn set_surface_definition_impl<T>()
where
    T: SurfaceDefinition<T>,
    T: 'static,
{
    set_surface_definition(ULSurfaceDefinition {
        create: Some(T::create_),
        destroy: Some(T::destroy_),
        get_width: Some(T::get_width_),
        get_height: Some(T::get_height_),
        get_row_bytes: Some(T::get_row_bytes_),
        get_size: Some(T::get_size_),
        lock_pixels: Some(T::lock_pixels_),
        unlock_pixels: Some(T::unlock_pixels_),
        resize: Some(T::resize_),
    });
}

pub fn set_surface_definition(surface_definition: ULSurfaceDefinition) {
    unsafe {
        ulPlatformSetSurfaceDefinition(surface_definition);
    }
}

pub trait Clipboard {
    unsafe extern "C" fn clear_() {
        Self::clear();
    }

    unsafe extern "C" fn read_plain_text_(result: ultralight_sys::ULString) {
        Self::read_plain_text(result.into());
    }

    unsafe extern "C" fn write_plain_text_(text: ultralight_sys::ULString) {
        Self::write_plain_text(text.into());
    }

    fn clear();
    fn read_plain_text(result: ULString);
    fn write_plain_text(text: ULString);
}

pub fn set_clipboard_impl<T: Clipboard>() {
    set_clipboard(ULClipboard {
        clear: Some(T::clear_),
        read_plain_text: Some(T::read_plain_text_),
        write_plain_text: Some(T::write_plain_text_),
    })
}

pub fn set_clipboard(clipboard: ULClipboard) {
    unsafe {
        ulPlatformSetClipboard(clipboard);
    }
}

pub trait Logger {
    unsafe extern "C" fn log_message_(level: ULLogLevel, message: ultralight_sys::ULString) {
        Self::log_message(level, message.into());
    }

    fn log_message(level: ULLogLevel, message: ULString);
}

pub fn set_logger_impl<T: Logger>() {
    set_logger(ULLogger {
        log_message: Some(T::log_message_),
    });
}

pub fn set_logger(logger: ULLogger) {
    unsafe {
        ulPlatformSetLogger(logger);
    }
}

pub fn enable_default_logger() {
    unsafe extern "C" fn logger(level: ULLogLevel, message: ultralight_sys::ULString) {
        let ll = match level {
            ULLogLevel::kLogLevel_Error => log::Level::Error,
            ULLogLevel::kLogLevel_Warning => log::Level::Warn,
            ULLogLevel::kLogLevel_Info => log::Level::Info,
        };
        log::log!(target: "Ultralight", ll, "{}", Into::<String>::into(ULString::from(message)));
    }
    set_logger(ULLogger {
        log_message: Some(logger),
    });
}

pub trait Filesystem {
    unsafe extern "C" fn _file_exists(path: ultralight_sys::ULString) -> bool {
        Self::file_exists(path.into())
    }

    unsafe extern "C" fn _get_file_size(handle: ULFileHandle, result: *mut i64) -> bool {
        if let Some(size) = Self::get_file_size(handle) {
            *result = size;
            true
        } else {
            false
        }
    }

    unsafe extern "C" fn _get_file_mime_type(
        path: ultralight_sys::ULString,
        result: ultralight_sys::ULString,
    ) -> bool {
        if let Some(mime) = Self::get_file_mime_type(path.into()) {
            ulStringAssignString(result, ULString::from(mime.as_ref()).into());
            true
        } else {
            false
        }
    }

    unsafe extern "C" fn _open_file(
        path: ultralight_sys::ULString,
        open_for_writing: bool,
    ) -> ULFileHandle {
        Self::open_file(path.into(), open_for_writing)
    }

    unsafe extern "C" fn _close_file(handle: ULFileHandle) {
        Self::close_file(handle);
    }

    unsafe extern "C" fn _read_from_file(handle: ULFileHandle, data: *mut i8, length: i64) -> i64 {
        Self::read_from_file(
            handle,
            &*slice_from_raw_parts_mut(data, length as usize),
            length,
        )
    }

    fn file_exists(path: ULString) -> bool;
    fn get_file_size(handle: ULFileHandle) -> Option<i64>;
    fn get_file_mime_type(path: ULString) -> Option<String>;
    fn open_file(path: ULString, open_write: bool) -> ULFileHandle;
    fn close_file(handle: ULFileHandle);
    fn read_from_file(handle: ULFileHandle, data: &[i8], length: i64) -> i64;
}

pub fn set_filesystem_impl<T: Filesystem>() {
    set_filesystem(ULFileSystem {
        file_exists: Some(T::_file_exists),
        get_file_size: Some(T::_get_file_size),
        get_file_mime_type: Some(T::_get_file_mime_type),
        open_file: Some(T::_open_file),
        close_file: Some(T::_close_file),
        read_from_file: Some(T::_read_from_file),
    });
}

pub fn set_filesystem(filesystem: ULFileSystem) {
    unsafe {
        ulPlatformSetFileSystem(filesystem);
    }
}

pub fn set_gpu_driver(driver: ULGPUDriver) {
    unsafe {
        ulPlatformSetGPUDriver(driver);
    }
}
