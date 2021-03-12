use std::os::raw::c_void;

use log::Level;

use ultralight_sys::{ULMessageLevel, ULMessageSource, ULView};

use crate::string::ULString;
use crate::View;

pub unsafe fn unpack_closure_view_cb<F>(
    closure: &mut F,
) -> (
    *mut c_void,
    unsafe extern "C" fn(
        *mut c_void,
        ULView,
        std::os::raw::c_ulonglong,
        bool,
        ultralight_sys::ULString,
    ),
)
where
    F: FnMut(View, std::os::raw::c_ulonglong, bool, &str),
{
    extern "C" fn trampoline<F>(
        data: *mut c_void,
        caller: ULView,
        frame_id: std::os::raw::c_ulonglong,
        is_main_frame: bool,
        url: ultralight_sys::ULString,
    ) where
        F: FnMut(View, std::os::raw::c_ulonglong, bool, &str),
    {
        let closure: &mut F = unsafe { &mut *(data as *mut F) };
        (*closure)(
            caller.into(),
            frame_id,
            is_main_frame,
            &Into::<String>::into(ULString::from(url)),
        );
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

// JSContextHooks
type ClosureHookCallbackSig = unsafe extern "C" fn(
    ultralight_sys::JSContextRef,
    ultralight_sys::JSObjectRef,
    ultralight_sys::JSObjectRef,
    u64,
    *const ultralight_sys::JSValueRef,
    *mut ultralight_sys::JSValueRef,
) -> ultralight_sys::JSValueRef;

pub unsafe fn unpack_closure_hook_cb<F>(closure: &mut F) -> (*mut c_void, ClosureHookCallbackSig)
where
    F: FnMut(
        ultralight_sys::JSContextRef,
        ultralight_sys::JSObjectRef,
        ultralight_sys::JSObjectRef,
        u64,
        *const ultralight_sys::JSValueRef,
        *mut ultralight_sys::JSValueRef,
    ) -> ultralight_sys::JSValueRef,
{
    unsafe extern "C" fn trampoline<F>(
        ctx: ultralight_sys::JSContextRef,
        function: ultralight_sys::JSObjectRef,
        this_object: ultralight_sys::JSObjectRef,
        argument_count: u64,
        arguments: *const ultralight_sys::JSValueRef,
        exception: *mut ultralight_sys::JSValueRef,
    ) -> ultralight_sys::JSValueRef
    where
        F: FnMut(
            ultralight_sys::JSContextRef,
            ultralight_sys::JSObjectRef,
            ultralight_sys::JSObjectRef,
            u64,
            *const ultralight_sys::JSValueRef,
            *mut ultralight_sys::JSValueRef,
        ) -> ultralight_sys::JSValueRef,
    {
        let closure: &mut F = &mut *(ultralight_sys::JSObjectGetPrivate(function) as *mut F);

        (*closure)(
            ctx,
            function,
            this_object,
            argument_count,
            arguments,
            exception,
        )
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

pub unsafe extern "C" fn log_forward_cb(
    _user_data: *mut ::std::os::raw::c_void,
    _caller: ULView,
    source: ULMessageSource,
    level: ULMessageLevel,
    message: ultralight_sys::ULString,
    line_number: ::std::os::raw::c_uint,
    column_number: ::std::os::raw::c_uint,
    source_id: ultralight_sys::ULString,
) {
    let level = match level {
        ULMessageLevel::kMessageLevel_Error => Level::Error,
        ULMessageLevel::kMessageLevel_Warning => Level::Warn,
        ULMessageLevel::kMessageLevel_Info => Level::Info,
        ULMessageLevel::kMessageLevel_Debug => Level::Debug,
        ULMessageLevel::kMessageLevel_Log => Level::Trace,
    };

    let target = match source {
        ULMessageSource::kMessageSource_XML => "xml",
        ULMessageSource::kMessageSource_JS => "js",
        ULMessageSource::kMessageSource_Network => "network",
        ULMessageSource::kMessageSource_ConsoleAPI => "consoleapi",
        ULMessageSource::kMessageSource_Storage => "storage",
        ULMessageSource::kMessageSource_AppCache => "appcache",
        ULMessageSource::kMessageSource_Rendering => "rendering",
        ULMessageSource::kMessageSource_CSS => "css",
        ULMessageSource::kMessageSource_Security => "security",
        ULMessageSource::kMessageSource_ContentBlocker => "contentblocker",
        ULMessageSource::kMessageSource_Other => "other",
    };

    log::log!(
        target: target,
        level,
        "({}, {}, {}) {}",
        Into::<String>::into(ULString::from(source_id)),
        line_number,
        column_number,
        Into::<String>::into(ULString::from(message))
    );
}
