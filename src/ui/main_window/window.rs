use crate::utils::consts;
use gpui_kit::IntoElement;
use gpui_kit::component::TitleBar;
use gpui_kit::component::h_flex;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::*;
use gpui_kit::{Context, Render, Styled, Window};

pub struct MainWindow {
    loading: bool,
    collapsed: bool,
}

impl MainWindow {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            loading: true,
            collapsed: false,
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bg = rgb(0xffffff);
        let foreground = rgb(0x000000);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(bg)
            .text_color(foreground)
            //
            // TitleBar
            .child(
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .h_full()
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            h_flex().items_center().gap_3().child(
                                h_flex()
                                    .items_center()
                                    .gap_2()
                                    .child(Icon::new(IconName::BookOpen))
                                    .child(div().font_weight(FontWeight::SEMIBOLD).child(consts::APP_NAME)),
                            ),
                        ),
                ),
            )
            //
            // Main app
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .flex()
                    .bg(bg)
                    .text_color(foreground)
                    .child("Tus libros, organizados."),
            )
    }
}
