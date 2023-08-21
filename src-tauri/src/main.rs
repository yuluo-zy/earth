// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod services;
mod plugins;

use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {

    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    info!("app init");

    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![greet])
        .system_tray(plugins::system_tray::create_tray())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}