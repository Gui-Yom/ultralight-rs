use log::{info, Level};
use simple_logger::SimpleLogger;

use ultralight_rs::{platform, App, Config, Overlay, Settings, ULString, Window, WindowFlags};
use ultralight_sys::ULLogLevel;

fn main() {
    SimpleLogger::new().init().unwrap();

    platform::enable_default_logger();

    let config = Config::new();
    let settings = Settings::new();
    settings.force_cpu_renderer(true);

    let mut app = App::new(&settings, &config);

    let mut window = Window::new(
        &app.main_monitor(),
        800,
        480,
        false,
        WindowFlags::kWindowFlags_Titled.0 as u32,
    );
    app.set_window(&window);

    info!("Device to pixels : {}", window.device_to_pixel(800));
    info!("Pixels to device : {}", window.pixels_to_device(800));
    let mut overlay = Overlay::new(
        &window,
        window.device_to_pixel(800) as u32,
        window.device_to_pixel(480) as u32,
        0,
        0,
    );

    window.set_resize_callback(&mut |width, height| {
        overlay.resize(width, height);
    });

    let mut view = overlay.view();
    view.enable_default_logger();
    view.on_dom_ready(&mut |mut view, _, _, _| {
        let result = view
            .evaluate_script("console.log('hello from js'); 1 + 1")
            .unwrap();
        info!("{}", result.as_number().unwrap());
    });

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
            <body>Gibberish</body>
            <script>console.log("hi");</script>
        </html>"#,
    );

    app.run();
    info!("Running !")
}
