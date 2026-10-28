use super::BookViewEvent;
use crate::database::DatabaseService;
use crate::database::authors::model::AuthorModel;
use crate::database::authors::service::AuthorService;
use crate::database::books::model::BooksModel;
use crate::ui::authors;
use crate::ui::main_window::ColorConfig;
use crate::utils::date_utils;
use gpui_kit::IntoElement;
use gpui_kit::base::Link;
use gpui_kit::component::WindowExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::notification::NotificationType;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, IconName, form};
use gpui_kit::component::{h_flex, v_flex};
use gpui_kit::*;
use gpui_kit::{Context, Render, Styled, Window};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct BookDetailModel {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) author_id: String,
    pub(crate) author_names: String,
    pub(crate) synopsis: String,
    pub(crate) lang: String,
    pub(crate) cover_color: u32,
    pub(crate) page_count: String,
    pub(crate) format_type: String,
    pub(crate) genres: Option<String>,
    pub(crate) published_date: String,
    pub(crate) publisher: String,
    pub(crate) subjects: Option<String>,
    pub(crate) ol_id: Option<String>,
    pub(crate) viaf_id: Option<String>,
    pub(crate) isbn: Option<String>,
    pub(crate) buy: Option<String>,
    pub(crate) created_at: String,
}

#[allow(dead_code)]
pub struct BookDetailsView {
    pub(crate) color_config: ColorConfig,
    pub(crate) book_detail: BookDetailModel,
    pub(crate) author: AuthorModel,
    pub(crate) author_detail_service: AuthorService,
}

impl EventEmitter<BookViewEvent> for BookDetailsView {}

impl BookDetailsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, db: Arc<DatabaseService>) -> Self {
        let author_detail_service = AuthorService::new(db.clone());
        Self {
            color_config: ColorConfig::default(),
            book_detail: BookDetailModel::default(),
            author: AuthorModel::default(),
            author_detail_service,
        }
    }

    //
    fn section(
        title: &'static str,
        description: &'static str,
        icon: IconName,
        content: impl IntoElement,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_4()
            .p_4()
            .rounded_xl()
            .border_1()
            .border_color(rgb(0xE5E2DC))
            .bg(rgb(0xFDFCFA))
            .child(
                h_flex()
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
                    ),
            )
            .child(content)
    }

    //
    fn meta_link(id: &'static str, value: Option<String>) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .items_center()
            .gap_4()
            .py_1()
            .child(div().text_xs().text_color(rgb(0x85837D)).child("Comprar en línea"))
            .child(match value {
                Some(url) if !url.trim().is_empty() => Link::new(id)
                    .href(url)
                    .cursor_pointer()
                    .open_with(|href, _, _, cx| cx.open_url(href))
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xB94B1B))
                                    .child("sitio web externo"),
                            )
                            .child(Icon::new(IconName::ExternalLink).size_3().text_color(rgb(0xB94B1B))),
                    )
                    .into_any_element(),
                _ => div()
                    .text_sm()
                    .text_color(rgb(0x85837D))
                    .child("No vinculado")
                    .into_any_element(),
            })
    }

    //
    fn kv_row(label: &'static str, value: String) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .items_center()
            .gap_4()
            .py_1()
            .border_b_1()
            .border_color(rgb(0xF0EDE8))
            .child(div().text_xs().text_color(rgb(0x85837D)).child(label))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x34352F))
                    .text_ellipsis()
                    .child(value),
            )
    }
    //
    fn ref_link(
        id: &'static str,
        label: &'static str,
        value: Option<String>,
        base_url: &'static str,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .items_center()
            .gap_4()
            .py_1()
            .border_b_1()
            .border_color(rgb(0xF0EDE8))
            .child(div().text_xs().text_color(rgb(0x85837D)).child(label))
            .child(match value {
                Some(v) if !v.trim().is_empty() => {
                    let url = format!("{base_url}{v}");
                    Link::new(id)
                        .href(url)
                        .cursor_pointer()
                        .open_with(|href, _, _, cx| cx.open_url(href))
                        .child(
                            h_flex()
                                .gap_1()
                                .items_center()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0x34352F))
                                        .child(v),
                                )
                                .child(Icon::new(IconName::ExternalLink).size_3()),
                        )
                        .into_any_element()
                }
                _ => div()
                    .text_sm()
                    .text_color(rgb(0x85837D))
                    .child("No vinculado")
                    .into_any_element(),
            })
    }
    //
    fn make_tags(&self, tag_str: Option<String>) -> impl IntoElement {
        let items: Vec<String> = if let Some(data) = tag_str {
            data.split(',')
                .map(|g| g.trim().to_string())
                .filter(|g| !g.is_empty())
                .collect()
        } else {
            vec![]
        };
        if items.is_empty() {
            return h_flex()
                .flex_wrap()
                .gap_2()
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded_full()
                        .bg(rgb(0xF5EEE5))
                        .text_sm()
                        .text_color(rgb(0x79563E))
                        .child("no definido"),
                )
                .into_any_element();
        }

        h_flex()
            .flex_wrap()
            .gap_2()
            .children(items.into_iter().map(|g| {
                div()
                    .px_3()
                    .py_1()
                    .rounded_full()
                    .bg(rgb(0xF5EEE5))
                    .text_sm()
                    .text_color(rgb(0x79563E))
                    .child(g)
            }))
            .into_any_element()
    }

    /// obtener el nombre del autor
    pub fn get_author_data(&mut self, author_id: i32, window: &mut Window, cx: &mut Context<Self>) {
        println!("get_author_data fn",);
        match self.author_detail_service.get_one(author_id) {
            //TO CHECK
            Ok(Some(author)) => {
                self.book_detail.author_names = format!("{} {}", author.name, author.last_name);
            }
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
    }

    pub fn make_book_data(&mut self, book: &BooksModel, cx: &mut Context<Self>) {
        self.book_detail.id = book.id;
        self.book_detail.title = book.title.clone();
        // author_names en `get_author_data`
        self.book_detail.author_id = book.author_id.clone().to_string();
        self.book_detail.synopsis = book.synopsis.clone();
        self.book_detail.cover_color = book.cover_color.clone().unwrap_or(0x000000);
        self.book_detail.format_type = book.format_type.clone();
        //
        self.book_detail.published_date = book.published_date.clone().to_string();
        self.book_detail.publisher = book.publisher.clone().unwrap_or("-".into());
        self.book_detail.page_count = book.page_count.clone().to_string();
        self.book_detail.lang = book.lang.clone();
        //
        self.book_detail.subjects = book.subjects.clone();
        self.book_detail.genres = book.genres.clone();
        //
        self.book_detail.ol_id = book.open_library_id.clone();
        self.book_detail.viaf_id = None; //"No disponible".into();
        self.book_detail.isbn = book.isbn.clone();
        //self.book_detail.buy = None; //"No existe enlace de compra".into();
        self.book_detail.buy = book.buy.clone();
        //
        self.book_detail.created_at = date_utils::date_to_local_string(book.created_at.clone(), "en");
    }
}

impl Render for BookDetailsView {
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
                    .w_full()
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
                                    .child("Detalle del libro"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(self.color_config.muted)
                                    .child("Información del libro agregado"),
                            ),
                    )
                    .child(
                        Button::new("btn-book-details-back")
                            .ghost()
                            .label("Volver")
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.emit(BookViewEvent::List);
                            })),
                    ),
            )
            //
            .child(
                v_flex()
                    .id("book-details-content")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .gap_4()
                    .overflow_y_scrollbar()
                    //
                    .child(
                        h_flex()
                            .w_full()
                            .items_start()
                            .gap_4()
                            //
                            // Columna izquierda: portada + ficha
                            .child(
                                h_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .items_start()
                                    .gap_3()
                                    //
                                    // Portada
                                    .child(
                                        v_flex()
                                            .w(px(200.))
                                            .flex_shrink_0()
                                            .items_center()
                                            .gap_2()
                                            .p_2()
                                            .rounded_xl()
                                            .border_1()
                                            .border_color(rgb(0xE5E2DC))
                                            .bg(rgb(0xFDFCFA))
                                            .child(
                                                div()
                                                    .h(px(225.))
                                                    .w_full()
                                                    .rounded_lg()
                                                    .bg(rgb(self.book_detail.cover_color.clone()))
                                                    .flex()
                                                    .flex_col()
                                                    .justify_end()
                                                    .p_2()
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(
                                                        div()
                                                            .w_full()
                                                            .text_size(px(18.))
                                                            .font_weight(FontWeight::BOLD)
                                                            .line_clamp(3)
                                                            .text_ellipsis()
                                                            .child(self.book_detail.title.clone()),
                                                    ),
                                            ),
                                    )
                                    //
                                    .child(
                                        v_flex()
                                            .flex_1()
                                            .min_w_0()
                                            .min_h_0()
                                            .gap_3()
                                            .pt_1()
                                            .child(
                                                div()
                                                    .text_size(px(24.))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x34352F))
                                                    .text_ellipsis()
                                                    .child(self.book_detail.title.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x625F59))
                                                    .child(format!("Autor: {}", self.book_detail.author_names.clone())),
                                            )
                                            .child(
                                                v_flex()
                                                    .w_full()
                                                    .gap_0()
                                                    .child(Self::kv_row(
                                                        "Fecha de publicación",
                                                        self.book_detail.published_date.clone(),
                                                    ))
                                                    .child(Self::kv_row(
                                                        "Editorial",
                                                        self.book_detail.publisher.clone(),
                                                    ))
                                                    .child(Self::kv_row("Idioma", self.book_detail.lang.clone()))
                                                    .child(Self::kv_row("Páginas", self.book_detail.page_count.clone()))
                                                    .child(Self::kv_row(
                                                        "Formato",
                                                        self.book_detail.format_type.clone(),
                                                    ))
                                                    .child(Self::kv_row(
                                                        "Agregado",
                                                        self.book_detail.created_at.clone(),
                                                    )),
                                            ),
                                    ),
                            )
                            //
                            // Columna derecha: sinopsis + referencias
                            .child(
                                v_flex().flex_1().min_w_0().gap_4().child(Self::section(
                                    "Sinopsis",
                                    "Los aspectos más relevantes de este título",
                                    IconName::BookOpen,
                                    v_flex().gap_2().child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x45433F))
                                            .line_clamp(9)
                                            .text_ellipsis()
                                            .child(self.book_detail.synopsis.clone()),
                                    ),
                                )),
                            ),
                    )
                    //
                    .child(
                        h_flex()
                            .w_full()
                            .items_start()
                            .gap_4()
                            // Perfil literario (ancho completo, tags)
                            .child(Self::section(
                                "Perfil literario",
                                "Géneros y temas de este título",
                                IconName::User,
                                v_flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Géneros literarios"),
                                    )
                                    .child(self.make_tags(self.book_detail.genres.clone()))
                                    .child(div().text_sm().font_weight(FontWeight::MEDIUM).child("Temas"))
                                    //
                                    // TODO
                                    .child(self.make_tags(self.book_detail.subjects.clone())),
                            ))
                            //
                            .child(Self::section(
                                "Referencias",
                                "Identificadores y enlaces del libro",
                                IconName::ExternalLink,
                                v_flex()
                                    .w_full()
                                    .gap_0()
                                    .child(Self::ref_link(
                                        "book-open-library-link",
                                        "Open Library ID",
                                        self.book_detail.ol_id.clone(),
                                        "https://openlibrary.org/books/",
                                    ))
                                    .child(Self::ref_link(
                                        "book-isbn-link",
                                        "ISBN",
                                        self.book_detail.isbn.clone(),
                                        "https://openlibrary.org/search?isbn=",
                                    ))
                                    .child(Self::meta_link("book-buy-link", self.book_detail.buy.clone())),
                            )),
                    ),
            )
    }
}
