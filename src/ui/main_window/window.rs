use super::ColorConfig;
use super::subcriptions::{ActiveModule, NavigateTo};
use crate::database::DatabaseService;
use crate::ui::authors::{AuthorDetailsView, AuthorFormView, Authors};
use crate::ui::books::{BookDetailsView, BookFormView, Books};
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
use std::sync::Arc;

pub struct MainWindow {
    #[allow(dead_code)]
    pub(crate) loading: bool,
    pub(crate) toggle_theme: bool,
    sidebar: Entity<AppSidebar>,
    pub(crate) active_module: ActiveModule,
    pub(crate) color_config: ColorConfig,
    //
    pub(crate) dashboard: Entity<Dashboard>,
    //
    // libros crear/editar
    pub(crate) books: Entity<Books>,
    pub(crate) book_form: Entity<BookFormView>,
    pub(crate) show_book_form: bool,
    // libros detalles
    pub(crate) books_details: Entity<BookDetailsView>,
    pub(crate) show_books_details: bool,
    //
    // autores crear/editar
    pub(crate) authors: Entity<Authors>,
    pub(crate) author_form: Entity<AuthorFormView>,
    pub(crate) show_author_form: bool,
    // autores detalles
    pub(crate) author_details: Entity<AuthorDetailsView>,
    pub(crate) show_author_details: bool,
    //
    //_sidebar_subscription: Subscription,
    pub(super) _subscriptions: Vec<Subscription>,
    #[allow(dead_code)]
    db_conn: Arc<DatabaseService>,
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
    pub fn new(window: &mut Window, cx: &mut Context<Self>, db_conn: Arc<DatabaseService>) -> Self {
        let sidebar = cx.new(|_| AppSidebar::new());
        let dashboard = cx.new(|_| Dashboard::new());
        //
        // cada modulo gestiona la conexión a la DB
        //
        let books = cx.new(|cx| Books::new(window, cx, db_conn.clone()));
        let book_form = cx.new(|cx| BookFormView::new(window, cx, db_conn.clone()));
        let books_details = cx.new(|cx| BookDetailsView::new(window, cx, db_conn.clone()));
        //
        // manejar instancia del módulo de autores
        let authors = cx.new(|cx| Authors::new(window, cx, db_conn.clone()));
        let author_form = cx.new(|cx| AuthorFormView::new(window, cx, db_conn.clone()));
        let author_details = cx.new(|cx| AuthorDetailsView::new(window, cx, db_conn.clone()));

        // manejar eventos de módulo autores
        let subs_author_view = Self::subscribe_author_events(&authors, window, cx, "authors_list");
        let subs_author_form = Self::subscribe_author_events(&author_form, window, cx, "author_form");
        let subs_author_details = Self::subscribe_author_events(&author_details, window, cx, "authors_details");
        //
        // manejar eventos de módulo libros
        let subs_books_view = Self::subscribe_books_events(&books, window, cx, "books_list");
        let subs_books_form = Self::subscribe_books_events(&book_form, window, cx, "books_form");
        let subs_books_details = Self::subscribe_books_events(&books_details, window, cx, "books_details");

        // manejar eventos de la sidebar
        let subs_sidebar = cx.subscribe(&sidebar, |this, _sidebar, event: &NavigateTo, cx| {
            println!("llegó desde NavigateTo: {:?}", event.module);
            // TO FIX validar y mejorar esta parte
            // cargar los datos de autor
            if event.module == ActiveModule::Authors {
                this.authors.update(cx, |authors, cx| {
                    authors.load_authors(cx);
                });
            } else if event.module == ActiveModule::Books {
                this.books.update(cx, |authors, cx| {
                    authors.load_books(cx);
                });
            }
            //

            this.active_module = event.module;
            cx.notify();
        });

        Self {
            loading: true,
            toggle_theme: true,
            sidebar,
            active_module: ActiveModule::Dashboard,
            color_config: ColorConfig::default(),
            //
            dashboard,
            // libros crear/editar
            books,
            book_form,
            show_book_form: false,
            books_details,
            show_books_details: false,
            authors,
            //
            // autores crear/editar
            author_form,
            show_author_form: false,
            // autores detalles
            author_details,
            show_author_details: false,
            _subscriptions: vec![
                subs_sidebar,
                //
                subs_author_view,
                subs_author_form,
                subs_author_details,
                //
                subs_books_view,
                subs_books_form,
                subs_books_details,
            ],
            db_conn,
        }
    }
}
