use super::BookFormView;
use crate::database::authors::model::AuthorModel;
use crate::database::books::model::BooksModel;
use chrono::Datelike;
use gpui_kit::component::IndexPath;
use gpui_kit::component::WindowExt;
use gpui_kit::component::calendar::Date;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{Context, Window};
impl BookFormView {
    pub fn click_btn_form_save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("click_btn_form_save fn");
        match self.validate_and_submit(window, cx) {
            Some(book) => {
                println!("aqui guardar en la db");
                match self.book_service.create_book(&book) {
                    Ok(result) => {
                        println!("Libro guardado con Id: {}", result);
                        window.push_notification((NotificationType::Success, "Libro guardado con exito"), cx);
                        self.clean_fields(window, cx);
                    }
                    Err(error) => {
                        eprintln!("Error al crear el Libro: {error}");
                        window.push_notification((NotificationType::Error, "Error al guardar el Libro"), cx);
                    }
                }
            }
            None => println!("YUKAS! guardar en la db"),
        };

        cx.notify();
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
        let published_date = self.published_date.read(cx).value();
        let publisher = self.publisher.read(cx).value().to_string();
        let page_count = self.page_count.read(cx).value();
        let format_type = self.format_type.read(cx).value().to_string();
        let genres = self.genres.read(cx).value().to_string();
        let subjects = self.subjects.read(cx).value().to_string();
        let open_library_id = self.open_library_id.read(cx).value().to_string();
        let isbn = self.isbn.read(cx).value().to_string();
        let buy = self.buy.read(cx).value().to_string();
        let cover_color = self.cover_color.read(cx).value().to_string();

        //
        // TODO validacion START
        //
        let title = title.trim();
        self.errors.err_title = if title.is_empty() {
            "Se necesita un título para el libro".into()
        } else if title.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_lang = if lang.is_empty() {
            "Se necesita al menos un idioma".into()
        } else if lang.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_synopsis = if synopsis.is_empty() {
            "Un breve resumen es necesario".into()
        } else if synopsis.len() < 20 {
            "Mínimo 20 caracteres.".into()
        } else {
            String::new()
        };
        let mut publishe_date_year = 0;
        match published_date.parse::<u32>() {
            Ok(value) => {
                // TODO guardar el valor
                println!("aqui el valor: {}", value);
                if value < 1 || value > today.year() as u32 {
                    self.errors.err_published_date =
                        "el año de publicación debe estar entre 1 y el año actual".to_string();
                } else {
                    self.errors.err_published_date = String::new();
                    publishe_date_year = value;
                }
            }
            Err(error) => {
                println!("error al convertir el valor de published_date: {}", error);
                self.errors.err_published_date = "el año de publicación debe estar entre 1 y el año actual".to_string();
            }
        };
        //
        self.errors.err_publisher = if publisher.is_empty() {
            String::new()
        } else if publisher.len() < 3 {
            "La editorial debe contener al menos 3 caracteres".into()
        } else {
            String::new()
        };
        let mut page_counts = 0;
        match page_count.parse::<u32>() {
            Ok(value) => {
                // TODO guardar el valor
                println!("aqui el valor: {}", value);
                if value < 1 || value > 1500 {
                    self.errors.err_page_count = "el número de paginas debe estar entre 1 y 1500".to_string();
                } else {
                    self.errors.err_page_count = String::new();
                    page_counts = value;
                }
            }
            Err(error) => {
                println!("error al convertir el valor de page_count: {}", error);
                self.errors.err_page_count = "el número de paginas debe estar entre 1 y 1500".to_string();
            }
        };

        self.errors.err_format_type = if format_type.is_empty() {
            "Define formato para el libro".into()
        } else if format_type.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_genres = if genres.is_empty() {
            "Agrega uno ó más géneros litearios".into()
        } else if genres.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_subjects = if subjects.is_empty() {
            "Agrega al menos un tema".into()
        } else if subjects.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_open_library_id = if open_library_id.is_empty() {
            String::new()
        } else if open_library_id.len() < 7 {
            "El Open library ID es de al menos 7 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_isbn = if isbn.is_empty() {
            String::new()
        } else if isbn.len() != 13 {
            "El ISBN debe contener 13 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_buy = if buy.is_empty() {
            String::new()
        } else if !buy.starts_with("http://") && !buy.starts_with("https://") {
            "La url de compra debe empezar por https:// ó http://".into()
        } else if buy.len() < 11 {
            "La url de compra debe contener al menos 11 caracteres.".into()
        } else {
            String::new()
        };
        self.errors.err_cover_color = if cover_color.is_empty() {
            String::new()
        } else if cover_color.len() != 8 {
            "El color de portada debe contener 8 caracteres.".into()
        } else {
            String::new()
        };
        //
        // TODO validacion END

        //
        let ok = self.errors.err_title.is_empty()
            && self.errors.err_title.is_empty()
            && self.errors.err_author_id.is_empty()
            && self.errors.err_synopsis.is_empty()
            && self.errors.err_lang.is_empty()
            && self.errors.err_page_count.is_empty()
            && self.errors.err_format_type.is_empty()
            && self.errors.err_genres.is_empty()
            && self.errors.err_subjects.is_empty()
            && self.errors.err_published_date.is_empty()
            && self.errors.err_publisher.is_empty()
            && self.errors.err_open_library_id.is_empty()
            && self.errors.err_isbn.is_empty()
            && self.errors.err_buy.is_empty()
            && self.errors.err_cover_color.is_empty();

        if ok {
            // TODO
            let book: BooksModel = BooksModel {
                id: 1,
                //title,
                title: title.to_string(),
                author_id,
                synopsis,
                lang,
                cover: None,
                cover_bytes: None,
                page_count: page_counts,
                format_type,
                genres: Some(genres),
                subjects: Some(subjects),
                published_date: publishe_date_year.to_string(),
                publisher: Some(publisher),
                open_library_id: Some(open_library_id),
                isbn: Some(isbn),
                buy: Some(buy),
                //cover_color: Option<u32>,
                ..Default::default()
            };
            println!("validate_and_submit book: {:?}", book);

            Some(book)
        } else {
            println!("validate_and_submit Error: {:?}", self.errors);
            window.push_notification(
                (
                    NotificationType::Error,
                    "se encontraron errores al validar los campos, por favor revisa",
                ),
                cx,
            );
            cx.notify();
            None
        }
    }

    pub fn clean_fields(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("clean_fields fn");
        self.title.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.lang.update(cx, |state, cx| state.set_value("", window, cx));
        let default_index = Some(IndexPath::default().row(0));
        self.author_id
            .update(cx, |state, cx| state.set_selected_index(default_index, window, cx));
        self.synopsis.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.published_date.update(cx, |state, cx| {
            let current_date = chrono::Local::now();
            let current_year = current_date.year();
            state.set_value(current_year.to_string(), window, cx);
        });
        self.publisher.update(cx, |state, cx| state.set_value("", window, cx));
        self.page_count.update(cx, |state, cx| state.set_value("1", window, cx));
        self.format_type.update(cx, |state, cx| state.set_value("", window, cx));
        self.genres.update(cx, |state, cx| state.set_value("", window, cx));
        self.subjects.update(cx, |state, cx| state.set_value("", window, cx));
        self.open_library_id
            .update(cx, |state, cx| state.set_value("", window, cx));
        self.isbn.update(cx, |state, cx| state.set_value("", window, cx));
        self.buy.update(cx, |state, cx| state.set_value("", window, cx));
        self.cover_color.update(cx, |state, cx| state.set_value("", window, cx));

        // limpiar la lista de errores
        self.errors = Default::default();
    }
}
