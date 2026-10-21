//use crate::database::authors::AuthorsItem;
use crate::ui::main_window::ColorConfig;
//use gpui_kit::AppContext;
use crate::database::authors::model::AuthorModel;
use crate::database::authors::service::AuthorService;
use crate::database::books::model::BooksModel;
use crate::database::books::service::BookService;
use gpui_kit::IntoElement;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::Icon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::h_flex;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};

#[derive(Debug, Clone)]
pub enum AuthorViewEvent {
    ShowForm,
    ShowList,
    ShowDetails { info: AuthorModel, books: Vec<BooksModel> },
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
