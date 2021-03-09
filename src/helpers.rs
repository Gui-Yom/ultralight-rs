use std::ptr::null_mut;

use anyhow::Result;

use ultralight_sys::{JSEvaluateScript, JSValueRef};

use crate::helpers_internal::unpack_closure_hook_cb;
use crate::jsc::{JSString, JSValue};
use crate::View;

pub fn create_js_function<T>(
    view: &View,
    name: &'static str,
    mut hook: &mut T,
) -> ultralight_sys::JSObjectRef
where
    T: FnMut(
        ultralight_sys::JSContextRef,
        ultralight_sys::JSObjectRef,
        ultralight_sys::JSObjectRef,
        u64,
        *const ultralight_sys::JSValueRef,
        *mut ultralight_sys::JSValueRef,
    ) -> ultralight_sys::JSValueRef,
{
    unsafe {
        let (hook_closure, hook_function) = unpack_closure_hook_cb(&mut hook);

        let classname_str = std::ffi::CString::new(name).unwrap();

        let mut class_def = ultralight_sys::kJSClassDefinitionEmpty.clone();
        class_def.className = classname_str.as_ptr();
        class_def.__bindgen_anon_1.__bindgen_anon_1.callAsFunction = Some(hook_function);

        let jsclass = ultralight_sys::JSClassCreate(&class_def);

        return view
            .use_js_ctx(|jsctx, _| ultralight_sys::JSObjectMake(jsctx, jsclass, hook_closure));
    }
}

pub fn set_js_object_property(view: &View, name: &str, object: ultralight_sys::JSObjectRef) {
    unsafe {
        view.use_js_ctx(|jsctx, jsroot| {
            ultralight_sys::JSObjectSetProperty(
                jsctx,
                jsroot,
                JSString::from(name).raw,
                object,
                0,
                std::ptr::null_mut() as *mut *const ultralight_sys::OpaqueJSValue,
            );
        });
    }
}
