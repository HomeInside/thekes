use super::BookViewEvent;
use crate::database::DatabaseService;
use crate::database::authors::service::AuthorService;
use crate::database::books::service::BookService;
use crate::ui::main_window::ColorConfig;
use chrono::{Datelike, NaiveDate};
use gpui_kit::IntoElement;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::IndexPath;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::form::field;
use gpui_kit::component::input::NumberInput;
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::select::{SearchableVec, Select, SelectState};
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
    pub(crate) author_id: Entity<SelectState<SearchableVec<String>>>,
    pub(crate) synopsis: Entity<TextareaState>,
    pub(crate) lang: Entity<InputState>,
    pub(crate) page_count: Entity<InputState>,
    pub(crate) format_type: Entity<InputState>,
    pub(crate) genres: Entity<InputState>,
    pub(crate) subjects: Entity<InputState>,
    pub(crate) published_date: Entity<DatePickerState>,
    pub(crate) publisher: Entity<InputState>,
    pub(crate) open_library_id: Entity<InputState>,
    pub(crate) isbn: Entity<InputState>,
    pub(crate) buy: Entity<InputState>,
    pub(crate) cover_color: Entity<InputState>,
    pub(crate) author_service: AuthorService,
    pub(crate) book_service: BookService,
    pub(crate) errors: FormBookErrors,
}
impl EventEmitter<BookViewEvent> for BookFormView {}

impl BookFormView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, db: Arc<DatabaseService>) -> Self {
        let author_service = AuthorService::new(db.clone());
        let book_service = BookService::new(db.clone());

        let current_date = chrono::Local::now();

        let published_date = cx.new(|cx| {
            let current_year = current_date.year();
            let current_month = current_date.month();
            let current_day = current_date.day();
            let min_year = 0; // año 0 d.C.

            // 0000-01-01
            let min_date = NaiveDate::from_ymd_opt(min_year, 1, 1).unwrap();
            // hoy
            let max_date = NaiveDate::from_ymd_opt(current_year, current_month, current_day).unwrap();

            let mut picker = DatePickerState::new(window, cx)
                .date_format("%Y-%m-%d")
                .disabled_matcher(calendar::Matcher::custom(move |d| *d < min_date || *d > max_date));

            picker.set_year_range((min_year, current_year + 1), cx);
            picker
        });

        //let author_list: Vec<String> = if let Some(list) = authors { list } else { vec![] };
        let author_list: Vec<String> = vec!["Select author".into(), "Arthur".into(), "Conan".into(), "Doyle".into()];

        // si estamos editando y `author_id` existe
        /*
        let default_index = author_id.and_then(|id| {
            // los id del select comienzan en 0
            let pos = (id - 1) as usize;
            (pos < author_list.len()).then_some(IndexPath::default().row(pos))
        });
        */
        let default_index = Some(IndexPath::default().row(0));

        Self {
            color_config: ColorConfig::default(),
            title: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. The Adventures of Sherlock Holmes")),
            author_id: cx.new(|cx| SelectState::new(SearchableVec::new(author_list), default_index, window, cx)),
            synopsis: cx.new(|cx| TextareaState::new(window, cx).rows(4).placeholder("Synopsis...")),
            lang: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. English, Spanish")),
            page_count: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Ej. 42")
                    .min(1.)
                    .max(1500.)
                    .default_value("1")
            }),
            format_type: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. physical_book, ebook")),
            genres: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. Mistery, Sci-Fi, Crime")),
            subjects: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. detective, London")),
            published_date,
            publisher: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. A. L. Burt Company")),
            open_library_id: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. OL47078012M")),
            isbn: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. 97896...")),
            buy: cx.new(|cx| InputState::new(window, cx).placeholder("https://my-fav-store.example")),
            cover_color: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. 0xF4D5C2")),
            author_service,
            book_service,
            errors: Default::default(),
        }
    }

    fn input_field(label: &'static str, input: &Entity<InputState>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(label))
            .child(Input::new(input))
    }
    fn section(
        title: &'static str,
        description: &'static str,
        icon: IconName,
        header_action: impl IntoElement,
        content: impl IntoElement,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_4()
            .p_4()
            .rounded_xl()
            .border_1()
            /*
            .border_color(rgb(0xE8D6C8))
            .bg(rgb(0xFAF7F2))
            .hover(|style| style.bg(rgb(0xF8F3EC)).border_color(rgb(0xE0D3C5)))
            */
            .border_color(rgb(0xE5E2DC))
            .bg(rgb(0xFDFCFA))
            .hover(|style| style.bg(rgb(0xFAF7F2)).border_color(rgb(0xE8D6C8)))
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .size(px(40.))
                            .flex_shrink_0()
                            .rounded_lg()
                            .bg(rgb(0xF4D5C2))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Icon::new(icon).text_color(rgb(0xB94B1B))),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_1()
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                            .child(div().text_sm().text_color(rgb(0x85837D)).child(description)),
                    )
                    .child(header_action),
            )
            .child(content)
    }
}

impl Render for BookFormView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .gap_4()
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
                                    .child("Agrega un libro a tu biblioteca"),
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
            // contenido principal
            .child(
                v_flex()
                    .id("book-form-content")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .gap_4()
                    .overflow_y_scrollbar()
                    // información principal
                    .child(Self::section(
                        "Información principal",
                        "Título, autor y sinopsis del libro",
                        IconName::BookOpen,
                        div(),
                        v_flex()
                            .gap_4()
                            .child(
                                field()
                                    .label("Título")
                                    .required(true)
                                    .description_fn({
                                        let msg = self.errors.err_title.clone();
                                        move |_, cx| {
                                            if msg.is_empty() {
                                                div()
                                            } else {
                                                div().text_color(cx.theme().danger).child(msg.clone())
                                            }
                                        }
                                    })
                                    .child(Input::new(&self.title)),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        div().flex_1().min_w_0().child(
                                            field()
                                                .label("Autor")
                                                .required(true)
                                                .description_fn({
                                                    let msg = self.errors.err_author_id.clone();
                                                    move |_, cx| {
                                                        if msg.is_empty() {
                                                            div()
                                                        } else {
                                                            div().text_color(cx.theme().danger).child(msg.clone())
                                                        }
                                                    }
                                                })
                                                .child(Select::new(&self.author_id).placeholder("Autores")),
                                        ),
                                    )
                                    .child(
                                        div().flex_1().min_w_0().child(
                                            field()
                                                .label("Idioma")
                                                .required(true)
                                                .description_fn({
                                                    let msg = self.errors.err_lang.clone();
                                                    move |_, cx| {
                                                        if msg.is_empty() {
                                                            div()
                                                        } else {
                                                            div().text_color(cx.theme().danger).child(msg.clone())
                                                        }
                                                    }
                                                })
                                                .child(Input::new(&self.lang)),
                                        ),
                                    ),
                            )
                            .child(
                                field()
                                    .label("Sinopsis")
                                    .required(true)
                                    .description_fn({
                                        let msg = self.errors.err_synopsis.clone();
                                        move |_, cx| {
                                            if msg.is_empty() {
                                                div()
                                            } else {
                                                div().text_color(cx.theme().danger).child(msg.clone())
                                            }
                                        }
                                    })
                                    .child(Textarea::new(&self.synopsis).bordered(true).h(px(160.))),
                            ),
                    ))
                    // publicación y clasificación
                    .child(Self::section(
                        "Publicación y clasificación",
                        "Datos editoriales, formato y géneros",
                        IconName::BookOpen,
                        div(),
                        v_flex()
                            .gap_4()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    //
                                    .child(
                                        field()
                                            .label("Fecha de publicación")
                                            .required(true)
                                            .description_fn({
                                                let msg = self.errors.err_published_date.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(
                                                DatePicker::new(&self.published_date)
                                                    .placeholder("Select a date...")
                                                    .cleanable(true),
                                            ),
                                    )
                                    .child(
                                        field()
                                            .label("Editorial")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_publisher.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.publisher)),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        div().flex_1().min_w_0().child(
                                            field()
                                                .label("Páginas")
                                                .required(true)
                                                .description_fn({
                                                    let msg = self.errors.err_page_count.clone();
                                                    move |_, cx| {
                                                        if msg.is_empty() {
                                                            div()
                                                        } else {
                                                            div().text_color(cx.theme().danger).child(msg.clone())
                                                        }
                                                    }
                                                })
                                                .child(NumberInput::new(&self.page_count)),
                                        ),
                                    )
                                    .child(
                                        field()
                                            .label("Formato")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_format_type.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.format_type)),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        field()
                                            .label("Géneros")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_genres.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.genres)),
                                    )
                                    .child(
                                        field()
                                            .label("Subjects")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_subjects.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.subjects)),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x85837D))
                                    .child("Géneros y subjects: separa varios valores con comas."),
                            ),
                    ))
                    // identificadores externos
                    .child(Self::section(
                        "Referencias y presentación",
                        "Identificadores externos y color de portada",
                        IconName::ExternalLink,
                        div(),
                        v_flex()
                            .gap_4()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        field()
                                            .label("Open Library ID")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_open_library_id.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.open_library_id)),
                                    )
                                    .child(
                                        field()
                                            .label("ISBN")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_isbn.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.isbn)),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        field()
                                            .label("URL de compra")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_buy.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.buy)),
                                    )
                                    .child(
                                        field()
                                            .label("Color de portada")
                                            .required(false)
                                            .description_fn({
                                                let msg = self.errors.err_cover_color.clone();
                                                move |_, cx| {
                                                    if msg.is_empty() {
                                                        div()
                                                    } else {
                                                        div().text_color(cx.theme().danger).child(msg.clone())
                                                    }
                                                }
                                            })
                                            .child(Input::new(&self.cover_color)), /*.child(
                                                                                       div()
                                                                                           .text_xs()
                                                                                           .text_color(rgb(0x85837D))
                                                                                           .child("Por ahora hex, ej. 0xF4D5C2"),
                                                                                   ),*/
                                    ),
                            ),
                    )),
            )
            // botones
            .child(
                h_flex()
                    .justify_end()
                    .items_center()
                    .gap_3()
                    .pt_2()
                    .child(
                        Button::new("book-form-cancel")
                            .ghost()
                            .secondary()
                            .label("Cancelar")
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.emit(BookViewEvent::List);
                            })),
                    )
                    .child(
                        Button::new("btn-book-form-save")
                            .primary()
                            .label("Crear")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.click_btn_form_save(window, cx);
                            })),
                    ),
            )
    }
}
