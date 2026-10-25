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
			date_of_birth: String::new(), //self.birth_date.read(cx).date().to_string(),
			// campo opcional en la DB
			date_of_death: Some(String::new()),
			birth_place: Some(self.nationality.read(cx).value().to_string()),
			biography: self.biography.read(cx).value().to_string(),
			genres: self.genre.read(cx).value().to_string(),
			open_library_id: Some(self.openlibrary_id.read(cx).value().to_string()),
			viaf_id: Some(self.viaf_id.read(cx).value().to_string()),
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

		// last_name
		self.errors.biography = if author.biography.is_empty() {
			"Se necesita una biografía.".into()
		} else if author.biography.len() < 20 {
			"Mínimo 20 caracteres.".into()
		} else {
			String::new()
		};

		// date_of_birth
		let today = chrono::Local::now().date_naive();
		// obtener la fecha del componente
		// - https://gpui-kit.com/component/date-picker/#handle-date-selection-events
		let birth_date_opt: Option<chrono::NaiveDate> = match self.birth_date.read(cx).date() {
			Date::Single(Some(bd)) => Some(bd),
			_ => None,
		};
		self.errors.date_of_birth = match birth_date_opt {
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

		self.errors.date_of_death = match death_date_opt {
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

		// birth_place
		self.errors.nationality = match author.birth_place {
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

		// open_library_id
		self.errors.openlibrary_id = match author.open_library_id {
			Some(ref bp) => {
				if bp.is_empty() {
					String::new()
				} else if bp.len() < 7 {
					"EL Open library ID es de al menos 7 caracteres.".into()
				} else {
					String::new()
				}
			}
			None => String::new(),
		};

		// viaf_id
		self.errors.viaf_id = match author.viaf_id {
			Some(ref bp) => {
				if bp.is_empty() {
					String::new()
				} else if bp.len() < 6 {
					"EL VIAF ID es de al menos 6 caracteres.".into()
				} else {
					String::new()
				}
			}
			None => String::new(),
		};

		let ok = self.errors.err_name.is_empty()
			&& self.errors.err_lastname.is_empty()
			&& self.errors.nationality.is_empty()
			&& self.errors.date_of_birth.is_empty()
			&& self.errors.date_of_death.is_empty()
			&& self.errors.biography.is_empty()
			&& self.errors.openlibrary_id.is_empty()
			&& self.errors.viaf_id.is_empty();

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

	fn clean_fields(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
		self.biography.update(cx, |state, cx| {
			state.set_value("", window, cx);
		});
	}
}
