// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hw_init;

#[tauri::command]
fn tick() {
  println!("Tick");
}

use std::sync::{Arc, Mutex};
use beolyd5_controller::{Beolyd5Controller};
use tauri::generate_context;

fn main() {
    let controller = Arc::new(Mutex::new(Beolyd5Controller::new()));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tick])
        .setup(move |app| {
            let app_handle = app.handle();
            hw_init::wire_up(app_handle, controller.clone());

            Ok(())
          })
        .run(generate_context!())
        .expect("error while running BS5 controller UI application");
}
