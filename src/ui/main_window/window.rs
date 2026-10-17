use super::ColorConfig;
use super::subcriptions::{ActiveModule, NavigateTo};
use crate::ui::dashboard::Dashboard;
use crate::ui::sidebar::AppSidebar;
use crate::utils::consts;
use gpui_kit::IntoElement;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::Sizable;
use gpui_kit::component::TitleBar;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::h_flex;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::*;
use gpui_kit::{Context, Render, Styled, Window};

pub struct MainWindow {
    pub(crate) loading: bool,
    pub(crate) toggle_theme: bool,
    sidebar: Entity<AppSidebar>,
    pub(crate) dashboard: Entity<Dashboard>,
    _sidebar_subscription: Subscription,
    pub(crate) active_module: ActiveModule,
    pub(crate) color_config: ColorConfig,
}

impl MainWindow {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|_| AppSidebar::new());
        let dashboard = cx.new(|_| Dashboard::new());

        let subscription = cx.subscribe(&sidebar, |this, _sidebar, event: &NavigateTo, cx| {
            println!("llegó desde NavigateTo");
            this.active_module = event.module;
            cx.notify();
        });

        Self {
            loading: true,
            toggle_theme: true,
            sidebar,
            dashboard,
            _sidebar_subscription: subscription,
            active_module: ActiveModule::Dashboard,
            color_config: ColorConfig::default(),
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            //TO FIX
            .bg(self.color_config.bg)
            //.bg(cx.theme().muted)
            .text_color(self.color_config.foreground)
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
                            h_flex()
                                .items_center()
                                .gap_3()
                                // app name
                                .child(
                                    h_flex()
                                        .items_center()
                                        .gap_2()
                                        .child(Icon::new(IconName::BookOpen))
                                        .child(div().font_weight(FontWeight::SEMIBOLD).child(consts::APP_NAME)),
                                )
                                // boton para el toggle-sidebar
                                .child(
                                    div().occlude().child(
                                        Button::new("toggle-sidebar")
                                            .ghost()
                                            .small()
                                            .icon(if self.sidebar.read(cx).collapsed {
                                                //IconName::PanelLeft
                                                IconName::PanelLeftClose
                                            } else {
                                                //IconName::PanelRight
                                                IconName::PanelLeftOpen
                                            })
                                            .tooltip(if self.sidebar.read(cx).collapsed {
                                                "Expand sidebar"
                                            } else {
                                                "Collapse sidebar"
                                            })
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                // se llama al evento de la sidebar
                                                this.sidebar.update(cx, |sidebar, cx| {
                                                    sidebar.click_toggle_sidebar(cx);
                                                });
                                            })),
                                    ),
                                ),
                        )
                        // botones de la derecha
                        .child(
                            h_flex()
                                .items_center()
                                .gap_3()
                                //
                                .child(
                                    div().occlude().child(
                                        Button::new("theme-button")
                                            .ghost()
                                            .small()
                                            .icon(if self.toggle_theme {
                                                IconName::Sun
                                            } else {
                                                //Icon::new(AssetIconName::MoonStar)
                                                IconName::Moon
                                            })
                                            .tooltip(if self.toggle_theme { "Light theme" } else { "Dark Theme" })
                                            .tooltip("Change theme")
                                            .on_click(cx.listener(|this, _, _, cx| this.click_toggle_theme(cx))),
                                    ),
                                )
                                //
                                .child(Icon::new(AssetIconName::Palette))
                                .child(Icon::new(IconName::Bell))
                                .child(
                                    div().occlude().child(
                                        Button::new("settings-button")
                                            .ghost()
                                            .small()
                                            .icon(IconName::Settings)
                                            .tooltip("Configuración")
                                            .on_click(cx.listener(|this, _, _, cx| this.click_settings(cx))),
                                    ),
                                ),
                        ),
                ),
            )
            //
            // Main content
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .flex()
                    .bg(self.color_config.bg)
                    .text_color(self.color_config.foreground)
                    //
                    // Sidebar
                    .child(self.sidebar.clone())
                    //
                    // Main module content
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .min_h_0()
                            .flex()
                            .flex_col()
                            .p_7()
                            .gap_6()
                            // cargar cada panel en la main app
                            .child(self.load_active_module(cx)),
                    ),
            )
    }
}
