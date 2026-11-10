use super::Books;
use crate::ui::books::BookViewEvent;
use crate::ui::dialogs;
use gpui_kit::Action;
use gpui_kit::component::WindowExt;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{Context, Window};
use gpui_kit::{Rgba, rgb};

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
    pub book_id: i32,
    pub book_op: BookOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReadingStatus {
    None = 0,
    Reading = 1,
    WantToRead = 2,
    Finished = 3,
}

impl From<u32> for ReadingStatus {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Reading,
            2 => Self::WantToRead,
            3 => Self::Finished,
            _ => Self::None,
        }
    }
}

impl From<ReadingStatus> for i32 {
    fn from(value: ReadingStatus) -> Self {
        value as i32
    }
}

impl ReadingStatus {
    pub fn color(self) -> Rgba {
        match self {
            Self::Reading => rgb(0x3B82F6),
            Self::WantToRead => rgb(0xF59E0B),
            Self::Finished => rgb(0x22C55E),
            Self::None => rgb(0x808080),
        }
    }
    pub fn default(self) -> Rgba {
        rgb(0x808080)
    }
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

    pub fn click_btn_new_book(&mut self, cx: &mut Context<Self>) {
        println!("click_btn_new_book fn");
        println!("enviando evento BookViewEvent::ShowForm");
        cx.emit(BookViewEvent::ShowForm);
    } //click_btn_new_book

    pub fn click_btn_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("click_btn_search fn");
        let text_search = self.input_search.read(cx).value().to_string();

        let result = self.find_books(&text_search);
        match result {
            Ok(list) => {
                self.result_count = list.len();
                self.book_list = list;
            }
            Err(error) => {
                window.push_notification((NotificationType::Warning, error), cx);
            }
        }
        cx.notify();
    } //click_btn_search

    pub fn toggle_favorite(&mut self, book_id: i32, window: &mut Window, cx: &mut Context<Self>) {
        match self.book_service.mark_as_fav(book_id) {
            Ok(result) => {
                let msg = if result {
                    "¡Libro marcado como favorito!"
                } else {
                    "Libro eliminado de favoritos"
                };

                window.push_notification((NotificationType::Success, msg), cx);
            }
            Err(error) => {
                let msg = format!("Ocurrió un error, al actualizar el libro: {}", error);
                eprintln!("{}", msg);

                window.push_notification((NotificationType::Warning, msg), cx);
            }
        }
        cx.notify();
    }

    pub fn set_current_status(
        &mut self,
        book_id: i32,
        new_value: ReadingStatus,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.book_service.save_current_status(book_id, new_value.into()) {
            Ok(Some(result)) => {
                //
                let msg = match result {
                    1 => "¡Libro marcado como: actualmente leyendo!",
                    2 => "¡Libro marcado como: quiero leerlo!",
                    3 => "¡Libro marcado como: finalizado!",
                    _ => "¡Libro desmarcado!",
                };

                window.push_notification((NotificationType::Success, msg), cx);
            }
            Ok(None) => {
                let msg = format!("Ocurrió un error, al actualizar el libro");
                eprintln!("{}", msg);

                window.push_notification((NotificationType::Warning, msg), cx);
            }
            Err(error) => {
                let msg = format!("Ocurrió un error, al actualizar el libro: {}", error);
                eprintln!("{}", msg);

                window.push_notification((NotificationType::Error, msg), cx);
            }
        }
        cx.notify();
    }

    pub fn show_details(&mut self, book_id: i32, window: &mut Window, cx: &mut Context<Self>) {
        println!("show_details fn");
        println!("show_details book_id: {}", book_id);

        match self.book_service.get_one(book_id) {
            //TO CHECK
            Ok(Some(book)) => {
                println!("enviando evento book ShowDetails");
                cx.emit(BookViewEvent::Details { info: Box::new(book) });
            }
            Ok(None) => {
                let msg = format!("no se encontró el libro con ID: {}", book_id);
                eprintln!("{}", msg);

                window.push_notification((NotificationType::Warning, msg), cx);
            }
            Err(error) => {
                let msg = format!("Ocurrió un error, al consultar por Id de libro: {}", error);
                eprintln!("{}", msg);

                window.push_notification((NotificationType::Error, msg), cx);
            }
        }
        cx.notify();
    }

    pub fn delete_item(&mut self, book_id: i32, window: &mut Window, cx: &mut Context<Self>) {
        println!("delete_item book_id: {}", book_id);

        dialogs::confirm(
            "Delete Book",
            "Are you sure you want to delete this book?",
            window,
            cx,
            move |books, window, cx| {
                println!("eliminando libro...");

                // TO CHECK
                match books.book_service.remove_book(book_id) {
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
                self.toggle_favorite(action.book_id, window, cx);
            }
            BookOperation::Reading => {
                self.set_current_status(action.book_id, ReadingStatus::Reading, window, cx);
            }
            BookOperation::WantToRead => {
                self.set_current_status(action.book_id, ReadingStatus::WantToRead, window, cx);
            }
            BookOperation::Finished => {
                self.set_current_status(action.book_id, ReadingStatus::Finished, window, cx);
            }
            BookOperation::Details => {
                self.show_details(action.book_id, window, cx);
            }
            BookOperation::Delete => {
                self.delete_item(action.book_id, window, cx);
            }
        }
        self.load_books(cx);

        // despues, si los métodos modifican el estado, sin notificar
        cx.notify();
    }
}
