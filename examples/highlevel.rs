use log::{info, Level};
use simple_logger::SimpleLogger;

use ultralight_rs::{App, Config, Logger, Overlay, Settings, ULString, Window, WindowFlags};
use ultralight_sys::ULLogLevel;

struct LoggerImpl {}

impl Logger for LoggerImpl {
    fn log_message(level: ULLogLevel, message: ULString) {
        let ll = match level {
            ULLogLevel::kLogLevel_Error => Level::Error,
            ULLogLevel::kLogLevel_Warning => Level::Warn,
            ULLogLevel::kLogLevel_Info => Level::Info,
        };
        log::log!(target: "UL", ll, "{}", Into::<String>::into(message));
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    ultralight_rs::platform_logger::<LoggerImpl>();

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
