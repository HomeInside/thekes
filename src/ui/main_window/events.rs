use super::MainWindow;
use crate::ui::authors::AuthorViewEvent;
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
        _window: &mut Window,
        cx: &mut Context<Self>,
        source: &'static str,
    ) -> Subscription
    where
        T: EventEmitter<AuthorViewEvent> + 'static,
    {
        /*
        cx.subscribe_in(table, window, |this, _table, event: &BooksEvent, window, cx| {
            this.handle_subs(event, window, cx);
        })
        */

        cx.subscribe(entity, move |this, _entity, event: &AuthorViewEvent, cx| {
            println!("{source} event {:?}", event);
            this.handle_subs(event, cx);
        })
    }

    pub fn handle_subs(&mut self, event: &AuthorViewEvent, cx: &mut Context<Self>) {
        match event {
            AuthorViewEvent::ShowForm => {
                self.show_author_form = true;
                self.show_author_details = false;
            }
            AuthorViewEvent::List => {
                self.show_author_form = false;
                self.show_author_details = false;
            }
            AuthorViewEvent::Details { info, books } => {
                self.show_author_form = false;
                self.show_author_details = true;
                //2.
                println!("handle_subs ShowDetails");
                self.author_details.update(cx, |author_detail, cx| {
                    author_detail.make_author_data(info, cx);
                    author_detail.make_books_data(books, cx);
                });
            }
        }
        cx.notify();
    }
}
