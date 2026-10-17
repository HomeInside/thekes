use gpui_kit::Context;

use super::Dashboard;

//acciones/eventos
impl Dashboard {
    pub fn click_button_demo(&mut self, cx: &mut Context<Self>) {
        println!("click_button_demo fn");
        cx.notify();
    }
}
