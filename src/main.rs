use dali::{MessageList, Sender};
use std::sync::Arc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());

    let msg_list = Arc::new(MessageList::default());

    ui.set_msgs(msg_list.to_model_rc());

    ui.on_get_input({
        let msg_list = msg_list.clone();
        let ui_handle = ui.as_weak();
        move |input: slint::SharedString| {
            msg_list.add_message(Sender::User, input.to_string());

            let tokio_runtime = tokio_runtime.clone();
            let msg_list_clone = msg_list.clone();
            let msg_list_clone2 = msg_list.clone();
            let ui_handle_clone = ui_handle.clone();

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_msgs(msg_list.to_model_rc());
            }

            let _ = slint::spawn_local(async move {
                let _result = tokio_runtime
                    .spawn(async move {
                        msg_list_clone.get_response().await;
                    })
                    .await
                    .expect("get_response failed");

                if let Some(ui) = ui_handle_clone.upgrade() {
                    ui.set_msgs(msg_list_clone2.to_model_rc());
                }
            })
            .unwrap();
        }
    });

    ui.on_clear({
        let msg_list = msg_list.clone();
        let ui_handle = ui.as_weak();
        move || {
            msg_list.clear();
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_msgs(msg_list.to_model_rc());
            }
        }
    });

    ui.run()
}
