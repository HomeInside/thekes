use super::AuthorViewEvent;
use crate::database::DatabaseService;
use crate::database::authors::model::AuthorModel;
use crate::database::authors::service::AuthorService;
use crate::database::books::model::BooksModel;
use crate::ui::main_window::ColorConfig;
use crate::utils::date_utils;
use gpui_kit::IntoElement;
use gpui_kit::base::Link;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::component::{h_flex, v_flex};
use gpui_kit::*;
use gpui_kit::{Context, Render, Styled, Window};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct AuthorDetailModel {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) last_name: String,
    pub(crate) names: String,
    pub(crate) birth_place: String,
    pub(crate) date_of_birth: String,
    pub(crate) date_of_death: String,
    pub(crate) genres: String,
    pub(crate) biography: String,
    pub(crate) description: String,
    pub(crate) langs: String,
    pub(crate) cover_color: u32,
    pub(crate) ol_id: Option<String>,
    pub(crate) viaf_id: Option<String>,
}

#[allow(dead_code)]
pub struct AuthorDetailsView {
    pub(crate) color_config: ColorConfig,
    pub(crate) author_detail: AuthorDetailModel,
    pub(crate) books: Vec<BooksModel>,
    pub(crate) author_detail_service: AuthorService,
}

impl AuthorDetailsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, db: Arc<DatabaseService>) -> Self {
        let author_detail_service = AuthorService::new(db.clone());
        Self {
            color_config: ColorConfig::default(),
            author_detail: AuthorDetailModel::default(),
            books: Vec::new(),
            author_detail_service,
        }
    }

    pub fn gen_monogram(&self) -> String {
        let first = self.author_detail.name.chars().next().unwrap_or('?');
        let last = self.author_detail.last_name.chars().next();

        match last {
            Some(last) => format!("{}{}", first.to_uppercase(), last.to_uppercase()),
            None => first.to_uppercase().to_string(),
        }
    }

    // TODO
    pub fn make_books_data(&mut self, books: &[BooksModel], _cx: &mut Context<Self>) {
        println!("make_books_data fn");
        if !books.is_empty() {
            self.books = books.to_vec();
        }
    }

    pub fn render_books(&self) -> impl IntoElement {
        if !self.books.is_empty() {
            v_flex()
                .gap_2()
                .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child("Otras obras"))
                .children(self.books.iter().map(|book| {
                    Self::book_card(
                        book.title.clone(),
                        book.published_date.to_string(),
                        book.genres.clone().unwrap_or_else(|| "-".to_string()),
                        0xB47B55,
                        // 0x657DA0,
                    )
                }))
        } else {
            //no se encontraron libros del autor con ID: {}", author_id
            v_flex().gap_2().child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("no se encontraron libros de este autor"),
            )
        }
    }

    ///usa los datos de `AuthorModel` y los
    /// organiza para mostrarlos en la ui
    pub fn make_author_data(&mut self, author: &AuthorModel, _cx: &mut Context<Self>) {
        self.author_detail.id = author.id;
        self.author_detail.name = author.name.clone();
        self.author_detail.last_name = author.last_name.clone();
        self.author_detail.names = format!("{} {}", author.name.clone(), author.last_name.clone());

        // lugar de nacimiento
        self.author_detail.birth_place = match &author.birth_place {
            Some(birth_place) => birth_place.to_string(),
            None => "-".to_string(),
        };

        // fecha de nacimiento
        self.author_detail.date_of_birth = date_utils::date_to_local_string(author.date_of_birth.clone(), "en");

        // fecha de fallecimiento
        self.author_detail.date_of_death = match &author.date_of_death {
            Some(death_year) => date_utils::date_to_local_string(death_year.to_string(), "en"),
            None => "—".to_string(),
        };

        self.author_detail.genres = author.genres.clone();
        self.author_detail.biography = author.biography.clone();
        self.author_detail.description = "sin descripción".into();

        self.author_detail.langs = match &author.langs {
            Some(langs) => langs.to_string(),
            None => "—".to_string(),
        };

        let cover_color = match &author.cover_color {
            Some(color) => *color,
            None => 0xF4D5C2,
        };

        self.author_detail.cover_color = cover_color;

        self.author_detail.ol_id = author.open_library_id.clone();
        self.author_detail.viaf_id = author.viaf_id.clone();
    }

    pub fn make_tags(&self) -> impl IntoElement {
        h_flex().flex_wrap().gap_2().children(
            self.author_detail
                .genres
                .split(',')
                .map(|genre| Self::tag(genre.trim().to_string())),
        )
    }

    // TODO
    pub fn make_influences(&self) -> impl IntoElement {
        h_flex().flex_wrap().gap_2().child(Self::tag("no definidas".into()))
        //.child(Self::tag("Edward Lear".into()))
        //.child(Self::tag("Nonsense literario".into())),
    }

    fn meta_link(
        id: &'static str,
        label: &'static str,
        value: Option<String>,
        base_url: &'static str,
    ) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(div().text_xs().text_color(rgb(0x85837D)).child(label))
            .child(match value {
                Some(value) => {
                    let url = format!("{base_url}{value}");

                    //div().child(Link::new(id).href(url).child(value))
                    Link::new(id)
                        .href(url)
                        .cursor_pointer()
                        .open_with(|href, _, _, cx| cx.open_url(href))
                        .child(
                            h_flex()
                                .gap_1()
                                .items_center()
                                .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(value))
                                .child(Icon::new(IconName::ExternalLink).size_3()),
                        )
                        .into_any_element()
                }
                None => div().text_sm().child("No vinculado").into_any_element(),
            })
    }

    //
    fn metadata(label: &'static str, value: String) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(div().text_xs().text_color(rgb(0x85837D)).child(label))
            .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(value))
    }

    fn tag(value: String) -> impl IntoElement {
        div()
            .px_3()
            .py_1()
            .rounded_full()
            .bg(rgb(0xF5EEE5))
            .text_sm()
            .text_color(rgb(0x79563E))
            .child(value)
    }

    fn section(
        title: &'static str,
        description: &'static str,
        icon: IconName,
        content: impl IntoElement,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_4()
            .p_4()
            .rounded_xl()
            .border_1()
            .border_color(rgb(0xE5E2DC))
            .bg(rgb(0xFDFCFA))
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .size(px(40.))
                            .flex_shrink_0()
                            .rounded_lg()
                            .bg(rgb(0xF4D5C2))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Icon::new(icon).text_color(rgb(0xB94B1B))),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_1()
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                            .child(div().text_sm().text_color(rgb(0x85837D)).child(description)),
                    ),
            )
            .child(content)
    }

    fn book_card(title: String, year: String, description: String, cover_color: u32) -> impl IntoElement {
        h_flex()
            .w_full()
            .items_center()
            .gap_3()
            .p_3()
            .rounded_lg()
            .border_1()
            .border_color(rgb(0xEEE9E2))
            .child(
                div()
                    .size(px(64.))
                    .flex_shrink_0()
                    .rounded_md()
                    .bg(rgb(cover_color))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(IconName::BookOpen).text_color(rgb(0xFFFFFF))),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                    .child(div().text_sm().text_color(rgb(0x85837D)).child(year))
                    .child(div().text_sm().text_color(rgb(0x625F59)).child(description)),
            )
    }

    // TO FIX revisar si se usa
    #[allow(dead_code)]
    fn featured_book(
        title: &'static str,
        year: &'static str,
        description: &'static str,
        cover_color: u32,
        initials: &'static str,
    ) -> impl IntoElement {
        v_flex()
            .flex_1()
            .min_w_0()
            .gap_3()
            .p_3()
            .rounded_xl()
            .border_1()
            .border_color(rgb(0xE5E2DC))
            .bg(rgb(0xFDFCFA))
            .child(
                div()
                    .w_full()
                    .h(px(190.))
                    .rounded_lg()
                    .bg(rgb(cover_color))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        v_flex()
                            .w(px(130.))
                            .h(px(160.))
                            .p_3()
                            .gap_2()
                            .items_center()
                            .justify_center()
                            .rounded_sm()
                            .border_1()
                            .border_color(rgb(0xFFFFFF))
                            .bg(rgb(cover_color))
                            .child(
                                div()
                                    .text_size(px(28.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(initials),
                            )
                            .child(div().text_xs().text_color(rgb(0xFFFFFF)).child("EDICIÓN LITERARIA")),
                    ),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                    .child(div().text_sm().text_color(rgb(0x85837D)).child(year))
                    .child(div().text_sm().text_color(rgb(0x625F59)).child(description)),
            )
    }
}

impl EventEmitter<AuthorViewEvent> for AuthorDetailsView {}

impl Render for AuthorDetailsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .gap_4()
            //
            // Cabecera
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(27.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Detalle del autor"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(self.color_config.muted)
                                    .child("Perfil biográfico y bibliografía"),
                            ),
                    )
                    .child(
                        Button::new("btn-author-details-back")
                            .ghost()
                            .label("Volver")
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.emit(AuthorViewEvent::List);
                            })),
                    ),
            )
            //
            .child(
                v_flex()
                    .id("author-details-content")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .gap_4()
                    .overflow_y_scrollbar()
                    .child(
                        h_flex()
                            .w_full()
                            .items_start()
                            .gap_6()
                            // Columna izquierda
                            // información básica
                            .child(
                                h_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .items_start()
                                    .gap_3()
                                    //
                                    // Tarjeta lateral
                                    .child(
                                        v_flex()
                                            .w(px(200.))
                                            .flex_shrink_0()
                                            .items_center()
                                            .gap_2()
                                            .p_2()
                                            .rounded_xl()
                                            .border_1()
                                            .border_color(rgb(0xE5E2DC))
                                            .bg(rgb(0xFDFCFA))
                                            .child(
                                                div()
                                                    .w_full()
                                                    .h(px(220.))
                                                    .rounded_lg()
                                                    .bg(rgb(0xE9D8C5))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(48.))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(rgb(0x977657))
                                                            //iniciales
                                                            .child(self.gen_monogram()),
                                                    ),
                                            ),
                                    )
                                    //
                                    // Información principal
                                    .child(
                                        v_flex()
                                            .flex_1()
                                            .min_w_0()
                                            .min_h_0()
                                            .gap_2()
                                            .pt_1()
                                            .child(
                                                div()
                                                    .text_size(px(30.))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child(self.author_detail.names.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_base()
                                                    .text_color(rgb(0x625F59))
                                                    .child(self.author_detail.description.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_base()
                                                    //.text_color(rgb(0x85837D))
                                                    .text_color(rgb(0x625F59))
                                                    .text_sm()
                                                    .child(format!(
                                                        "Nombre real: {}",
                                                        self.author_detail.names.clone()
                                                    )),
                                            )
                                            .child(
                                                h_flex()
                                                    .flex_wrap()
                                                    .gap_6()
                                                    .child(Self::metadata(
                                                        "Nacimiento",
                                                        self.author_detail.date_of_birth.clone(),
                                                    ))
                                                    .child(Self::metadata(
                                                        "Fallecimiento",
                                                        self.author_detail.date_of_death.clone(),
                                                    )),
                                            )
                                            .child(
                                                h_flex()
                                                    .flex_wrap()
                                                    .gap_6()
                                                    .child(Self::metadata(
                                                        "Nacionalidad",
                                                        self.author_detail.birth_place.clone(),
                                                    ))
                                                    .child(Self::metadata("Idiomas", self.author_detail.langs.clone())),
                                            ),
                                    ), //
                            )
                            //
                            // Columna derecha: biografía
                            .child(
                                div().w_full().items_start().flex_1().min_w_0().child(Self::section(
                                    "Biografía",
                                    "Vida y trayectoria literaria",
                                    IconName::BookOpen,
                                    v_flex().gap_2().child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x45433F))
                                            // si la biografía es muy larga...
                                            .line_clamp(9)
                                            .text_ellipsis()
                                            .child(self.author_detail.biography.clone()),
                                    ),
                                )),
                            ),
                    )
                    //
                    // Perfil literario
                    .child(Self::section(
                        "Perfil literario",
                        "Géneros e influencias",
                        //AssetIconName::Feather,
                        IconName::User,
                        v_flex()
                            .gap_3()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Géneros literarios"),
                            )
                            .child(self.make_tags())
                            //
                            .child(div().text_sm().font_weight(FontWeight::MEDIUM).child("Influencias"))
                            .child(self.make_influences()),
                    ))
                    .child(Self::section(
                        "Bibliografía",
                        "Obras destacadas y publicaciones del autor",
                        IconName::BookOpen,
                        v_flex()
                            .gap_4()
                            // TODO vamos por aqui
                            // Obras destacadas
                            /*
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_3()
                                    .child(Self::featured_book(
                                        "Alice's Adventures in Wonderland",
                                        "1865",
                                        "Novela · Fantasía",
                                        0x52776D,
                                        "ALICE",
                                    ))
                                    .child(Self::featured_book(
                                        "Through the Looking-Glass",
                                        "1871",
                                        "Novela · Fantasía",
                                        0x76658C,
                                        "MIRROR",
                                    )),
                            )
                            */
                            // lista secundaria
                            .child(self.render_books()),
                    ))
                    //
                    // referencias
                    .child(Self::section(
                        "Referencias externas",
                        "Identificadores bibliográficos",
                        IconName::ExternalLink,
                        v_flex()
                            .gap_3()
                            .child(Self::meta_link(
                                "open-library-link",
                                "OPEN LIBRARY",
                                self.author_detail.ol_id.clone(),
                                "https://openlibrary.org/authors/",
                            ))
                            .child(Self::meta_link(
                                "viaf-link",
                                "VIAF",
                                self.author_detail.viaf_id.clone(),
                                "https://viaf.org/viaf/",
                            )),
                    )),
            )
    }
}
