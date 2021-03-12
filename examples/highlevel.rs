use log::{info, Level};
use simple_logger::SimpleLogger;

use ultralight_rs::{
    platform, App, Config, Logger, Overlay, Settings, ULString, Window, WindowFlags,
};
use ultralight_sys::ULLogLevel;

fn main() {
    SimpleLogger::new().init().unwrap();

    platform::enable_default_logger();

    let config = Config::new();
    let settings = Settings::new();
    settings.force_cpu_renderer(true);

    let mut app = App::new(&settings, &config);

    let window = Window::new(
        &app.main_monitor(),
        800,
        480,
        false,
        WindowFlags::kWindowFlags_Titled.0 as u32,
    );
    app.set_window(&window);

    let overlay = Overlay::new(&window, 800, 480, 0, 0);
    let mut view = overlay.view();
    view.load_html(
        r#"
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
        </html>"#,
    );
    view.enable_default_logger();
    view.set_dom_ready_callback(&mut |mut view, _, _, _| {
        let result = view
            .evaluate_script("console.log('hello from js'); 1 + 1")
            .unwrap();
        info!("{}", result.as_number().unwrap());
    });

    app.run();
    info!("Running !")
}
