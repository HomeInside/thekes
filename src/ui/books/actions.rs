use super::{BookAction, BookOperation, Books};
use gpui_kit::{Context, Window};

//acciones/eventos
impl Books {
    pub fn click_search_filters(&mut self, cx: &mut Context<Self>) {
        println!("click_search_filters fn");
        let text = self.input_search.read(cx).value().to_string();
        println!("input_search: {}", text);
        cx.notify();
    }
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

    pub fn click_book_action(&mut self, action: &BookAction, _window: &mut Window, cx: &mut Context<Self>) {
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
        }

        // despues, si los métodos modifican el estado, sin notificar
        //cx.notify();
    }
}
