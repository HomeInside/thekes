use super::{BookAction, BookOperation};
use crate::database::DatabaseService;
use crate::database::books::model::BooksModel;
use crate::database::books::service::BookService;
use crate::ui::main_window::ColorConfig;
use gpui_kit::IntoElement;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::h_flex;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum BookViewEvent {
    ShowForm,
    List,
    Details { info: Box<BooksModel> },
}
pub struct Books {
    color_config: ColorConfig,
    pub input_search: Entity<InputState>,
    //pub hovered_book: Option<&'static str>,
    //focus_handle: FocusHandle,
    pub(crate) book_list: Vec<BooksModel>,
    pub(crate) result_count: usize,
    pub(crate) book_service: BookService,
}

impl EventEmitter<BookViewEvent> for Books {}

impl Render for Books {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let compact = window.viewport_size().width < px(700.);

        div()
            // deben ir en la raiz para que puedan propagarse
            // los eventos en cualquier parte de la app
            //
            //establece donde está el contexto de foco de Books.
            //.track_focus(&self.focus_handle)
            //
            //registra los handlers que reciben BookAction.
            .on_action(cx.listener(Self::click_book_action))
            //
            .size_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            .gap_5()
            // Encabezado
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .when(compact, |this| this.flex_col().items_start())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(27.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Mi biblioteca"),
                            )
                            .child(div().text_sm().text_color(self.color_config.muted).child(format!(
                                "{} {}",
                                self.result_count,
                                if self.result_count == 1 { "libro" } else { "libros" }
                            ))),
                    )
                    .child(
                        Button::new("btn-add-book")
                            .primary()
                            .label("Nuevo")
                            .on_click(cx.listener(|this, _, _, cx| this.click_btn_new_book(cx))),
                    ),
            )
            // Barra de herramientas
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .gap_2()
                    .when(compact, |this| this.flex_col().items_stretch())
                    // Búsqueda
                    .child(
                        div()
                            .w_full()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .border_1()
                            .border_color(cx.theme().border)
                            .bg(cx.theme().background)
                            .child(Icon::new(AssetIconName::Search).text_color(self.color_config.muted))
                            .child(div().flex_1().min_w_0().child(Input::new(&self.input_search)))
                            .child(
                                h_flex().items_center().gap_2().child(
                                    Button::new("btn-search-book")
                                        .primary()
                                        .label("Buscar")
                                        .on_click(cx.listener(|this, _, _, cx| this.click_btn_search(cx))),
                                ),
                            ),
                    ),
            )
            // Catálogo
            .child(
                div()
                    .id("books-catalog")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .child(self.draw_books()),
            )
    }
} //impl Render for Books

// métodos
impl Books {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, db: Arc<DatabaseService>) -> Self {
        let book_service = BookService::new(db.clone());
        Self {
            color_config: ColorConfig::default(),
            input_search: cx.new(|cx| InputState::new(window, cx).placeholder("Buscar por título o autor...")),
            //hovered_book: None,
            //focus_handle: cx.focus_handle(),
            book_list: Vec::new(),
            result_count: 0,
            book_service,
        }
    }

    pub fn find_books(&self, query: &str) -> Vec<BooksModel> {
        let query = query.trim().to_lowercase();

        self.book_list
            .iter()
            .filter(|book| query.is_empty() || book.title.to_lowercase().contains(&query))
            .cloned()
            .collect()
    }

    fn draw_books(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_wrap()
            .items_start()
            .gap_4()
            .children(self.book_list.iter().map(|book| self.book_card(book)))
    }

    fn book_card(&self, book: &BooksModel) -> impl IntoElement {
        let book_id = book.id;
        let _book_title = book.title.clone();
        let button_id = format!("book-menu-{}", book.id);
        let buy = book.buy.clone();

        div()
            .id(format!("book-card-{}", book.title))
            .w(px(185.))
            .flex_shrink_0()
            .p_2()
            .rounded_xl()
            .border_1()
            .border_color(rgb(0xE5E5E5))
            .bg(rgb(0xFFFFFF))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0xF5F5F5)).border_color(rgb(0xA0A0A0)))
            .flex()
            .flex_col()
            .gap_2()
            // Portada
            .child(
                div()
                    .h(px(225.))
                    .w_full()
                    .rounded_lg()
                    //TODO
                    .bg(rgb(book.cover_color.unwrap_or(0x000000)))
                    //.bg(rgb(0xF5F5F5))
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
                            .child(book.title.clone()),
                    ),
            )
            // Información
            .child(
                h_flex()
                    .items_start()
                    .gap_1()
                    .child(
                        div().flex_1().min_w_0().flex().flex_col().gap_1().child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x777777))
                                .line_clamp(2)
                                .text_ellipsis()
                                //TO FIX es autor_id
                                //.child(book.author)
                                .child(book.synopsis.clone())
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x777777))
                                        .child(book.published_date.to_string()),
                                ),
                        ),
                    )
                    .child(
                        Button::new(button_id.clone())
                            .ghost()
                            .small()
                            .icon(AssetIconName::EllipsisVertical)
                            .tooltip("Más opciones")
                            .dropdown_menu({
                                move |menu, _window, _cx| {
                                    let buy = buy
                                        .clone()
                                        .unwrap_or_else(|| "https://www.amazon.com/amz-books/store".to_string());

                                    menu.menu_with_icon(
                                        "Detalles",
                                        AssetIconName::BookText,
                                        Box::new(BookAction {
                                            book_id,
                                            book_op: BookOperation::Details,
                                        }),
                                    )
                                    .separator()
                                    .menu_with_icon(
                                        "Favorito",
                                        IconName::Heart,
                                        Box::new(BookAction {
                                            book_id,
                                            book_op: BookOperation::Favorite,
                                        }),
                                    )
                                    .menu_with_icon(
                                        "Estoy leyendo",
                                        AssetIconName::BookOpenCheck,
                                        Box::new(BookAction {
                                            book_id,
                                            book_op: BookOperation::Reading,
                                        }),
                                    )
                                    .menu_with_icon(
                                        "Quiero leer",
                                        AssetIconName::ListTodo,
                                        Box::new(BookAction {
                                            book_id,
                                            book_op: BookOperation::WantToRead,
                                        }),
                                    )
                                    .menu_with_icon(
                                        "Finalizado",
                                        AssetIconName::CircleCheckBig,
                                        Box::new(BookAction {
                                            book_id,
                                            book_op: BookOperation::Finished,
                                        }),
                                    )
                                    .separator()
                                    .link("Comprar en línea", buy)
                                    .separator()
                                    .menu_with_icon(
                                        "Eliminar",
                                        AssetIconName::Trash,
                                        Box::new(BookAction {
                                            book_id,
                                            book_op: BookOperation::Delete,
                                        }),
                                    )
                                }
                            }),
                    ),
            )
    }
}
