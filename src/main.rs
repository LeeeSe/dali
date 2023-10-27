use std::thread;

use dali::{choose_video_file, compress_video};
use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_choose_video_file({
        let ui = ui_handle.unwrap();
        move || {
            let input = choose_video_file();
            if !input.is_none() {
                ui.set_compress_enabled(true);
                ui.set_compress_video_input_file_path(SharedString::from(
                    input.unwrap().to_str().unwrap(),
                ));
            }
        }
    });

    ui.on_compress_video({
        let ui_handle = ui_handle.clone(); // clone the weak to move it to the callback
        move || {
            let (tx, rx) = std::sync::mpsc::channel::<f32>();
            // can unwrap in the callback, that's fine
            let input = ui_handle
                .unwrap()
                .get_compress_video_input_file_path()
                .clone();
            let input = std::path::Path::new(input.as_str());
            let binding = ui_handle.unwrap().get_video_type().clone().to_owned();
            let video_type = binding.as_str();
            compress_video(input, video_type, &tx);
            let ui_handle = ui_handle.clone(); // clone it again to move it to the thread
            thread::spawn(move || {
                while let Ok(progress) = rx.recv() {
                    // use upgrade_in_event_loop to get a callback in the main thread again.
                    ui_handle
                        .upgrade_in_event_loop(move |ui| {
                            ui.set_compressing_progress_value(progress)
                        })
                        .expect("ui 更新失败");
                }
            });
        }
    });

    ui.run()
}
