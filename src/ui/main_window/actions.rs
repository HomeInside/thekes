use super::MainWindow;
use super::subcriptions::ActiveModule;
use gpui_kit::Context;
use gpui_kit::IntoElement;
use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::*;

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

    // TO REMOVE
    // TODO falta implementar el módulo de anotaciones
    pub fn load_notes(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Anotaciones")
    }

    // TO CHECK
    pub fn load_active_module(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        //println!("load_active_module fn: {:?}", self.active_module);
        match self.active_module {
            ActiveModule::Dashboard => self.dashboard.clone().into_any_element(),
            ActiveModule::Books => {
                if self.show_book_form {
                    //TODO debe ser algo como self.book_form
                    self.books.clone().into_any_element()
                } else if self.show_books_details {
                    self.books_details.clone().into_any_element()
                } else {
                    self.books.clone().into_any_element()
                }
            }
            ActiveModule::Authors => {
                if self.show_author_form {
                    self.author_form.clone().into_any_element()
                } else if self.show_author_details {
                    self.author_details.clone().into_any_element()
                } else {
                    self.authors.clone().into_any_element()
                }
            }
            //
            ActiveModule::Notes => self.load_notes(cx).into_any_element(),
            _ => div().child("nada aun").into_any_element(),
        }
    } //load_active_module
}
