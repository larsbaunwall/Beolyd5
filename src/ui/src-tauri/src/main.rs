// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn tick() {
  println!("Tick");
}

use std::sync::{Arc, Mutex};
use beolyd5_controller::{Beolyd5Controller, Button, Wheel};
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
            let app_handle = app.handle();
            let controller = Arc::new(Mutex::new(Beolyd5Controller::new()));
            let controller_clone = Arc::clone(&controller);

            let app_handle_clone = app_handle.clone();
            app_handle_clone.emit_all("wheelEvent", ()).unwrap();

            let _ = match controller_clone.lock().unwrap().open() {
                Ok(_) => app_handle.emit_all("deviceOpened", ()),
                Err(err) => app_handle.emit_all("error", Payload {
                    message: format!("Failed to open device: {:?}", err),
                }),
            };

            let app_handle_clone = app_handle.clone();
            controller_clone.lock().unwrap().register_wheel_event_callback(Arc::new(Mutex::new(move |(wheel, pos): (Wheel, u8)| {
                let payload = Payload {
                    message: format!("Received WheelEvent: {:?} at position {}", wheel, pos),
                };
                app_handle_clone.emit_all("wheelEvent", Some(payload)).unwrap();
            })));

            let app_handle_clone = app_handle.clone();
            controller_clone.lock().unwrap().register_button_event_callback(Arc::new(Mutex::new(move |button: Button| {
                let payload = Payload {
                    message: format!("Received ButtonEvent: {:?}", button),
                };
                app_handle_clone.emit_all("buttonEvent", Some(payload)).unwrap();
            })));

            Ok(())
          })
        .run(tauri::generate_context!())
        .expect("error while running BS5 controller UI application");
}

