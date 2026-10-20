use super::ColorConfig;
use super::subcriptions::{ActiveModule, NavigateTo};
use crate::ui::authors::{AuthorFormView, AuthorViewEvent, Authors};
use crate::ui::books::Books;
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
    pub(crate) books: Entity<Books>,
    pub(crate) authors: Entity<Authors>,
    _sidebar_subscription: Subscription,
    pub(crate) active_module: ActiveModule,
    pub(crate) color_config: ColorConfig,
    // para autores
    pub(crate) author_form: Entity<AuthorFormView>,
    pub(crate) show_author_form: bool,
    _author_view_subscription: Subscription,
    _author_form_subscription: Subscription,
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

// métodos
impl MainWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|_| AppSidebar::new());
        let dashboard = cx.new(|_| Dashboard::new());
        let books = cx.new(|cx| Books::new(window, cx));
        //let authors = cx.new(|_| Authors::new());
        let authors = cx.new(|cx| Authors::new(window, cx));
        // ==============
        // TODO para autores
        let author_form = cx.new(|cx| AuthorFormView::new(window, cx));
        //
        let author_view_subscription = cx.subscribe(&authors, |this, _authors, event: &AuthorViewEvent, cx| {
            match event {
                AuthorViewEvent::ShowForm => this.show_author_form = true,
                AuthorViewEvent::ShowList => this.show_author_form = false,
            }
            cx.notify();
        });

        let author_form_subscription = cx.subscribe(&author_form, |this, _form, event: &AuthorViewEvent, cx| {
            match event {
                AuthorViewEvent::ShowForm => this.show_author_form = true,
                AuthorViewEvent::ShowList => this.show_author_form = false,
            }
            cx.notify();
        });
        // ==============

        let subscription = cx.subscribe(&sidebar, |this, _sidebar, event: &NavigateTo, cx| {
            println!("llegó desde NavigateTo: {:?}", event.module);
            this.active_module = event.module;
            cx.notify();
        });

        Self {
            loading: true,
            toggle_theme: true,
            sidebar,
            dashboard,
            books,
            authors,
            _sidebar_subscription: subscription,
            active_module: ActiveModule::Dashboard,
            color_config: ColorConfig::default(),
            // TODO para autores
            author_form,
            show_author_form: false,
            _author_view_subscription: author_view_subscription,
            _author_form_subscription: author_form_subscription,
        }
    }

    // TO REMOVE
    pub fn load_notes(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Anotaciones")
    }

    // TO CHECK
    pub fn load_active_module(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        //println!("load_active_module fn: {:?}", self.active_module);
        match self.active_module {
            ActiveModule::Dashboard => self.dashboard.clone().into_any_element(),
            ActiveModule::Books => self.books.clone().into_any_element(),
            //ActiveModule::Authors => self.authors.clone().into_any_element(),
            // TODO para autores
            ActiveModule::Authors => {
                if self.show_author_form {
                    self.author_form.clone().into_any_element()
                } else {
                    self.authors.clone().into_any_element()
                }
            }
            //
            ActiveModule::Notes => self.load_notes(cx).into_any_element(),
            _ => div().child("nada aun").into_any_element(),
        }
    } //load_active_module
}
