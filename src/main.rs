// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui_kit::assets::Assets;
use gpui_kit::component::TitleBar;
use gpui_kit::*;

mod ui;
mod utils;
use crate::ui::main_window::MainWindow;
use crate::utils::consts;

fn main() {
    let app = gpui_kit::application().with_assets(Assets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        // ventana principal
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1280.), px(720.)), cx)),
            ..TitleBar::window_options()
        };

        gpui_kit::open_window(window_options, cx, |window, cx| {
            window.set_window_title(consts::APP_NAME);

            window.on_window_should_close(cx, |_window, _cx| {
                println!("cerrando thekes...");
                true
            });

            cx.new(|cx| MainWindow::new(window, cx))
        })
        .expect("Failed to open main window");
    });
}
