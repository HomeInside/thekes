mod view;
pub use view::{AuthorViewEvent, Authors};
mod actions;
mod form_view;
pub mod form_view_actions;
pub use form_view::AuthorFormView;
mod details_view;
pub use details_view::AuthorDetailsView;
