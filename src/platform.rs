use std::ffi::c_void;

use ultralight_sys::{
    ulEnablePlatformFileSystem, ulEnablePlatformFontLoader, ulPlatformSetClipboard,
    ulPlatformSetFileSystem, ulPlatformSetGPUDriver, ulPlatformSetLogger,
    ulPlatformSetSurfaceDefinition, ULClipboard, ULFileSystem, ULGPUDriver, ULLogLevel, ULLogger,
    ULSurfaceDefinition,
};

use crate::ULString;

pub fn enable_platform_fontloader() {
    unsafe {
        ulEnablePlatformFontLoader();
    }
}

pub fn enable_platform_filesystem(path: &str) {
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
        let ptr = (Box::leak(boxed) as &'static mut T) as *mut T as *mut c_void;
        println!("created: 0x{:#?}", ptr);
        ptr
    }
    unsafe extern "C" fn destroy_(user_data: *mut c_void) {
        T::destroy(&mut *(user_data as *mut T));
    } // TODO surface definition trait
      /*
      unsafe extern "C" fn get_width_(user_data: *mut c_void);
      unsafe extern "C" fn get_height_(user_data: *mut c_void);
      unsafe extern "C" fn get_row_bytes_(user_data: *mut c_void);
      unsafe extern "C" fn get_size_(user_data: *mut c_void);
      unsafe extern "C" fn lock_pixels_(user_data: *mut c_void);
      unsafe extern "C" fn unlock_pixels_(user_data: *mut c_void);
      unsafe extern "C" fn resize_(user_data: *mut c_void);*/

    fn create(width: u32, height: u32) -> Self;
    fn destroy(&mut self);
}

pub fn platform_surface_definition<T>()
where
    T: SurfaceDefinition<T>,
    T: 'static,
{
    unsafe {
        let def = ULSurfaceDefinition {
            create: Some(T::create_),
            destroy: Some(T::destroy_),
            get_width: None,
            get_height: None,
            get_row_bytes: None,
            get_size: None,
            lock_pixels: None,
            unlock_pixels: None,
            resize: None,
        };
        ulPlatformSetSurfaceDefinition(def);
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

pub fn platform_clipboard<T: Clipboard>() {
    unsafe {
        let def = ULClipboard {
            clear: Some(T::clear_),
            read_plain_text: Some(T::read_plain_text_),
            write_plain_text: Some(T::write_plain_text_),
        };
        ulPlatformSetClipboard(def);
    }
}

pub trait Logger {
    unsafe extern "C" fn log_message_(level: ULLogLevel, message: ultralight_sys::ULString) {
        Self::log_message(level, message.into());
    }

    fn log_message(level: ULLogLevel, message: ULString);
}

pub fn platform_logger<T: Logger>() {
    unsafe {
        let def = ULLogger {
            log_message: Some(T::log_message_),
        };
        ulPlatformSetLogger(def);
    }
}

pub trait Filesystem {
    unsafe extern "C" fn file_exists_(path: ultralight_sys::ULString) -> bool {
        Self::file_exists(path.into())
    }
    // TODO filesystem trait

    fn file_exists(path: ULString) -> bool;
}

pub fn platform_filesystem<T: Filesystem>() {
    unsafe {
        let def = ULFileSystem {
            file_exists: Some(T::file_exists_),
            get_file_size: None,
            get_file_mime_type: None,
            open_file: None,
            close_file: None,
            read_from_file: None,
        };
        ulPlatformSetFileSystem(def);
    }
}

pub trait GpuDriver {}

pub fn platform_gpudriver<T: GpuDriver>() {
    unsafe {
        let def = ULGPUDriver {
            begin_synchronize: None,
            end_synchronize: None,
            next_texture_id: None,
            create_texture: None,
            update_texture: None,
            destroy_texture: None,
            next_render_buffer_id: None,
            create_render_buffer: None,
            destroy_render_buffer: None,
            next_geometry_id: None,
            create_geometry: None,
            update_geometry: None,
            destroy_geometry: None,
            update_command_list: None,
        };
        ulPlatformSetGPUDriver(def);
    }
}
