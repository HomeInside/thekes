use super::AuthorViewEvent;
use crate::ui::main_window::ColorConfig;
use gpui_kit::IntoElement;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::component::{h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};

#[allow(dead_code)]
pub struct AuthorDetailsView {
    pub(crate) color_config: ColorConfig,
}

impl AuthorDetailsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            color_config: ColorConfig::default(),
        }
    }

    //
    fn metadata(label: &'static str, value: &'static str) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(div().text_xs().text_color(rgb(0x85837D)).child(label))
            .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(value))
    }

    fn tag(value: &'static str) -> impl IntoElement {
        div()
            .px_3()
            .py_1()
            .rounded_full()
            .bg(rgb(0xF5EEE5))
            .text_sm()
            .text_color(rgb(0x79563E))
            .child(value)
    }

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

    fn book_card(
        title: &'static str,
        year: &'static str,
        description: &'static str,
        cover_color: u32,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .items_center()
            .gap_3()
            .p_3()
            .rounded_lg()
            .border_1()
            .border_color(rgb(0xEEE9E2))
            .child(
                div()
                    .size(px(64.))
                    .flex_shrink_0()
                    .rounded_md()
                    .bg(rgb(cover_color))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(IconName::BookOpen).text_color(rgb(0xFFFFFF))),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                    .child(div().text_sm().text_color(rgb(0x85837D)).child(year))
                    .child(div().text_sm().text_color(rgb(0x625F59)).child(description)),
            )
    }

    fn featured_book(
        title: &'static str,
        year: &'static str,
        description: &'static str,
        cover_color: u32,
        initials: &'static str,
    ) -> impl IntoElement {
        v_flex()
            .flex_1()
            .min_w_0()
            .gap_3()
            .p_3()
            .rounded_xl()
            .border_1()
            .border_color(rgb(0xE5E2DC))
            .bg(rgb(0xFDFCFA))
            .child(
                div()
                    .w_full()
                    .h(px(190.))
                    .rounded_lg()
                    .bg(rgb(cover_color))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        v_flex()
                            .w(px(130.))
                            .h(px(160.))
                            .p_3()
                            .gap_2()
                            .items_center()
                            .justify_center()
                            .rounded_sm()
                            .border_1()
                            .border_color(rgb(0xFFFFFF))
                            .bg(rgb(cover_color))
                            .child(
                                div()
                                    .text_size(px(28.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(initials),
                            )
                            .child(div().text_xs().text_color(rgb(0xFFFFFF)).child("EDICIÓN LITERARIA")),
                    ),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                    .child(div().text_sm().text_color(rgb(0x85837D)).child(year))
                    .child(div().text_sm().text_color(rgb(0x625F59)).child(description)),
            )
    }
}

impl EventEmitter<AuthorViewEvent> for AuthorDetailsView {}

impl Render for AuthorDetailsView {
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
                                    .child("Detalle del autor"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(self.color_config.muted)
                                    .child("Perfil biográfico y bibliografía"),
                            ),
                    )
                    .child(
                        Button::new("btn-author-details-back")
                            .ghost()
                            .label("Volver")
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.emit(AuthorViewEvent::ShowList);
                            })),
                    ),
            )
            //
            .child(
                v_flex()
                    .id("author-details-content")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .gap_4()
                    .overflow_y_scrollbar()
                    .child(
                        h_flex()
                            .w_full()
                            .items_start()
                            .gap_6()
                            // Columna izquierda
                            // información básica
                            .child(
                                h_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .items_start()
                                    .gap_3()
                                    //
                                    // Tarjeta lateral
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
                                                    .w_full()
                                                    .h(px(220.))
                                                    .rounded_lg()
                                                    .bg(rgb(0xE9D8C5))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(48.))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(rgb(0x977657))
                                                            .child("LC"),
                                                    ),
                                            )
                                    )
                                    //
                                    // Información principal
                                    .child(
                                        v_flex()
                                            .flex_1()
                                            .min_w_0()
                                            .min_h_0()
                                            .gap_2()
                                            .pt_1()
                                            .child(
                                                div()
                                                    .text_size(px(30.))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Lewis Carroll"),
                                            )
                                            .child(
                                                div()
                                                    .text_base()
                                                    .text_color(rgb(0x625F59))
                                                    //.text_sm()
                                                    .child("Escritor, matemático y fotógrafo británico."),
                                                )
                                                .child(
                                                    div()
                                                    .text_base()
                                                    //.text_color(rgb(0x85837D))
                                                    .text_color(rgb(0x625F59))
                                                    .text_sm()
                                                    .child("Nombre real: Charles Lutwidge Dodgson"),
                                            )
                                            .child(
                                                h_flex()
                                                    .flex_wrap()
                                                    .gap_6()
                                                    .child(Self::metadata("Nacimiento", "27 de enero de 1832"))
                                                    .child(Self::metadata("Fallecimiento", "14 de enero de 1898")),
                                            )
                                            .child(
                                                h_flex()
                                                    .flex_wrap()
                                                    .gap_6()
                                                    .child(Self::metadata("Nacionalidad", "Británica"))
                                                    .child(Self::metadata("Idioma", "Inglés")),
                                            ),
                                    ), //
                            )
                            //
                            // Columna derecha: biografía
                            .child(
                                div().w_full().items_start().flex_1().min_w_0().child(Self::section(
                                    "Biografía",
                                    "Vida y trayectoria literaria",
                                    IconName::BookOpen,
                                    v_flex()
                                        .gap_2()
                                        .child(div().text_sm().text_color(rgb(0x45433F)).child(
                                            "The Reverend Charles Lutwidge Dodgson, better known by the pen name Lewis Carroll, was an English author, mathematician, logician, Anglican clergyman and photographer.His most famous writings are Alice's Adventures in Wonderland and its sequel Through the Looking-Glass as well as the poems \"The Hunting of the Snark\" and \"Jabberwocky\", all considered to be within the genre of literary nonsense.Oxford scholar, Church of England Deacon, University Lecturer in Mathematics and Logic, academic author of learned theses, gifted pioneer of portrait photography, colourful writer of imaginative genius and yet a shy and pedantic man, Lewis Carroll stands pre-eminent in the pantheon of inventive literary geniuses.He also has works published under his real name.",
                                        )),
                                )),
                            )
                    )
                    //
                    // Perfil literario
                    .child(Self::section(
                        "Perfil literario",
                        "Géneros e influencias",
                        //AssetIconName::Feather,
                        IconName::User,
                        v_flex()
                            .gap_3()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Géneros literarios"),
                            )
                            .child(
                                h_flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(Self::tag("Literatura infantil"))
                                    .child(Self::tag("Fantasía"))
                                    .child(Self::tag("Poesía"))
                                    .child(Self::tag("Ficción"))
                                    //
                                    .child(Self::tag("uno"))
                                    .child(Self::tag("dos"))
                                    .child(Self::tag("aqui van"))
                                    .child(Self::tag("tres o cuatro casi"))
                                    .child(Self::tag("cinco")),
                            )
                            .child(div().text_sm().font_weight(FontWeight::MEDIUM).child("Influencias"))
                            .child(
                                h_flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(Self::tag("Edward Lear"))
                                    .child(Self::tag("Nonsense literario")),
                            ),
                    ))
                    .child(Self::section(
                        "Bibliografía",
                        "Obras destacadas y publicaciones del autor",
                        IconName::BookOpen,
                        v_flex()
                            .gap_4()
                            // Obras destacadas
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_3()
                                    .child(Self::featured_book(
                                        "Alice's Adventures in Wonderland",
                                        "1865",
                                        "Novela · Fantasía",
                                        0x52776D,
                                        "ALICE",
                                    ))
                                    .child(Self::featured_book(
                                        "Through the Looking-Glass",
                                        "1871",
                                        "Novela · Fantasía",
                                        0x76658C,
                                        "MIRROR",
                                    )),
                            )
                            // lista secundaria
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child("Otras obras"))
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(Self::book_card(
                                        "The Hunting of the Snark",
                                        "1876",
                                        "Poema narrativo",
                                        0xB47B55,
                                    ))
                                    .child(Self::book_card(
                                        "Sylvie and Bruno",
                                        "1889",
                                        "Novela · Fantasía",
                                        0x657DA0,
                                    )),
                            ),
                    ))
                    //
                    // referencias
                    .child(Self::section(
                        "Referencias externas",
                        "Identificadores bibliográficos",
                        IconName::ExternalLink,
                        v_flex()
                            .gap_3()
                            .child(Self::metadata("OPEN LIBRARY", "OL22098A"))
                            .child(Self::metadata("VIAF", "Pendiente de vincular")),
                    )),
            )
    }
}
