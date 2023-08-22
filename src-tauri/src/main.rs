// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod services;
mod plugins;

use tauri::async_runtime::Mutex;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use crate::services::storage::Storage;

use crate::services::cmd::get_photo;

fn main() {

    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    info!("app init");

    tauri::Builder::default()
        .manage(Mutex::new(Storage::default()))
        .invoke_handler(tauri::generate_handler![
            get_photo,
        ])
        .system_tray(plugins::system_tray::create_tray())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}