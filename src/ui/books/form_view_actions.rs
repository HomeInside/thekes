use super::BookFormView;
use crate::database::authors::model::AuthorModel;
use crate::database::books::model::BooksModel;
use gpui_kit::component::WindowExt;
use gpui_kit::component::calendar::Date;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{Context, Window};

impl BookFormView {
    pub fn click_btn_form_save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("click_btn_form_save fn");
    }

    fn validate_and_submit(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<BooksModel> {
        println!("validate_and_submit fn");
        let today = chrono::Local::now().date_naive();

        let title = self.title.read(cx).value().to_string();
        let author_id = self
            .author_id
            .read(cx)
            .selected_index(cx)
            .map(|id| {
                // los id del select comienzan en 0
                (id.row + 1) as i32
            })
            .unwrap_or_default();
        let lang = self.lang.read(cx).value().to_string();
        let synopsis = self.synopsis.read(cx).value().to_string();
        //
        //let published_date = self.published_date.read(cx).value().to_string();
        let published_date_opt: Option<chrono::NaiveDate> = match self.published_date.read(cx).date() {
            Date::Single(Some(pd)) => Some(pd),
            _ => None,
        };
        self.errors.err_published_date = match published_date_opt {
            None => "Selecciona una fecha.".into(),
            Some(d) if d >= today => "La fecha debe ser anterior a la actual(hoy).".into(),
            Some(_) => String::new(),
        };
        //book.published_date = published_date_opt.unwrap_or_default().to_string();
        //
        let publisher = self.publisher.read(cx).value().to_string();
        //
        let page_count = self.page_count.read(cx).value().to_string();
        let format_type = self.format_type.read(cx).value().to_string();
        let genres = self.genres.read(cx).value().to_string();
        let subjects = self.subjects.read(cx).value().to_string();
        //
        let open_library_id = self.open_library_id.read(cx).value().to_string();
        let isbn = self.isbn.read(cx).value().to_string();
        let buy = self.buy.read(cx).value().to_string();
        let cover_color = self.cover_color.read(cx).value().to_string();

        None
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
