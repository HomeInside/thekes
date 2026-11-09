use gpui_kit::IntoElement;
use gpui_kit::*;
use gpui_kit::{Context, Styled};

use crate::ui::main_window::ColorConfig;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::{Icon, IconName};

#[derive(Debug, Clone, Default)]
pub struct LibraryStats {
    authors: usize,
    books: usize,
}

pub struct Dashboard {
    dash_config: ColorConfig,
    library_info: LibraryStats,
}

// métodos
impl Dashboard {
    pub fn new() -> Self {
        Self {
            dash_config: ColorConfig::default(),
            library_info: LibraryStats::default(),
        }
    }

    pub fn sidebar_card(
        &self,
        icon: AssetIconName,
        title: &'static str,
        description: &'static str,
        count: usize,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .min_w(px(140.))
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
                    .size(px(40.))
                    .rounded_lg()
                    .bg(rgb(0xF4D5C2))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(icon).text_color(rgb(0xB94B1B))),
            )
            .child(
                div()
                    .text_size(px(27.))
                    .font_weight(FontWeight::BOLD)
                    .child(count.to_string()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                    .child(div().text_xs().text_color(self.dash_config.muted).child(description)),
            )
    }
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            // Header
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
                            .child(div().text_sm().child("Tu biblioteca, organizada.")),
                    )
                    .child(
                        div()
                            .h(px(36.))
                            .px_4()
                            .flex()
                            .items_center()
                            .rounded_md()
                            .text_color(rgb(0xffffff)),
                    ),
            )
            // Welcome Panel
            .child(
                div()
                    .w_full()
                    .p_6()
                    .rounded_xl()
                    .border_1()
                    .bg(rgb(0xffffff))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Todo en su lugar."),
                    )
                    .child(
                        div()
                            .text_sm()
                            .child("Organiza tu biblioteca y encuentra rápidamente lo que necesitas."),
                    ),
            )
            // Acceso rápido
            .child(
                div().flex().items_center().justify_between().child(
                    div()
                        .text_size(px(18.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("Acceso rápido"),
                ),
            )
            // Cards
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(self.sidebar_card(AssetIconName::LibraryBig, "Libros", "en tu biblioteca", 128))
                    .child(self.sidebar_card(AssetIconName::Users, "Autores", "registrados", 42))
                    .child(self.sidebar_card(IconName::Heart.into(), "Libros favoritos", "literatura destacada", 16)),
            )
    }
}
