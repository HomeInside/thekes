// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui_kit::assets::{Assets as ComponentAssets, icon_assets};
use gpui_kit::component::TitleBar;
use gpui_kit::*;
use std::borrow::Cow;

mod database;
mod ui;
mod utils;
use crate::ui::main_window::MainWindow;
use crate::utils::consts;
use database::DatabaseService;
use std::sync::{Arc, Mutex};

icon_assets!(
    ExtraIcons,
    [
        LibraryBig,
        Users,
        Book,
        Bookmark,
        BookCheck,
        CircleCheckBig,
        NotebookPen,
        //
        BookHeart,
        BookOpenCheck,
        ListTodo,
        SquareLibrary,
        TimerReset,
        Palette,
        MoonStar,
        BookText,
        Feather,
        //
        //BookBookmark, // no existe aun
        Accessibility,
        AlarmClock
    ]
);
struct AppAssets;

// from https://gpui-kit.com/docs/assets/#pick-additional-catalog-icons-with-icon_assets
impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = ExtraIcons.load(path)? {
            return Ok(Some(bytes));
        }
        ComponentAssets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = ComponentAssets.list(path)?;
        paths.extend(ExtraIcons.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(AppAssets);
    //let connection = manage::connect()?;
    let conn = database::connect().expect("unable create database connect");
    database::init_db(&conn).expect("unable init_db database");
    let db = Arc::new(DatabaseService::new(conn));

    app.run(move |cx| {
        gpui_kit::init(cx);

        // ventana principal
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1320.), px(800.)), cx)),
            ..TitleBar::window_options()
        };

        gpui_kit::open_window(window_options, cx, |window, cx| {
            window.set_window_title(consts::APP_NAME);

            window.on_window_should_close(cx, |_window, _cx| {
                println!("cerrando thekes...");
                true
            });

            cx.new(|cx| MainWindow::new(window, cx, db))
        })
        .expect("Failed to open main window");
    });
}
