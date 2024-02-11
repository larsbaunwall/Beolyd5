// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hw_controller;

use std::sync::Mutex;
use hw_controller::HWController;
use tauri::{generate_context, Manager};


#[tauri::command]
async fn tick(state: tauri::State<'_, Mutex<HWController>>) -> Result<(), ()> {
  let controller = match state.lock() {
    Ok(controller) => controller,
    Err(_) => return Err(()),
  };
  controller.tick().expect("TODO: panic message");

  Ok(())
}

fn main() {
    tauri::Builder::default()
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
