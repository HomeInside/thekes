use super::subcriptions::ActiveModule;
use gpui_kit::Context;
use gpui_kit::IntoElement;
use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::*;

use super::MainWindow;

// acciones/eventos
impl MainWindow {
    pub fn click_settings(&mut self, cx: &mut Context<Self>) {
        println!("click_settings fn");
        cx.notify();
    }

    pub fn click_toggle_theme(&mut self, cx: &mut Context<Self>) {
        println!("click_toggle_theme fn");

        self.toggle_theme = !self.toggle_theme;

        if self.toggle_theme {
            Theme::change(ThemeMode::Light, None, cx);
        } else {
            Theme::change(ThemeMode::Dark, None, cx);
        }
        cx.notify();
    }
}
