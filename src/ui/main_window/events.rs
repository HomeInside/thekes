use super::MainWindow;
use crate::ui::authors::AuthorViewEvent;
use crate::ui::books::BookViewEvent;
use gpui_kit::*;
use gpui_kit::{Context, Window};

impl MainWindow {
    /// Registra una suscripción a los eventos de navegación de las
    /// vistas del módulo de autores.
    /// El tipo de la entidad debe implementar [`EventEmitter<AuthorViewEvent>`],
    /// Permite reutilizar la misma lógica de suscripción para diferentes entidades
    /// que emiten eventos de tipo [`AuthorViewEvent`].
    ///
    /// Cuando se emite un evento, este es manejado por [`MainWindow::handle_subs`],
    /// que actualiza el estado de navegación y notifica los cambios a GPUI.
    ///
    pub fn subscribe_author_events<T>(
        entity: &Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
        source: &'static str,
    ) -> Subscription
    where
        T: EventEmitter<AuthorViewEvent> + 'static,
    {
        cx.subscribe_in(
            entity,
            window,
            move |this, _entity, event: &AuthorViewEvent, window, cx| {
                println!("{source} event {:?}", event);
                this.handle_subs(event, window, cx);
            },
        )
    }
    pub fn subscribe_books_events<T>(
        entity: &Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
        source: &'static str,
    ) -> Subscription
    where
        T: EventEmitter<BookViewEvent> + 'static,
    {
        cx.subscribe_in(
            entity,
            window,
            move |this, _entity, event: &BookViewEvent, window, cx| {
                println!("{source} event {:?}", event);
                this.handle_subs_books(event, window, cx);
            },
        )
    }

    // TO FIX rename to `handle_subs_authors`
    pub fn handle_subs(&mut self, event: &AuthorViewEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            AuthorViewEvent::ShowForm => {
                self.author_form.update(cx, |author_detail, cx| {
                    println!("aqui limpia los campos del showform");
                    author_detail.clean_fields(window, cx);
                });
                self.show_author_form = true;
                self.show_author_details = false;
            }
            AuthorViewEvent::List => {
                self.authors.update(cx, |authors, cx| {
                    println!("handle_subs AuthorViewEvent::List llamado a load_authors");
                    authors.load_authors(cx);
                });
                self.show_author_form = false;
                self.show_author_details = false;
            }
            AuthorViewEvent::Details { info, books } => {
                self.show_author_form = false;
                self.show_author_details = true;
                //2.
                println!("handle_subs AuthorViewEvent::Details");
                self.author_details.update(cx, |author_detail, cx| {
                    author_detail.make_author_data(info, cx);
                    author_detail.make_books_data(books, cx);
                });
            }
        }
        cx.notify();
    }

    pub fn handle_subs_books(&mut self, event: &BookViewEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            BookViewEvent::ShowForm => {
                self.book_form.update(cx, |book_detail, cx| {
                    println!("aqui limpia los campos del book showform");
                    book_detail.clean_fields(window, cx);
                });
                self.show_book_form = true;
                self.show_books_details = false;
            }
            BookViewEvent::List => {
                self.books.update(cx, |_authors, _cx| {
                    println!("handle_subs_books BookViewEvent::List llamado a load_books");
                    //authors.load_books(cx);
                });
                self.show_book_form = false;
                self.show_books_details = false;
            }
            BookViewEvent::Details { info } => {
                self.show_book_form = false;
                self.show_books_details = true;
                //2.
                println!("handle_subs_books BookViewEvent::Details");
                self.books_details.update(cx, |books_details, cx| {
                    println!("llamando a un evento de make_book_data, books_details");
                    books_details.make_book_data(info, cx);
                });
            }
        }
        cx.notify();
    }
}
