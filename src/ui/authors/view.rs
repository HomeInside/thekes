use gpui_kit::IntoElement;
use gpui_kit::*;
use gpui_kit::{Context, Styled};

use crate::ui::main_window::ColorConfig;

pub struct Authors {
    color_config: ColorConfig,
}

impl Render for Authors {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            // HEADER
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
                                .child("Lista de Autores"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(self.color_config.muted)
                                .child("Tus autores favoritos, organizados."),
                        ),
                ),
            )
    }
}

// métodos
impl Authors {
    pub fn new() -> Self {
        Self {
            color_config: ColorConfig::default(),
        }
    }
}
