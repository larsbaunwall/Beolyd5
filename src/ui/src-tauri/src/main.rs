// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn tick() {
  println!("Tick");
}

use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tick])
        .setup(|app| {

            // emit the `event-name` event to all webview windows on the frontend
            app.emit_all("navWheelUpdated", ()).unwrap();
            Ok(())
          })
        .run(tauri::generate_context!())
        .expect("error while running BS5 control application");
}


