use super::{AuthorViewEvent, Authors};
use crate::database::DatabaseService;
use crate::database::authors::model::AuthorModel;
use crate::database::authors::service::AuthorService;
use crate::database::books::service::BookService;
use crate::ui::dialogs;
use crate::ui::main_window::ColorConfig;
use gpui_kit::component::WindowExt;
use gpui_kit::component::input::InputState;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{Action, AppContext};
use gpui_kit::{Context, Window};
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub enum AuthorOperation {
    Favorite,
    Details,
    Delete,
}

#[derive(Action, Clone, PartialEq)]
#[action(namespace = view_book, no_json)]
pub struct AuthorAction {
    pub author_id: i32,
    pub author_op: AuthorOperation,
}

// métodos/acciones/eventos
impl Authors {
    //pub fn new(window: &mut Window, cx: &mut Context<Self>, author_service: AuthorService) -> Self {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, db: Arc<DatabaseService>) -> Self {
        //let author_service = AuthorService::new((*db).clone());
        let author_service = AuthorService::new(db.clone());
        let book_service = BookService::new(db.clone());

        Self {
            color_config: ColorConfig::default(),
            input_search: cx.new(|cx| InputState::new(window, cx).placeholder("Buscar por nombre...")),
            authors_list: Vec::new(),
            result_count: 0,
            author_service,
            book_service,
        }
    }

    // TO FIX
    // GPUI-Kit todavía no tiene inicializado el estado de
    // notificaciones cuando se llama `load_authors()` desde
    // `Authors::new()` ó `MainWindow::new()`, por lo que
    // no se puede usar `window: &mut Window` para usar las
    // notificaciones, hay que buscar otra solución
    pub fn load_authors(&mut self, cx: &mut Context<Self>) {
        println!("Authors::load_authors fn");
        match self.author_service.get_all() {
            Ok(list) => {
                println!("Autores encontrados: {}", list.len());
                self.result_count = list.len();
                self.authors_list = list;
                // organizar la ui aqui
                //window.push_notification((NotificationType::Success, "autores cargados"), cx);
            }
            Err(error) => {
                eprintln!("Error al consultar autores: {error}");
                //window.push_notification((NotificationType::Error, "Error al consultar autores"), cx);
            }
        }
        cx.notify();
    }

    pub fn find_authors(&self, query: &str) -> Vec<AuthorModel> {
        let query = query.trim().to_lowercase();

        self.authors_list
            .iter()
            .filter(|author| {
                query.is_empty()
                    || author.name.to_lowercase().contains(&query)
                    || author.last_name.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    }

    pub fn click_btn_new_author(&mut self, cx: &mut Context<Self>) {
        println!("click_btn_new_author fn");
        println!("enviando evento ShowForm");
        cx.emit(AuthorViewEvent::ShowForm);
    } //click_btn_new_author

    pub fn click_btn_search(&mut self, cx: &mut Context<Self>) {
        println!("click_btn_search fn");
        let text_search = self.input_search.read(cx).value().to_string();
        println!("input_search: {}", text_search);

        self.authors_list = self.find_authors(&text_search);
        self.result_count = self.authors_list.len();
        cx.notify();
    } //click_btn_search

    pub fn gen_monogram(author: &AuthorModel) -> String {
        let first = author.name.chars().next().unwrap_or('?');
        let last = author.last_name.chars().next();

        match last {
            Some(last) => format!("{}{}", first.to_uppercase(), last.to_uppercase()),
            None => first.to_uppercase().to_string(),
        }
    }

    //READY
    pub fn author_metadata(author: &AuthorModel) -> (String, String, String) {
        let birth_place = author
            .birth_place
            .clone()
            .unwrap_or_else(|| "Lugar desconocido".to_string());

        let lifespan = match &author.date_of_death {
            Some(death_year) => format!("{} — {}", author.date_of_birth, death_year),
            None => author.date_of_birth.to_string(),
        };

        (birth_place, lifespan, author.genres.clone())
    }

    //READY
    pub fn click_author_action(&mut self, action: &AuthorAction, window: &mut Window, cx: &mut Context<Self>) {
        // antes, notifica el estado actual
        cx.notify();

        match &action.author_op {
            AuthorOperation::Favorite => {
                self.add_fav(action.author_id, cx);
            }
            AuthorOperation::Details => {
                self.show_details(action.author_id, window, cx);
            }
            AuthorOperation::Delete => {
                self.delete_item(action.author_id, window, cx);
            }
        }

        // despues, si los métodos modifican el estado, sin notificar
        //cx.notify();
    }

    // TODO
    pub fn add_fav(&mut self, author_id: i32, _cx: &mut Context<Self>) {
        println!("add_fav author_id: {}", author_id);
    }

    //READY
    /// consulta un autor por su Id, y envía el
    /// evento `ShowDetails`, si existe el registro
    pub fn show_details(&mut self, author_id: i32, window: &mut Window, cx: &mut Context<Self>) {
        println!("show_details author_id: {}", author_id);

        match self.author_service.get_one(author_id) {
            //TO CHECK
            Ok(Some(author)) => match self.book_service.by_author(author_id) {
                Ok(list) => {
                    println!("enviando evento ShowDetails");
                    cx.emit(AuthorViewEvent::Details {
                        info: Box::new(author),
                        books: list,
                    });
                }
                Err(error) => {
                    let msg = format!("Ocurrió un error, al consultar libros por Id de autor: {}", error);
                    eprintln!("{}", msg);

                    window.push_notification((NotificationType::Error, msg), cx);
                }
            },
            Ok(None) => {
                let msg = format!("no se encontró el autor con ID: {}", author_id);
                eprintln!("{}", msg);

                window.push_notification((NotificationType::Warning, msg), cx);
            }
            Err(error) => {
                let msg = format!("Ocurrió un error, al consultar por Id de autor: {}", error);
                eprintln!("{}", msg);

                window.push_notification((NotificationType::Error, msg), cx);
            }
        }
        cx.notify();
    }

    //READY
    pub fn delete_item(&mut self, author_id: i32, window: &mut Window, cx: &mut Context<Self>) {
        println!("delete_item author_id: {}", author_id);

        dialogs::confirm(
            "Delete Author",
            "Are you sure you want to delete this author?",
            window,
            cx,
            move |author, window, cx| {
                println!("eliminando autor...");

                // TO CHECK
                match author.author_service.remove_author(author_id) {
                    Ok(_result) => {
                        window.push_notification((NotificationType::Success, "Author deleted"), cx);
                        author.load_authors(cx);
                    }
                    Err(error) => {
                        let msg = format!("Ocurrió un error, al eliminar el autor: {}", error);
                        eprintln!("{}", msg);
                        window.push_notification((NotificationType::Error, msg), cx);
                    }
                };
                cx.notify();
            },
        );
    } //delete_item
}
