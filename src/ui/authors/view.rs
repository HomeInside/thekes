use super::actions::{AuthorAction, AuthorOperation};
use crate::database::authors::model::AuthorModel;
use crate::database::authors::service::AuthorService;
use crate::database::books::model::BooksModel;
use crate::database::books::service::BookService;
use crate::ui::main_window::ColorConfig;
use gpui_kit::IntoElement;
use gpui_kit::Rgba;
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
use gpui_kit::rgb;
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};

#[derive(Debug, Clone)]
pub enum AuthorViewEvent {
    ShowForm,
    List,
    Details {
        info: Box<AuthorModel>,
        books: Vec<BooksModel>,
    },
}

pub struct Authors {
    pub(crate) color_config: ColorConfig,
    pub(crate) input_search: Entity<InputState>,
    pub(crate) authors_list: Vec<AuthorModel>,
    pub(crate) result_count: usize,
    pub(crate) author_service: AuthorService,
    pub(crate) book_service: BookService,
}

impl EventEmitter<AuthorViewEvent> for Authors {}

impl Authors {
    pub fn draw_authors(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_1()
            .children(self.authors_list.iter().map(|author| self.author_row(author)))
    }

    pub fn author_row(&self, author: &AuthorModel) -> impl IntoElement {
        let author_id = author.id;
        let button_id = format!("author-menu-{}", author.id);
        let full_name = format!("{} {}", author.name, author.last_name);
        let initials = Self::gen_monogram(author);

        let cover_color = author.cover_color.unwrap_or(0xF4D5C2);
        let accent = rgb(cover_color);

        div()
            /*
            .hover(|style| style.bg(rgb(0xF5F3EE)).border_color(rgb(0x00000000)))
            */
            .id(format!("author-row-{}", author.id))
            .w_full()
            .flex()
            .items_center()
            .gap_3()
            .p_2()
            .rounded_lg()
            .border_1()
            .cursor_pointer()
            //.border_color(rgb(0xF5F3EE))
            //.hover(|style| style.bg(rgb(0xF0F0E9)).border_color(rgb(0x00000000)))
            .hover(|style| style.bg(rgb(0xF0F0E9)).border_color(rgb(0xE0E2D8)))
            //
            // Monograma de iniciales
            .child(
                div()
                    .size(px(46.))
                    .flex_shrink_0()
                    .rounded_lg()
                    .bg(accent)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(15.))
                            .font_weight(FontWeight::BOLD)
                            //.text_color(rgb(0x343B34))
                            .text_color(rgb(0x655448))
                            .child(initials),
                    ),
            )
            //
            .child({
                let (nationality, dates, genre) = Self::author_metadata(author);

                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    // Nombre
                    .child(
                        div()
                            .text_size(px(14.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x34352F))
                            .text_ellipsis()
                            .child(full_name),
                    )
                    // Nacionalidad y fechas
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_xs().text_color(rgb(0x82796F)).child(nationality))
                            .child(div().size(px(3.)).rounded_full().bg(rgb(0xD6B5A3)))
                            .child(div().text_xs().text_color(rgb(0x82796F)).child(dates)),
                    )
                    // Perfil literario
                    .child(div().text_xs().text_color(rgb(0xA1846F)).child(genre))
            })
            //
            // Menú de acciones
            .child(
                Button::new(button_id)
                    .ghost()
                    .small()
                    .icon(AssetIconName::EllipsisVertical)
                    .tooltip("Más opciones")
                    .dropdown_menu({
                        move |menu, _window, _cx| {
                            menu.menu_with_icon(
                                "Favorito",
                                IconName::Heart,
                                Box::new(AuthorAction {
                                    author_id,
                                    author_op: AuthorOperation::Favorite,
                                }),
                            )
                            .menu_with_icon(
                                "Detalles",
                                AssetIconName::Users,
                                Box::new(AuthorAction {
                                    author_id,
                                    author_op: AuthorOperation::Details,
                                }),
                            )
                            .separator()
                            .menu_with_icon(
                                "Eliminar",
                                AssetIconName::Trash,
                                Box::new(AuthorAction {
                                    author_id,
                                    author_op: AuthorOperation::Delete,
                                }),
                            )
                        }
                    }),
            )
    }

    // dashboard de autores
    pub fn authors_dashboard(&self, label: &'static str, value: String, accent: Rgba) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_1()
            .p_3()
            .rounded_lg()
            .border_1()
            .border_color(rgb(0xE3E0D8))
            .bg(rgb(0xF5F3EE))
            .child(div().text_xs().text_color(rgb(0x77776F)).child(label))
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(div().size(px(8.)).rounded_full().bg(accent))
                    .child(
                        div()
                            .text_size(px(22.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x292B29))
                            .child(value),
                    ),
            )
    }

    // Índice alfabético
    // TODO todavía no filtra al seleccionar una letra
    pub fn draw_alphabet_index(&self) -> impl IntoElement {
        let letters = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
            "V", "W", "X", "Y", "Z",
        ];

        div()
            .w_full()
            .flex()
            .flex_wrap()
            .items_center()
            .justify_center()
            .gap_1()
            .p_2()
            .rounded_lg()
            .bg(rgb(0xF5F3EE))
            .children(letters.into_iter().map(|letter| {
                div()
                    .min_w(px(25.))
                    .h(px(28.))
                    .px_1()
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x65675F))
                    .child(letter)
            }))
    }
}

impl Render for Authors {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let compact = window.viewport_size().width < px(700.);

        div()
            // deben ir en la raiz para que puedan propagarse
            // los eventos en cualquier parte de la app
            //registra los handlers que reciben AuthorAction.
            .on_action(cx.listener(Self::click_author_action))
            .size_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            //.gap_4()
            .gap_2()
            //
            // Cabecera
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(27.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Autores"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(self.color_config.muted)
                                    .child("Las personas detrás de tus historias favoritas."),
                            ),
                    )
                    //
                    //.child(AssetIconName::Users),
                    // TO FIX si se mueve de sitio
                    .child(
                        Button::new("btn-add-author")
                            .primary()
                            .label("Nuevo")
                            .on_click(cx.listener(|this, _, _, cx| this.click_btn_new_author(cx))),
                    ),
            )
            //
            // dashboard de autores
            .child(
                div()
                    .w_full()
                    .grid_cols(3)
                    .grid()
                    .gap_3()
                    .when(compact, |this| this.grid_cols(1))
                    .child(self.authors_dashboard("Autores", self.result_count.to_string(), rgb(0xC6D2B9))),
                //
                //TO FIX agregar mas info para las card
                //.child(self.authors_dashboard("Obras", "—".to_string(), rgb(0xC3CBE0)))
                //.child(self.authors_dashboard("Sin biografía", "—".to_string(), rgb(0xD6B5A3))),
            )
            //
            // botón nuevo
            // TO FIX si se mueve de sitio
            /*.child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .child("")
                    .child("")
                    .child(
                        Button::new("btn-add-author")
                            .primary()
                            //.icon(AssetIconName::Plus)
                            .label("Nuevo")
                            .on_click(cx.listener(|this, _, _, cx| this.click_btn_new_author(cx))),
                    ),
            )*/
            //
            // Búsqueda
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap_3()
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
                            Button::new("btn-search-author")
                                .primary()
                                .label("Buscar")
                                .on_click(cx.listener(|this, _, _, cx| this.click_btn_search(cx))),
                        ),
                    ),
            )
            //
            // mostrar Índice alfabético
            .child(self.draw_alphabet_index())
            //
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .pb_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x55594F))
                            .child("Índice de autores"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(self.color_config.muted)
                            //.child(format!("{} registros", self.result_count)),
                            .child(if self.result_count == 1 {
                                format!("{} registro", self.result_count)
                                //"registro"
                            } else {
                                //"registros"
                                format!("{} registros", self.result_count)
                            }),
                    ),
            )
            //
            // lista de autores
            .child(
                div()
                    .id("authors-catalog")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .child(self.draw_authors()),
            )
    }
}
