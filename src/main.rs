#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod llm;
use dali::{ui, MessageList, Sender};
use i_slint_backend_winit::winit::{platform::macos::WindowBuilderExtMacOS, window::WindowButtons};
use std::sync::Arc;
use ui::*;

fn main() -> Result<(), slint::PlatformError> {
    let mut backend = i_slint_backend_winit::Backend::new()?;
    backend.window_builder_hook = Some(Box::new(|builder| {
        builder
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
    }));
    slint::platform::set_platform(Box::new(backend)).unwrap();

    let ui = AppWindow::new()?;
    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());

    let msg_list = Arc::new(MessageList::default());
    let current_service = ui.get_current_service().to_string();
    msg_list.set_current_service(current_service);

    ui.set_msgs(msg_list.to_model_rc());

    ui.on_get_input({
        let msg_list = msg_list.clone();
        let ui_handle = ui.as_weak();
        move |input: slint::SharedString| {
            msg_list.add_message(Sender::User, input.to_string());

            let tokio_runtime = tokio_runtime.clone();
            let msg_list_clone = msg_list.clone();
            let ui_handle_clone = ui_handle.clone();

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_msgs(msg_list.to_model_rc());
            }

            let _ = slint::spawn_local(async move {
                let _result = tokio_runtime
                    .spawn(async move {
                        msg_list_clone.get_response_stream(ui_handle_clone).await;
                    })
                    .await
                    .expect("get_response failed");
            })
            .unwrap();
        }
    });

    ui.on_clear({
        let msg_list = msg_list.clone();
        let ui_handle = ui.as_weak();
        move || {
            msg_list.clear();
            ui_handle.unwrap().set_msgs(msg_list.to_model_rc());
        }
    });

    ui.on_current_item_changed({
        let msg_list = msg_list.clone();
        let ui_handle = ui.as_weak();
        move || {
            let current_service = ui_handle.unwrap().get_current_service().to_string();

            msg_list.set_current_service(current_service);
            msg_list.clear();
            ui_handle.unwrap().set_msgs(msg_list.to_model_rc());
            ui_handle.unwrap().set_scroll_y(0.0);
        }
    });

    ui.run()
}
