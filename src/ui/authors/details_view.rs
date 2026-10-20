use super::AuthorViewEvent;
use crate::ui::main_window::ColorConfig;
use gpui_kit::IntoElement;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;
use gpui_kit::{Context, Entity, Render, Styled, Window};

pub struct AuthorDetailsView {
    pub(crate) color_config: ColorConfig,
}

impl AuthorDetailsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            color_config: ColorConfig::default(),
        }
    }
}

impl EventEmitter<AuthorViewEvent> for AuthorDetailsView {}

impl Render for AuthorDetailsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            // deben ir en la raiz para que puedan propagarse
            // los eventos en cualquier parte de la app
            //.on_action(cx.listener(Self::click_author_action))
            .size_full()
            .child("aqui va el detalle del autor")
            .child(
                Button::new("btn-author-details-back")
                    .ghost()
                    .label("Volver")
                    .on_click(cx.listener(|_this, _, _, cx| {
                        cx.emit(AuthorViewEvent::ShowList);
                    })),
            )
    }
}
