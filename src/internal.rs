//! Various functions to unpack closures and get a function pointer.
//! We need to define a trampoline for each closure signature

use std::ffi::c_void;
use std::os::raw::{c_int, c_uint, c_ulonglong};
use std::ptr::null_mut;

use log::Level;

use ultralight_sys::{ULIntRect, ULMessageLevel, ULMessageSource, ULView};

use crate::string::ULString;
use crate::{Cursor, View};

/// Unpacks a closure for that specific signature
/// Valid for :
/// - [ULFinishLoadingCallback]
/// - [ULDOMReadyCallback]
/// - [ULBeginLoadingCallback]
/// - [ULWindowObjectReadyCallback]
pub unsafe fn unpack_closure_view_0<F>(
    closure: &mut F,
) -> (
    *mut c_void,
    unsafe extern "C" fn(*mut c_void, ULView, c_ulonglong, bool, ultralight_sys::ULString),
)
where
    F: FnMut(View, u64, bool, ULString),
{
    extern "C" fn trampoline<F>(
        data: *mut c_void,
        caller: ULView,
        frame_id: c_ulonglong,
        is_main_frame: bool,
        url: ultralight_sys::ULString,
    ) where
        F: FnMut(View, u64, bool, ULString),
    {
        let closure = unsafe { &mut *(data as *mut F) };
        closure(caller.into(), frame_id, is_main_frame, url.into());
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

/// Unpacks a closure for that specific signature
/// Valid for :
/// - [ULChangeTitleCallback]
/// - [ULChangeURLCallback]
/// - [ULChangeTooltipCallback]
pub unsafe fn unpack_closure_view_1<F>(
    closure: &mut F,
) -> (
    *mut c_void,
    unsafe extern "C" fn(*mut c_void, ULView, ultralight_sys::ULString),
)
where
    F: FnMut(View, ULString),
{
    extern "C" fn trampoline<F>(data: *mut c_void, caller: ULView, value: ultralight_sys::ULString)
    where
        F: FnMut(View, ULString),
    {
        let closure = unsafe { &mut *(data as *mut F) };
        closure(caller.into(), value.into());
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

/// Unpacks a closure for that specific signature
/// Valid for :
/// - [ULUpdateHistoryCallback]
pub unsafe fn unpack_closure_view_history<F>(
    closure: &mut F,
) -> (*mut c_void, unsafe extern "C" fn(*mut c_void, ULView))
where
    F: FnMut(View),
{
    extern "C" fn trampoline<F>(data: *mut c_void, caller: ULView)
    where
        F: FnMut(View),
    {
        let closure = unsafe { &mut *(data as *mut F) };
        closure(caller.into());
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

/// Unpacks a closure for that specific signature
/// Valid for :
/// - [ULFailLoadingCallback]
pub unsafe fn unpack_closure_view_create_child<F>(
    closure: &mut F,
) -> (
    *mut c_void,
    unsafe extern "C" fn(
        *mut c_void,
        ULView,
        ultralight_sys::ULString,
        ultralight_sys::ULString,
        bool,
        ULIntRect,
    ) -> ULView,
)
where
    F: FnMut(View, ULString, ULString, bool, ULIntRect) -> Option<View>,
{
    extern "C" fn trampoline<F>(
        data: *mut c_void,
        caller: ULView,
        opener_url: ultralight_sys::ULString,
        target_url: ultralight_sys::ULString,
        is_popup: bool,
        popup_rect: ULIntRect,
    ) -> ULView
    where
        F: FnMut(View, ULString, ULString, bool, ULIntRect) -> Option<View>,
    {
        let closure = unsafe { &mut *(data as *mut F) };
        if let Some(view) = closure(
            caller.into(),
            opener_url.into(),
            target_url.into(),
            is_popup,
            popup_rect,
        ) {
            view.raw
        } else {
            null_mut()
        }
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

/// Unpacks a closure for that specific signature
/// Valid for :
/// - [ULFailLoadingCallback]
pub unsafe fn unpack_closure_view_fail_loading<F>(
    closure: &mut F,
) -> (
    *mut c_void,
    unsafe extern "C" fn(
        *mut c_void,
        ULView,
        c_ulonglong,
        bool,
        ultralight_sys::ULString,
        ultralight_sys::ULString,
        ultralight_sys::ULString,
        c_int,
    ),
)
where
    F: FnMut(View, u64, bool, ULString, ULString, ULString, i32),
{
    extern "C" fn trampoline<F>(
        data: *mut c_void,
        caller: ULView,
        frame_id: u64,
        is_main_frame: bool,
        url: ultralight_sys::ULString,
        description: ultralight_sys::ULString,
        error_domain: ultralight_sys::ULString,
        error_code: i32,
    ) where
        F: FnMut(View, u64, bool, ULString, ULString, ULString, i32),
    {
        let closure = unsafe { &mut *(data as *mut F) };
        closure(
            caller.into(),
            frame_id,
            is_main_frame,
            url.into(),
            description.into(),
            error_domain.into(),
            error_code,
        );
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

/// Unpacks a closure for that specific signature
/// Valid for :
/// - [ULChangeCursorCallback]
pub unsafe fn unpack_closure_view_cursor<F>(
    closure: &mut F,
) -> (
    *mut c_void,
    unsafe extern "C" fn(*mut c_void, ULView, Cursor),
)
where
    F: FnMut(View, Cursor),
{
    extern "C" fn trampoline<F>(data: *mut c_void, caller: ULView, cursor: Cursor)
    where
        F: FnMut(View, Cursor),
    {
        let closure = unsafe { &mut *(data as *mut F) };
        closure(caller.into(), cursor);
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
    _user_data: *mut c_void,
    _caller: ULView,
    source: ULMessageSource,
    level: ULMessageLevel,
    message: ultralight_sys::ULString,
    line_number: c_uint,
    column_number: c_uint,
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
