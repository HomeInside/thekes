use gpui_kit::IntoElement;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;
use gpui_kit::{Context, Styled};

use crate::ui::main_window::ColorConfig;
use gpui_kit::component::IconName;

pub struct Dashboard {
    dash_config: ColorConfig,
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        //
        div()
            // HEADER
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
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
                                    //.text_color(self.sidebar_config.muted)
                                    .child("Tu espacio de trabajo, organizado."),
                            ),
                    )
                    .child(
                        div()
                            .h(px(36.))
                            .px_4()
                            .flex()
                            .items_center()
                            .rounded_md()
                            //.bg(self.sidebar_config.accent)
                            .text_color(rgb(0xffffff))
                            //.child("+  Nuevo proyecto")
                            .child(
                                Button::new("nuevo-pro")
                                    .ghost()
                                    .small()
                                    .icon(IconName::PanelLeftClose)
                                    //TODO
                                    // de esta forma se usa Self
                                    //.on_click(cx.listener(Self::click_button_demo)),
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        // de esta forma se usa this
                                        this.click_button_demo(cx);
                                        cx.notify();
                                    })),
                            ),
                    ),
            )
            // WELCOME PANEL
            .child(
                div()
                    .w_full()
                    .p_6()
                    .rounded_xl()
                    .border_1()
                    //.border_color(self.sidebar_config.border)
                    .bg(rgb(0xffffff))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_size(px(22.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Todo en su lugar."),
                    )
                    .child(
                        div()
                            .text_sm()
                            //.text_color(self.sidebar_config.muted)
                            .child("Organiza tus proyectos y encuentra rápidamente lo que necesitas."),
                    ),
            )
            // ACCESO RÁPIDO
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(18.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Acceso rápido"),
                    )
                    .child(
                        div()
                            .text_sm()
                            //.text_color(self.sidebar_config.accent)
                            .child("Ver todo"),
                    ),
            )
            // CARDS
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(self.sidebar_card(
                        "▣",
                        "Mis proyectos",
                        "Organiza tu trabajo",
                        rgb(0xf9e6dc),
                        self.dash_config.accent,
                        self.dash_config.border,
                        self.dash_config.muted,
                    ))
                    .child(self.sidebar_card(
                        "☆",
                        "Favoritos",
                        "Elementos destacados",
                        rgb(0xf6efcf),
                        rgb(0x9d7b20),
                        self.dash_config.border,
                        self.dash_config.muted,
                    ))
                    .child(self.sidebar_card(
                        "◷",
                        "Recientes",
                        "Continúa donde lo dejaste",
                        rgb(0xe2ece4),
                        rgb(0x497653),
                        self.dash_config.border,
                        self.dash_config.muted,
                    )),
            )
    }
}

// métodos
impl Dashboard {
    pub fn new() -> Self {
        Self {
            dash_config: ColorConfig::default(),
        }
    }

    pub fn sidebar_card(
        &self,
        icon: &'static str,
        title: &'static str,
        description: &'static str,
        _icon_bg: impl Into<gpui_kit::Hsla>,
        icon_color: impl Into<gpui_kit::Hsla>,
        border: impl Into<gpui_kit::Hsla>,
        muted: impl Into<gpui_kit::Hsla>,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .min_w(px(140.))
            .p_4()
            .rounded_lg()
            .border_1()
            .border_color(border)
            .bg(rgb(0xffffff))
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .size(px(40.))
                    .rounded_md()
                    .bg(rgb(0xFFCCE5))
                    .text_color(icon_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(20.))
                    .child(icon),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(div().font_weight(FontWeight::MEDIUM).child(title))
                    .child(div().text_xs().text_color(muted).child(description)),
            )
    }
}
