use super::AuthorViewEvent;
use crate::ui::main_window::ColorConfig;
use gpui_kit::IntoElement;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, IconName, h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};

pub struct AuthorFormView {
    pub(crate) color_config: ColorConfig,

    // Información personal
    name: Entity<InputState>,
    lastname: Entity<InputState>,
    nationality: Entity<InputState>,
    birth_date: Entity<InputState>,
    death_date: Entity<InputState>,

    // Perfil literario
    genre: Entity<InputState>,
    pseudonyms: Entity<InputState>,
    languages: Entity<InputState>,
    literary_movements: Entity<InputState>,
    biography: Entity<TextareaState>,

    // Información adicional
    openlibrary_id: Entity<InputState>,
    viaf_id: Entity<InputState>,
    notes: Entity<TextareaState>,
    show_additional: bool,
}

impl EventEmitter<AuthorViewEvent> for AuthorFormView {}

impl AuthorFormView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            color_config: ColorConfig::default(),
            name: cx.new(|cx| InputState::new(window, cx).placeholder("Nombre")),
            lastname: cx.new(|cx| InputState::new(window, cx).placeholder("Apellido")),
            nationality: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. Scottish")),
            birth_date: cx.new(|cx| InputState::new(window, cx).placeholder("AAAA-MM-DD")),
            death_date: cx.new(|cx| InputState::new(window, cx).placeholder("AAAA-MM-DD")),
            genre: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. Fantasía, ciencia ficción")),
            pseudonyms: cx.new(|cx| InputState::new(window, cx).placeholder("Separa los seudónimos por comas")),
            languages: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. Español, inglés")),
            literary_movements: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. Modernismo, realismo mágico")),
            biography: cx.new(|cx| {
                TextareaState::new(window, cx)
                    .rows(6)
                    .placeholder("Escribe aquí la biografía del autor...")
            }),
            openlibrary_id: cx.new(|cx| InputState::new(window, cx).placeholder("Ej. OL31353A")),
            viaf_id: cx.new(|cx| InputState::new(window, cx).placeholder("Identificador VIAF")),
            notes: cx.new(|cx| {
                TextareaState::new(window, cx)
                    .rows(3)
                    .placeholder("Notas privadas sobre el autor...")
            }),
            show_additional: false,
        }
    }

    fn field(label: &'static str, input: &Entity<InputState>) -> impl IntoElement {
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

    // TO REMOVE
    fn date_row(&self) -> impl IntoElement {
        h_flex()
            .w_full()
            .gap_4()
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(Self::field("Fecha de nacimiento", &self.birth_date)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(Self::field("Fecha de fallecimiento(opcional)", &self.death_date)),
            )
    }
}

impl Render for AuthorFormView {
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
                                    .child("Crear autor"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(self.color_config.muted)
                                    .child("agrega un autor a tu biblioteca"),
                            ),
                    )
                    .child(
                        Button::new("author-form-back")
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
                    .id("author-form-content")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .gap_4()
                    .overflow_y_scrollbar()
                    //
                    // Información personal
                    .child(Self::section(
                        "Información personal",
                        "Datos principales del autor",
                        IconName::User,
                        div(),
                        v_flex()
                            .gap_4()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(div().flex_1().min_w_0().child(Self::field("Nombre", &self.name)))
                                    .child(div().flex_1().min_w_0().child(Self::field("Apellidos", &self.lastname))),
                            )
                            .child(Self::field("Nacionalidad", &self.nationality))
                            .child(self.date_row()),
                    ))
                    //
                    // Perfil literario
                    .child(Self::section(
                        "Perfil literario",
                        "Géneros, seudónimos y contexto literario",
                        IconName::BookOpen,
                        div(),
                        v_flex()
                            .gap_4()
                            .child(Self::field("Géneros literarios", &self.genre))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x85837D))
                                    .child("Puedes introducir varios valores separados por comas"),
                            )
                            .child(Self::field("Seudónimo(s)", &self.pseudonyms))
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_w_0()
                                            .child(Self::field("Idiomas de escritura", &self.languages)),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_w_0()
                                            .child(Self::field("Movimientos o corrientes", &self.literary_movements)),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(div().text_sm().font_weight(FontWeight::MEDIUM).child("Biografía"))
                                    .child(
                                        div()
                                            .h(px(220.))
                                            .w_full()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(rgb(0xE5E2DC))
                                            .bg(rgb(0xFFFFFF))
                                            .p_2()
                                            .child(Textarea::new(&self.biography).bordered(false).size_full()),
                                    )
                                    .child(div().text_xs().text_color(rgb(0x85837D)).child(
                                        "Incluye su trayectoria, obras destacadas y cualquier detalle relevante.",
                                    )),
                            ),
                    ))
                    //
                    // Información adicional
                    .child(Self::section(
                        "Información adicional",
                        "Identificadores externos y notas internas.",
                        IconName::BookOpen,
                        Button::new("toggle-author-additional")
                            .ghost()
                            .small()
                            .label(if self.show_additional { "Ocultar" } else { "Mostrar" })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.show_additional = !this.show_additional;
                                cx.notify();
                            })),
                        v_flex().gap_4().when(self.show_additional, |content| {
                            content
                                .child(
                                    h_flex()
                                        .w_full()
                                        .gap_4()
                                        .child(
                                            div()
                                                .flex_1()
                                                .min_w_0()
                                                .child(Self::field("Open Library ID", &self.openlibrary_id)),
                                        )
                                        .child(div().flex_1().min_w_0().child(Self::field("VIAF ID", &self.viaf_id))),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child(div().text_sm().font_weight(FontWeight::MEDIUM).child("Notas internas"))
                                        .child(
                                            div()
                                                .h(px(120.))
                                                .w_full()
                                                .rounded_lg()
                                                .border_1()
                                                .border_color(rgb(0xE5E2DC))
                                                .bg(rgb(0xFFFFFF))
                                                .p_2()
                                                .child(Textarea::new(&self.notes).bordered(false).size_full()),
                                        ),
                                )
                        }),
                    )),
            )
            //
            .child(
                h_flex()
                    .justify_end()
                    .items_center()
                    .gap_3()
                    .pt_2()
                    .child(
                        Button::new("author-form-cancel")
                            .ghost()
                            .secondary()
                            .label("Cancelar")
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.emit(AuthorViewEvent::ShowList);
                            })),
                    )
                    .child(
                        Button::new("btn-author-form-save")
                            .primary()
                            .label("Crear")
                            .on_click(cx.listener(|_this, _, _, _cx| {
                                println!("btn-author-form-save fn");
                            })),
                    ),
            )
    }
}
