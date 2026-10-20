use crate::ui::main_window::ColorConfig;
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
