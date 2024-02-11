// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hw_controller;

use std::sync::Mutex;
use hw_controller::HWController;
use tauri::{generate_context, Manager};

// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn click(state: tauri::State<'_, Mutex<HWController>>) -> Result<(), ()> {
  let controller = match state.lock() {
    Ok(controller) => controller,
    Err(_) => return Err(()),
  };
  controller.click();

  Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![click])
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
