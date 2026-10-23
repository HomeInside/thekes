use super::Books;
use crate::ui::dialogs;
//use crate::ui::dialogs::ConfirmDialog;
use gpui_kit::Action;
use gpui_kit::component::WindowExt;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{Context, Window};

#[derive(Clone, PartialEq)]
pub enum BookOperation {
    Favorite,
    Reading,
    WantToRead,
    Finished,
    Details,
    Delete,
}

#[derive(Action, Clone, PartialEq)]
#[action(namespace = view_book, no_json)]
pub struct BookAction {
    pub book_id: u64,
    pub book_op: BookOperation,
}

//acciones/eventos
impl Books {
    // TO FIX
    // GPUI-Kit todavía no tiene inicializado el estado de
    // notificaciones cuando se llama `load_books()` desde
    // `Books::new()` ó `MainWindow::new()`, por lo que
    // no se puede usar `window: &mut Window` para usar las
    // notificaciones, hay que buscar otra solución
    pub fn load_books(&mut self, cx: &mut Context<Self>) {
        println!("Books::load_authors fn");
        match self.book_service.get_all() {
            Ok(list) => {
                println!("libros encontrados: {}", list.len());
                self.result_count = list.len();
                self.book_list = list;
                // organizar la ui aqui
                //window.push_notification((NotificationType::Success, "libros cargados"), cx);
            }
            Err(error) => {
                eprintln!("Error al consultar libros: {error}");
                //window.push_notification((NotificationType::Error, "Error al consultar libros"), cx);
            }
        }
        cx.notify();
    }

    pub fn click_btn_new_book(&mut self, _cx: &mut Context<Self>) {
        println!("click_btn_new_book fn");
    } //click_btn_new_book
    pub fn click_btn_search(&mut self, cx: &mut Context<Self>) {
        println!("click_btn_search fn");
        let text_search = self.input_search.read(cx).value().to_string();
        println!("input_search: {}", text_search);

        self.book_list = self.find_books(&text_search);
        self.result_count = self.book_list.len();
        cx.notify();
    } //click_btn_search

    pub fn toggle_favorite(&mut self, book_id: u64, _cx: &mut Context<Self>) {
        println!("toggle_favorite fn");
        println!("toggle_favorite book_id: {}", book_id);
    }
    pub fn set_reading(&mut self, book_id: u64, _cx: &mut Context<Self>) {
        println!("set_reading fn");
        println!("set_reading book_id: {}", book_id);
    }
    pub fn set_want_to_read(&mut self, book_id: u64, _cx: &mut Context<Self>) {
        println!("set_want_to_read fn");
        println!("set_want_to_read book_id: {}", book_id);
    }
    pub fn set_finished(&mut self, book_id: u64, _cx: &mut Context<Self>) {
        println!("set_finished fn");
        println!("set_finished book_id: {}", book_id);
    }

    pub fn show_details(&mut self, book_id: u64, _cx: &mut Context<Self>) {
        println!("show_details fn");
        println!("show_details book_id: {}", book_id);
    }

    pub fn delete_item(&mut self, book_id: u64, window: &mut Window, cx: &mut Context<Self>) {
        println!("delete_item book_id: {}", book_id);

        dialogs::confirm(
            "Delete Book",
            "Are you sure you want to delete this book?",
            window,
            cx,
            move |books, window, cx| {
                println!("eliminando libro...");

                // TO CHECK
                match books.book_service.remove_book(book_id as i32) {
                    Ok(_result) => {
                        window.push_notification((NotificationType::Success, "Book deleted"), cx);
                        books.load_books(cx);
                    }
                    Err(error) => {
                        let msg = format!("Ocurrió un error, al eliminar el libro: {}", error);
                        eprintln!("{}", msg);
                        window.push_notification((NotificationType::Error, msg), cx);
                    }
                };
                cx.notify();
            },
        );
    } //delete_item

    pub fn click_book_action(&mut self, action: &BookAction, window: &mut Window, cx: &mut Context<Self>) {
        // antes, notifica el estado actual
        cx.notify();

        match &action.book_op {
            BookOperation::Favorite => {
                self.toggle_favorite(action.book_id, cx);
            }
            BookOperation::Reading => {
                self.set_reading(action.book_id, cx);
            }
            BookOperation::WantToRead => {
                self.set_want_to_read(action.book_id, cx);
            }
            BookOperation::Finished => {
                self.set_finished(action.book_id, cx);
            }
            BookOperation::Details => {
                self.show_details(action.book_id, cx);
            }
            BookOperation::Delete => {
                self.delete_item(action.book_id, window, cx);
            }
        }

        // despues, si los métodos modifican el estado, sin notificar
        //cx.notify();
    }
}
