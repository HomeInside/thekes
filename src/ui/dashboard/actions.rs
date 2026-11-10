use super::Dashboard;
use gpui_kit::Context;

//acciones/eventos
impl Dashboard {
    /// obtener estadisticas desde la DB
    pub fn load_stats(&mut self, cx: &mut Context<Self>) {
        println!("Dashboard load_stats fn");
        match self.general_service.get_stats() {
            Ok(Some(stats)) => {
                self.library_info.authors = usize::try_from(stats.authors).unwrap_or(0);
                self.library_info.books = usize::try_from(stats.books).unwrap_or(0);
                self.library_info.favs = usize::try_from(stats.favs).unwrap_or(0);
                self.library_info.publ = usize::try_from(stats.publ).unwrap_or(0);
                self.library_info.genre = usize::try_from(stats.genre).unwrap_or(0);
                self.library_info.lang = usize::try_from(stats.lang).unwrap_or(0);
                self.library_info.quote = stats.quote.clone();
                //
                self.library_info.rb_title = stats.rb_title.clone();
                self.library_info.rb_published = stats.rb_published.clone();
                self.library_info.rb_cover_color = match stats.rb_cover_color {
                    Some(value) => u32::try_from(value).unwrap_or(0x000000),
                    None => 0x000000,
                };

                self.library_info.r_author = stats.r_author.clone();
            }
            Ok(None) => {
                let msg = "no se encontra estadísticas";
                eprintln!("{}", msg);
            }
            Err(error) => {
                let msg = format!("Ocurrió un error, al consultar las estadísticas: {}", error);
                eprintln!("{}", msg);
            }
        }
        cx.notify();
    }
}
