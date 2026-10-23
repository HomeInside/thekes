use gpui_kit::IntoElement;
use gpui_kit::Styled;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::WindowExt;
use gpui_kit::component::button::ButtonVariant;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::{Context, Window};
use std::{cell::RefCell, rc::Rc};

/// Una función libre que abre un diálogo de confirmación y ejecuta
/// un callback cuando el usuario confirma ('Yes').
///
/// El callback recibe acceso mutable a la entidad de origen y solo se
/// ejecuta si la entidad sigue existiendo cuando el usuario confirma.
///
pub fn confirm<EntityC: 'static>(
    title: &'static str,
    description: impl IntoElement + Clone + 'static,
    window: &mut Window,
    cx: &mut Context<EntityC>,
    on_confirm: impl Fn(&mut EntityC, &mut Window, &mut Context<EntityC>) + 'static,
) {
    let entity = cx.entity().downgrade();
    let on_confirm = Rc::new(RefCell::new(on_confirm));

    window.open_alert_dialog(cx, {
        let on_confirm = Rc::clone(&on_confirm);
        let description = description.clone();

        move |alert, _, cx| {
            let entity = entity.clone();
            let on_confirm = Rc::clone(&on_confirm);

            alert
                .title(title)
                .description(description.clone())
                .confirm()
                .ok_text("Yes")
                .ok_variant(ButtonVariant::Danger)
                .cancel_text("No")
                .cancel_variant(ButtonVariant::Secondary)
                .icon(
                    Icon::new(IconName::TriangleAlert)
                        .size_8()
                        .text_color(cx.theme().warning),
                )
                .close_button(true)
                //
                .on_ok(move |_, window, app| {
                    let callback = Rc::clone(&on_confirm);

                    if let Some(entity) = entity.upgrade() {
                        entity.update(app, |this, cx| {
                            (callback.borrow_mut())(this, window, cx);
                        });
                    }

                    true
                })
        }
    });
} //confirm
