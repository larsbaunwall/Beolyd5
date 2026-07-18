// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hw_controller;

use hw_controller::HWController;
use std::sync::Mutex;
use tauri::{generate_context, Manager};

#[tauri::command]
async fn tick(state: tauri::State<'_, Mutex<HWController>>) -> Result<(), String> {
    let controller = match state.lock() {
        Ok(controller) => controller,
        Err(err) => return Err(format!("Failed to lock hardware controller: {err}")),
    };
    controller
        .tick()
        .map_err(|err| format!("Failed to tick hardware controller: {err}"))?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![tick])
        .setup(|app| {
            let app_handle = app.handle();
            let hw: HWController = HWController::new(app_handle.clone());
            app.manage(Mutex::new(hw.clone()));

            hw.init();

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running BS5 controller UI application");
}
