use super::AuthorFormView;
use crate::database::authors::model::AuthorModel;
use gpui_kit::component::WindowExt;
use gpui_kit::component::calendar::Date;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{Context, Window};

impl AuthorFormView {
    pub fn click_btn_form_save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("click_btn_form_save fn");
        match self.validate_and_submit(window, cx) {
            Some(author) => {
                println!("aqui guardar en la db");
                match self.author_service.create_author(&author) {
                    Ok(result) => {
                        println!("Autor guardado con Id: {}", result);
                        window.push_notification((NotificationType::Success, "autor guardado con exito"), cx);
                        self.clean_fields(window, cx);
                    }
                    Err(error) => {
                        eprintln!("Error al crear el autor: {error}");
                        window.push_notification((NotificationType::Error, "Error al guardar el autor"), cx);
                    }
                }
            }
            None => println!("YUKAS! guardar en la db"),
        };

        cx.notify();
    }

    fn validate_and_submit(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<AuthorModel> {
        println!("validate_and_submit fn");
        // TODO
        // crear un struct del autor a guardar y manejar
        // la instancia en la validacion de los campos
        let mut author = AuthorModel {
            //id: None,
            id: 1, // deberia ser Option<i32>
            name: self.name.read(cx).value().to_string(),
            last_name: self.lastname.read(cx).value().to_string(),
            birth_place: Some(self.nationality.read(cx).value().to_string()),
            //
            date_of_birth: String::new(), //self.birth_date.read(cx).date().to_string(),
            date_of_death: Some(String::new()),
            //
            genres: self.genre.read(cx).value().to_string(),
            pseudonyms: Some(self.pseudonyms.read(cx).value().to_string()),
            //langs: Some(self.languages.read(cx).value().to_string()),
            writing_lang: Some(self.languages.read(cx).value().to_string()),
            biography: self.biography.read(cx).value().to_string(),
            //
            movements_or_trends: Some(self.literary_movements.read(cx).value().to_string()),
            //
            open_library_id: Some(self.openlibrary_id.read(cx).value().to_string()),
            viaf_id: Some(self.viaf_id.read(cx).value().to_string()),
            //
            notes: Some(self.notes.read(cx).value().to_string()),
            //
            //created_at: chrono::Local::now().date_naive().to_string(),
            ..Default::default()
        };

        // name
        self.errors.err_name = if author.name.is_empty() {
            "Se necesita un nombre.".into()
        } else if author.name.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };

        // last_name
        self.errors.err_lastname = if author.last_name.is_empty() {
            "Se necesita un apellido.".into()
        } else if author.last_name.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };

        // birth_place
        self.errors.err_nationality = match author.birth_place {
            Some(ref bp) => {
                if bp.is_empty() {
                    "Se necesita un nacionalidad.".into()
                } else if bp.len() < 3 {
                    "Mínimo 3 caracteres.".into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        // date_of_birth
        let today = chrono::Local::now().date_naive();
        // obtener la fecha del componente
        // - https://gpui-kit.com/component/date-picker/#handle-date-selection-events
        let birth_date_opt: Option<chrono::NaiveDate> = match self.birth_date.read(cx).date() {
            Date::Single(Some(bd)) => Some(bd),
            _ => None,
        };
        self.errors.err_date_of_birth = match birth_date_opt {
            None => "Selecciona una fecha.".into(),
            Some(d) if d >= today => "La fecha debe ser anterior a hoy.".into(),
            Some(_) => String::new(),
        };

        // date_of_death
        // obtener la fecha del componente
        let death_date_opt: Option<chrono::NaiveDate> = match self.death_date.read(cx).date() {
            Date::Single(Some(dd)) => Some(dd),
            _ => None,
        };

        self.errors.err_date_of_death = match death_date_opt {
            // si no hay fecha seleccionada, es válido
            None => String::new(),
            Some(dd) if dd > today => "esta fecha no puede ser una fecha futura.".into(),
            // la fecha de fallecimiento debe ser mayor a la fecha de nacimiento
            Some(dd)
                if let Some(bd) = birth_date_opt
                    && dd <= bd =>
            {
                "esta fecha debe ser posterior al nacimiento.".into()
            }
            Some(_) => String::new(),
        };

        // genres
        self.errors.err_genres = if author.genres.is_empty() {
            "Se necesita al menos un género literario.".into()
        } else if author.genres.len() < 3 {
            "Mínimo 3 caracteres.".into()
        } else {
            String::new()
        };

        // pseudonyms
        self.errors.err_pseudonyms = match author.pseudonyms {
            Some(ref ps) => {
                if ps.is_empty() {
                    String::new()
                } else if ps.len() < 2 {
                    "EL seudónimo debe contener al menos 2 caracteres.".into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        // langs
        self.errors.err_langs = match author.writing_lang {
            Some(ref la) => {
                if la.is_empty() {
                    String::new()
                } else if la.len() < 3 {
                    "el(los) idioma(s) debe(n) contener al menos 3 caracteres.".into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        // biography
        self.errors.err_biography = if author.biography.is_empty() {
            "Se necesita una biografía.".into()
        } else if author.biography.len() < 20 {
            "Mínimo 20 caracteres.".into()
        } else {
            String::new()
        };

        // movements_or_trends
        self.errors.err_literary_movements = match author.movements_or_trends {
            Some(ref mo) => {
                if mo.is_empty() {
                    String::new()
                } else if mo.len() < 3 {
                    "debe contener al menos 3 caracteres.".into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        // open_library_id
        self.errors.err_openlibrary_id = match author.open_library_id {
            Some(ref ol) => {
                if ol.is_empty() {
                    String::new()
                } else if ol.len() < 7 {
                    "EL Open library ID es de al menos 7 caracteres.".into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        // viaf_id
        self.errors.err_viaf_id = match author.viaf_id {
            Some(ref vi) => {
                if vi.is_empty() {
                    String::new()
                } else if vi.len() < 6 {
                    "EL VIAF ID es de al menos 6 caracteres.".into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        // notes
        self.errors.err_notes = match author.notes {
            Some(ref no) => {
                if no.is_empty() {
                    String::new()
                } else if no.len() < 10 {
                    "debe contener al menos 10 caracteres.".into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        let ok = self.errors.err_name.is_empty()
            && self.errors.err_lastname.is_empty()
            && self.errors.err_nationality.is_empty()
            && self.errors.err_date_of_birth.is_empty()
            && self.errors.err_date_of_death.is_empty()
            && self.errors.err_genres.is_empty()
            && self.errors.err_pseudonyms.is_empty()
            && self.errors.err_langs.is_empty()
            && self.errors.err_biography.is_empty()
            && self.errors.err_literary_movements.is_empty()
            && self.errors.err_openlibrary_id.is_empty()
            && self.errors.err_viaf_id.is_empty()
            && self.errors.err_notes.is_empty();

        author.date_of_birth = birth_date_opt.unwrap_or_default().to_string();
        author.date_of_death = match death_date_opt {
            Some(dd) => Some(dd.to_string()),
            None => None,
        };

        if ok {
            // (self.on_save)(book, window, cx);
            println!("validate_and_submit listo fn");
            println!("validate_and_submit autor: {:?}", author);
            Some(author)
        } else {
            println!("validate_and_submit Error fn");
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
        self.name.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.lastname.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.nationality.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.birth_date.update(cx, |state, cx| {
            state.set_date(chrono::Local::now().date_naive(), window, cx);
        });
        self.death_date.update(cx, |state, cx| {
            state.set_date(chrono::Local::now().date_naive(), window, cx);
        });
        self.genre.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.pseudonyms.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.languages.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.biography.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.openlibrary_id.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.viaf_id.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.literary_movements.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.notes.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });

        // limpiar la lista de errores
        self.errors = Default::default();
    }
}
