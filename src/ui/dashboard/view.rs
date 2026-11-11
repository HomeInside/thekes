use crate::database::DatabaseService;
use crate::database::general::service::GeneralService;
use crate::ui::main_window::ColorConfig;
use gpui_kit::IntoElement;
use gpui_kit::Styled;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::{Context, Render, Window};
use gpui_kit::{FontWeight, ParentElement, div, px, rgb};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct LibraryStats {
    pub(crate) authors: usize,
    pub(crate) books: usize,
    pub(crate) favs: usize,
    pub(crate) publ: usize,
    pub(crate) genre: usize,
    pub(crate) lang: usize,
    pub(crate) quote: String,
    //
    pub have_r_book: bool,
    pub rb_title: String,
    pub rb_published: String,
    pub rb_cover_color: u32,
    pub r_author: String,
}

pub struct Dashboard {
    pub(crate) dash_config: ColorConfig,
    pub(crate) library_info: LibraryStats,
    pub(crate) general_service: GeneralService,
}

impl Dashboard {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, db: Arc<DatabaseService>) -> Self {
        let general_service = GeneralService::new(db.clone());
        Self {
            dash_config: ColorConfig::default(),
            library_info: LibraryStats::default(),
            general_service,
        }
    }

    /// tarjetas de la dashboard
    pub fn dash_card(
        &self,
        icon: impl IntoElement,
        label: &'static str,
        value: String,
        description: &'static str,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .min_w(px(150.))
            .p_4()
            .rounded_lg()
            .border_1()
            .border_color(self.dash_config.border)
            .bg(rgb(0xffffff))
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .size(px(38.))
                    .rounded_lg()
                    .bg(rgb(0xF4D5C2))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon),
            )
            .child(
                div()
                    .text_size(px(26.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x292B29))
                    .child(value),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x34352F))
                            .child(label),
                    )
                    .child(div().text_xs().text_color(self.dash_config.muted).child(description)),
            )
    }

    /// cita del día
    pub fn quote_card(&self) -> impl IntoElement {
        div()
            .w_full()
            .p_5()
            .rounded_xl()
            .border_1()
            .border_color(rgb(0xE5E2DC))
            .bg(rgb(0xFDFCFA))
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(Icon::new(AssetIconName::NotebookPen).text_color(rgb(0xB94B1B)).size_5())
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x55594F))
                            .child("Cita del día"),
                    ),
            )
            .child(
                div()
                    .text_base()
                    .text_color(rgb(0x34352F))
                    .child(self.library_info.quote.clone()),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(self.dash_config.muted)
                    .child("— Thekes, in collaboration with your favorite authors"),
            )
    }

    /// libro recomendado
    pub fn recommendation_card(&self) -> impl IntoElement {
        let book_info = if self.library_info.have_r_book {
            div()
                .flex()
                .flex_col()
                .gap_1()
                .min_w_0()
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x292B29))
                        .text_ellipsis()
                        //.child("las aventuras de sherlck holmes"),
                        .child(self.library_info.rb_title.clone()),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x34352F))
                        .text_ellipsis()
                        //.child("Sir Arthur Conan Doyle"),
                        .child(self.library_info.r_author.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(self.dash_config.muted)
                        //.child("Publicado en 1920"),
                        .child(self.library_info.rb_published.clone()),
                )
        } else {
            div()
                .text_sm()
                .text_color(self.dash_config.muted)
                .child("Próximamente: un título aleatorio de tu colección.")
        };

        div()
            .w_full()
            .p_5()
            .rounded_xl()
            .border_1()
            .border_color(rgb(0xE5E2DC))
            .bg(rgb(0xFDFCFA))
            .flex()
            .flex_col()
            .gap_4()
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(Icon::new(AssetIconName::BookHeart).text_color(rgb(0xB94B1B)).size_5())
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x55594F))
                            .child("Recomendación"),
                    ),
            )
            //
            .child(
                div()
                    .text_base()
                    .text_color(rgb(0x34352F))
                    .child("Explora un libro de tu biblioteca"),
            )
            //
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    // un card pequeño con el color de portada
                    .child(
                        div()
                            .w(px(56.))
                            .h(px(72.))
                            .flex_shrink_0()
                            .rounded_md()
                            .bg(rgb(self.library_info.rb_cover_color.clone()))
                            .border_1()
                            .border_color(rgb(0xE5E2DC)),
                    )
                    .child(book_info),
            )
    }
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_6()
            // Header
            .child(
                div().flex().items_center().justify_between().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_size(px(27.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Inicio"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(self.dash_config.muted)
                                .child("Tu biblioteca, organizada."),
                        ),
                ),
            )
            //
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_3()
                    .child(self.dash_card(
                        Icon::new(AssetIconName::LibraryBig).text_color(rgb(0xB94B1B)).size_5(),
                        "Libros",
                        self.library_info.books.to_string(),
                        "en tu biblioteca",
                    ))
                    .child(self.dash_card(
                        Icon::new(AssetIconName::Users).text_color(rgb(0xB94B1B)).size_5(),
                        "Autores",
                        self.library_info.authors.to_string(),
                        "las voces de tus historias",
                    ))
                    .child(self.dash_card(
                        Icon::new(IconName::Heart).text_color(rgb(0xB94B1B)).size_5(),
                        "Favoritos",
                        self.library_info.favs.to_string(),
                        "lecturas imprescindibles",
                    ))
                    .child(self.dash_card(
                        Icon::new(IconName::Building2).text_color(rgb(0xB94B1B)).size_5(),
                        "Editoriales",
                        //"—".into(),
                        self.library_info.publ.to_string(),
                        "las casas que los trajeron",
                    ))
                    .child(self.dash_card(
                        Icon::new(IconName::Palette).text_color(rgb(0xB94B1B)).size_5(),
                        "Géneros",
                        //"—".into(),
                        self.library_info.genre.to_string(),
                        "los caminos que exploras",
                    ))
                    .child(self.dash_card(
                        Icon::new(IconName::Globe).text_color(rgb(0xB94B1B)).size_5(),
                        "Idiomas",
                        //"—".into(),
                        self.library_info.lang.to_string(),
                        "las lenguas que te acompañan",
                    )),
            )
            //
            .child(self.quote_card())
            //
            .child(self.recommendation_card())
    }
}
