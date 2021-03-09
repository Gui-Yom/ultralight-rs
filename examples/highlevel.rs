use log::info;
use simple_logger::SimpleLogger;
use ultralight_rs::app::App;
use ultralight_rs::config::Config;
use ultralight_rs::overlay::Overlay;
use ultralight_rs::settings::Settings;
use ultralight_rs::window::{Window, WindowFlags};

fn main() {
    SimpleLogger::new().init().unwrap();

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
    view.log_to_stdout();
    view.set_dom_ready_callback(&mut |mut view, _, _, _| {
        view.evaluate_script("console.log('hello from js'); 1 + 1");
    });

    app.run();
    info!("Running !")
}
