use gpui_kit::Rgba;
use gpui_kit::rgb;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ColorConfig {
    pub(crate) bg: Rgba,
    pub(crate) sidebar_bg: Rgba,
    pub(crate) foreground: Rgba,
    pub(crate) muted: Rgba,
    pub(crate) accent: Rgba,
    pub(crate) border: Rgba,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            bg: rgb(0xf8f7f4),
            sidebar_bg: rgb(0xf0efeb),
            foreground: rgb(0x292825),
            muted: rgb(0x85837d),
            accent: rgb(0xe85d20),
            border: rgb(0xe5e2dc),
        }
    }
}
