use crate::utils::consts;
use gpui_kit::IntoElement;
use gpui_kit::assets::IconName as AssetIconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::h_flex;
use gpui_kit::component::sidebar::{Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem};
use gpui_kit::component::{Icon, IconName};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::{Context, Styled};

use crate::ui::main_window::ColorConfig;
use crate::ui::main_window::subcriptions::{ActiveModule, NavigateTo};

pub struct AppSidebar {
    pub(crate) collapsed: bool,
    pub(crate) sidebar_config: ColorConfig,
    pub(crate) active_item: ActiveModule,
}

// eventos para notificar el cambio de modulo/panel
impl EventEmitter<NavigateTo> for AppSidebar {}

// render
impl Render for AppSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new("MySidebar")
            .w(px(250.))
            .collapsed(self.collapsed)
            .collapsible(true)
            //TO FIX
            //.bg(self.sidebar_config.sidebar_bg)
            .bg(cx.theme().muted)
            .text_color(self.sidebar_config.foreground)
            .header(
                SidebarHeader::new()
                    // app name
                    .child(
                        h_flex()
                            .w_full()
                            //.h(px(42.))
                            .items_center()
                            //.justify_start()
                            .gap_2()
                            .px_2()
                            .text_color(self.sidebar_config.accent)
                            .when(self.collapsed, |this| this.justify_center())
                            .child(Icon::new(IconName::BookOpen))
                            .when(!self.collapsed, |this| {
                                this.child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        //.text_size(px(21.))
                                        .text_size(px(26.))
                                        .font_weight(FontWeight::BOLD)
                                        .child(consts::APP_NAME),
                                )
                            }),
                    ),
            )
            // Inicio
            .child(
                SidebarGroup::new("Inicio").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("Panel del control")
                                .icon(IconName::LayoutDashboard)
                                .active(self.active_item == ActiveModule::Dashboard)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    //TODO
                                    this.active_item = ActiveModule::Dashboard;
                                    println!("enviando event NavigateTo::Dashboard");
                                    cx.emit(NavigateTo {
                                        module: ActiveModule::Dashboard,
                                    });
                                    cx.notify();
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("Todos los libros")
                                .icon(AssetIconName::LibraryBig)
                                .active(self.active_item == ActiveModule::Books)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    //TODO
                                    this.active_item = ActiveModule::Books;
                                    println!("enviando event NavigateTo::Books");
                                    cx.emit(NavigateTo {
                                        module: ActiveModule::Books,
                                    });
                                    cx.notify();
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("Autores")
                                .icon(AssetIconName::Users)
                                .active(self.active_item == ActiveModule::Authors)
                                .on_click(cx.listener(|_this, _, _, cx| {
                                    //TODO
                                    //this.active_item = SidebarItem::Authors;
                                    cx.notify();
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("Anotaciones")
                                .icon(AssetIconName::NotebookPen)
                                .active(self.active_item == ActiveModule::Notes)
                                .on_click(cx.listener(|_this, _, _, cx| {
                                    //TODO
                                    //this.active_item = SidebarItem::Notes;
                                    cx.notify();
                                })),
                        ),
                ),
            )
            //
            // Biblioteca
            .child(
                SidebarGroup::new("Biblioteca").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("Sin clasificar")
                                .icon(IconName::FolderClosed)
                                .active(self.active_item == ActiveModule::Unclassified)
                                .on_click(cx.listener(|_this, _, _, cx| {
                                    //TODO
                                    // this.active_item = SidebarItem::Unclassified;
                                    cx.notify();
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("Favoritos")
                                .icon(IconName::Heart)
                                .active(self.active_item == ActiveModule::Favorites)
                                .on_click(cx.listener(|_this, _, _, cx| {
                                    //TODO
                                    // this.active_item = SidebarItem::Favorites;
                                    cx.notify();
                                })),
                        )
                        //
                        .child(
                            SidebarMenuItem::new("Estoy leyendo")
                                .icon(AssetIconName::BookOpenCheck)
                                .active(self.active_item == ActiveModule::CurrentlyReading)
                                .on_click(cx.listener(|_this, _, _, cx| {
                                    //TODO
                                    // this.active_item = SidebarItem::CurrentlyReading;
                                    cx.notify();
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("Lo que quiero leer")
                                .icon(AssetIconName::ListTodo)
                                .active(self.active_item == ActiveModule::WantToRead)
                                .on_click(cx.listener(|_this, _, _, cx| {
                                    //TODO
                                    // this.active_item = SidebarItem::WantToRead;
                                    cx.notify();
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("Finalizado")
                                .icon(AssetIconName::CircleCheckBig)
                                .active(self.active_item == ActiveModule::Finished)
                                .on_click(cx.listener(|_this, _, _, cx| {
                                    //TODO
                                    // this.active_item = SidebarItem::Finished;
                                    cx.notify();
                                })),
                        ),
                ),
            )
            /*
            //TO TEST
            // iconos
            .child(
                SidebarGroup::new("Iconos").child(
                    SidebarMenu::new()
                        .child(SidebarMenuItem::new("Colecciones").icon(AssetIconName::Book))
                        .child(SidebarMenuItem::new("BookHeart").icon(AssetIconName::BookHeart))
                        .child(SidebarMenuItem::new("BookOpenCheck").icon(AssetIconName::BookOpenCheck))
                        .child(SidebarMenuItem::new("ListTodo").icon(AssetIconName::ListTodo))
                        .child(SidebarMenuItem::new("SquareLibrary").icon(AssetIconName::SquareLibrary))
                        .child(SidebarMenuItem::new("TimerReset").icon(AssetIconName::TimerReset))
                        .child(SidebarMenuItem::new("Accessibility").icon(AssetIconName::Accessibility))
                        .child(SidebarMenuItem::new("AlarmClock").icon(AssetIconName::AlarmClock))
                        .child(SidebarMenuItem::new("Folder").icon(IconName::Folder))
                        .child(SidebarMenuItem::new("Inbox").icon(IconName::Inbox)),
                ),
            )
            */
            //
            // FOOTER
            .footer(
                SidebarFooter::new().child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .gap_2()
                        .when(!self.collapsed, |this| {
                            this.child(
                                // Avatar
                                div()
                                    .size(px(36.))
                                    .flex_shrink_0()
                                    .rounded_full()
                                    .bg(rgb(0xf3d5c5))
                                    .text_color(self.sidebar_config.accent)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("SH"),
                            )
                            .child(
                                // Información de la cuenta
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(div().text_sm().font_weight(FontWeight::MEDIUM).child("Sherlock Holmes"))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(self.sidebar_config.muted)
                                            .child("personal account"),
                                    ),
                            )
                        })
                        // Botón de configuración
                        .child(
                            div().occlude().child(
                                Button::new("sidebar-settings")
                                    .ghost()
                                    .small()
                                    .icon(IconName::Settings)
                                    .tooltip("Configuración")
                                    .on_click(cx.listener(|_this, _, _, _cx| {
                                        //this.click_settings(cx)
                                        println!("aqui va el click de click_settings");
                                    })),
                            ),
                        ),
                ),
            )
    }
}

// metodos/events
impl AppSidebar {
    pub fn new() -> Self {
        Self {
            collapsed: false,
            sidebar_config: ColorConfig::default(),
            active_item: ActiveModule::Dashboard,
        }
    }

    pub fn click_toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        println!("toggle_sidebar fn");
        self.collapsed = !self.collapsed;
        cx.notify();
    }
}
