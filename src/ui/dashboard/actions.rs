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
