use std::sync::mpsc;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
pub enum AsyncProcessMessage {
    StartRotate,
    StopRotate,
    PreviousPhoto,
    NextPhoto,
}
pub fn create_tray() -> SystemTray {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    // let auto_rotate = CustomMenuItem::new("auto_rotate".to_string(), "Auto Rotate");
    let previous_photo = CustomMenuItem::new("previous_photo".to_string(), "Previous Photo");
    let next_photo = CustomMenuItem::new("next_photo".to_string(), "Next Photo");

    let tray_menu = SystemTrayMenu::new()
        // .add_item(auto_rotate)
        .add_item(previous_photo)
        .add_item(next_photo)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}


pub fn handle_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
    sender: mpsc::Sender<AsyncProcessMessage>,
) {
    match event {
        SystemTrayEvent::DoubleClick {
            tray_id,
            position,
            size,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "previous_photo" => {
                let tx = sender.clone();
                // tokio::spawn(async move {
                //     tx.send(AsyncProcessMessage::PreviousPhoto).await.unwrap();
                // });
            }
            "next_photo" => {
                let tx = sender.clone();
                // tokio::spawn(async move {
                //     tx.send(AsyncProcessMessage::NextPhoto).await.unwrap();
                //     println!("send");
                // });
            }
            "show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    };
}