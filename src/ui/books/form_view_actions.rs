use super::BookFormView;
use crate::database::authors::model::AuthorModel;
use crate::database::books::model::BooksModel;
use gpui_kit::component::WindowExt;
use gpui_kit::component::calendar::Date;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{Context, Window};

impl BookFormView {
    pub fn click_btn_form_save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        todo!()
    }
    fn validate_and_submit(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<BooksModel> {
        todo!()
    }
    pub fn clean_fields(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.title.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.synopsis.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });

        // limpiar la lista de errores
        self.errors = Default::default();
    }
}
