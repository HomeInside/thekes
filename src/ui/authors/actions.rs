use super::{AuthorViewEvent, Authors};
use crate::database::authors::{AUTHORS_LIST, AuthorsItem};
use crate::ui::main_window::ColorConfig;
use gpui_kit::Action;
use gpui_kit::AppContext;
use gpui_kit::IntoElement;
use gpui_kit::Rgba;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::component::{h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::rgb;
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};

#[derive(Clone, PartialEq)]
pub enum AuthorOperation {
    Favorite,
    Details,
}

#[derive(Action, Clone, PartialEq)]
#[action(namespace = view_book, no_json)]
pub struct AuthorAction {
    pub author_id: u64,
    pub author_op: AuthorOperation,
}

// métodos/acciones/eventos
impl Authors {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            color_config: ColorConfig::default(),
            input_search: cx.new(|cx| InputState::new(window, cx).placeholder("Buscar por nombre...")),
            all_authors: Self::find_authors(""),
            result_count: AUTHORS_LIST.len(),
        }
    }

    pub fn find_authors(query: &str) -> Vec<AuthorsItem> {
        let query = query.trim().to_lowercase();

        AUTHORS_LIST
            .iter()
            .filter(|author| {
                query.is_empty()
                    || author.name.to_lowercase().contains(&query)
                    || author.lastname.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    }

    pub fn click_btn_new_author(&mut self, cx: &mut Context<Self>) {
        println!("click_btn_new_author fn");
        println!("enviando evento ShowForm");
        cx.emit(AuthorViewEvent::ShowForm);
    } //click_btn_new_author

    pub fn click_btn_search(&mut self, cx: &mut Context<Self>) {
        println!("click_btn_search fn");
        let text_search = self.input_search.read(cx).value().to_string();
        println!("input_search: {}", text_search);

        self.all_authors = Self::find_authors(&text_search);
        self.result_count = self.all_authors.len();
        cx.notify();
    } //click_btn_search

    pub fn draw_authors(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_1()
            .children(self.all_authors.iter().map(|author| self.author_row(author)))
    }

    pub fn gen_monogram(author: &AuthorsItem) -> String {
        let first = author.name.chars().next().unwrap_or('?');
        let last = author.lastname.chars().next();

        match last {
            Some(last) => format!("{}{}", first.to_uppercase(), last.to_uppercase()),
            None => first.to_uppercase().to_string(),
        }
    }

    pub fn author_row(&self, author: &AuthorsItem) -> impl IntoElement {
        let author_id = author.id;
        let button_id = format!("author-menu-{}", author.id);
        let full_name = format!("{} {}", author.name, author.lastname);
        let initials = Self::gen_monogram(author);
        let accent = rgb(author.cover_color);

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
                            .separator()
                            .menu_with_icon(
                                "Detalles",
                                AssetIconName::BookText,
                                Box::new(AuthorAction {
                                    author_id,
                                    author_op: AuthorOperation::Details,
                                }),
                            )
                        }
                    }),
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

    // TO REMOVE
    fn author_metadata(author: &AuthorsItem) -> (&'static str, &'static str, &'static str) {
        match author.id {
            1 => ("Colombia", "1927 — 2014", "Realismo mágico"),
            2 => ("Reino Unido", "1882 — 1941", "Modernismo"),
            3 => ("Argentina", "1899 — 1986", "Cuento · Ensayo"),
            _ => ("Por determinar", "Fechas desconocidas", "Género no definido"),
        }
    }

    // TODO
    // acciones cuando ocurra el evento de favoritos/detalles
    pub fn show_details(&mut self, author_id: u64, _cx: &mut Context<Self>) {
        println!("show_details fn");
        println!("show_details author_id: {}", author_id);
    }

    pub fn click_author_action(&mut self, action: &AuthorAction, _window: &mut Window, cx: &mut Context<Self>) {
        // antes, notifica el estado actual
        cx.notify();

        match &action.author_op {
            AuthorOperation::Favorite => {
                self.show_details(action.author_id, cx);
            }
            AuthorOperation::Details => {
                self.show_details(action.author_id, cx);
            }
        }

        // despues, si los métodos modifican el estado, sin notificar
        //cx.notify();
    }
}
