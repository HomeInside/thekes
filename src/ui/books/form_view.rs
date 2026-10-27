use super::BookViewEvent;
use crate::database::DatabaseService;
use crate::database::authors::service::AuthorService;
use crate::database::books::service::BookService;
use crate::ui::main_window::ColorConfig;
use chrono::{Datelike, NaiveDate};
use gpui_kit::IntoElement;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::form::field;
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, IconName, h_flex, v_flex};
use gpui_kit::component::{
    calendar::{self},
    date_picker::{DatePicker, DatePickerState, DateRangePreset},
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct FormBookErrors {
    pub(crate) err_title: String,
    pub(crate) err_author_id: String,
    pub(crate) err_synopsis: String,
    pub(crate) err_lang: String,
    pub(crate) err_page_count: String,
    pub(crate) err_format_type: String,
    pub(crate) err_genres: String,
    pub(crate) err_subjects: String,
    pub(crate) err_published_date: String,
    pub(crate) err_publisher: String,
    pub(crate) err_open_library_id: String,
    pub(crate) err_isbn: String,
    pub(crate) err_buy: String,
    pub(crate) err_cover_color: String,
}

pub struct BookFormView {
    pub(crate) color_config: ColorConfig,
    pub(crate) title: Entity<InputState>,
    pub(crate) synopsis: Entity<TextareaState>,
    pub(crate) author_service: AuthorService,
    pub(crate) book_service: BookService,
    pub(crate) errors: FormBookErrors,
}
impl EventEmitter<BookViewEvent> for BookFormView {}

impl BookFormView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, db: Arc<DatabaseService>) -> Self {
        let author_service = AuthorService::new(db.clone());
        let book_service = BookService::new(db.clone());
        Self {
            color_config: ColorConfig::default(),
            title: cx.new(|cx| InputState::new(window, cx).placeholder("book title")),
            synopsis: cx.new(|cx| TextareaState::new(window, cx).rows(3).placeholder("synopsis...")),
            author_service,
            book_service,
            errors: Default::default(),
        }
    }
}
impl Render for BookFormView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .gap_4()
            //
            // Cabecera
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(27.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Nuevo Libro"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(self.color_config.muted)
                                    .child("agrega un libro a tu biblioteca"),
                            ),
                    )
                    .child(
                        Button::new("book-form-back")
                            .ghost()
                            .label("Volver")
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.emit(BookViewEvent::List);
                            })),
                    ),
            )
        //
    }
}
