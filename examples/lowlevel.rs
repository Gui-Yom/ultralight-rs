use std::ffi::c_void;
use std::ptr::null_mut;

use ultralight_sys::{
    ulAppGetMainMonitor, ulAppRun, ulAppSetWindow, ulCreateApp, ulCreateConfig, ulCreateOverlay,
    ulCreateRenderer, ulCreateSettings, ulCreateStringUTF8, ulCreateView, ulCreateWindow,
    ulDestroyApp, ulDestroyConfig, ulDestroyOverlay, ulDestroySession, ulDestroySettings,
    ulDestroyString, ulDestroyWindow, ulOverlayGetView, ulPlatformSetLogger,
    ulSettingsSetForceCPURenderer, ulViewLoadHTML, ulViewLockJSContext, ulViewSetDOMReadyCallback,
    ulViewSetFinishLoadingCallback, C_Config, JSEvaluateScript, ULConfig, ULString, ULView,
    ULWindowFlags,
};

fn main() {
    unsafe {
        println!("heho");
        let mut config = ulCreateConfig();
        let settings = ulCreateSettings();
        ulSettingsSetForceCPURenderer(settings, true);

        let mut ul_app = ulCreateApp(settings, config);

        ulDestroyConfig(config);
        ulDestroySettings(settings);

        let window = ulCreateWindow(
            ulAppGetMainMonitor(ul_app),
            800,
            480,
            false,
            ULWindowFlags::kWindowFlags_Titled.0 as u32,
        );

        println!("created window");
        ulAppSetWindow(ul_app, window);

        println!("creating overlay");
        let overlay = ulCreateOverlay(window, 800, 480, 0, 0);

        println!("created overlay");

        let view = ulOverlayGetView(overlay);

        extern "C" fn loaded(
            data: *mut c_void,
            view: ULView,
            frame_id: u64,
            is_main_frame: bool,
            url: ULString,
        ) {
            println!("loaded !");
        }

        extern "C" fn ready(
            data: *mut c_void,
            view: ULView,
            frame_id: u64,
            is_main_frame: bool,
            url: ULString,
        ) {
            println!("dom ready !");
        }

        ulViewSetFinishLoadingCallback(view, Some(loaded), null_mut());

        ulViewSetDOMReadyCallback(view, Some(ready), null_mut());

        let html = {
            let raw = r#"
        <html>
            <head>
                <style>
                    body {
                        background-color: black;
                        color: white;
                        font-size: 100px;
                    }
                </style>
            </head>
            <body>Hello</body>
        </html>"#;
            ulCreateStringUTF8(raw.as_ptr().cast(), raw.len() as u64)
        };
        println!("Loading html");
        ulViewLoadHTML(view, html);
        ulDestroyString(html);

        println!("run app");
        ulAppRun(ul_app);

        ulDestroyOverlay(overlay);
        ulDestroyWindow(window);
        ulDestroyApp(ul_app);
    }
}
