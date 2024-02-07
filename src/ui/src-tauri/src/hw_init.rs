use std::sync::{Arc, Mutex};
use beolyd5_controller::{Beolyd5Controller, Button, Wheel};
use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct ButtonEvent {
    button: Button,
}

#[derive(Clone, serde::Serialize)]
struct WheelEvent {
    wheel: Wheel,
    position: u8,
}

#[derive(Clone, serde::Serialize)]
struct Diagnostics {
    messageType: String,
    message: String,
}

pub fn wire_up(app_handle: tauri::AppHandle, controller: Arc<Mutex<Beolyd5Controller>>) {

    // Create a new thread
    tauri::async_runtime::spawn(async move {
        let app_handle_clone = app_handle.clone();
        app_handle_clone.emit_all("wheelEvent", ()).unwrap();

        let app_handle_clone = app_handle.clone();
        controller.lock().unwrap().register_wheel_event_callback(Arc::new(Mutex::new(move |(wheel, pos): (Wheel, u8)| {
            let payload = WheelEvent {
                wheel: wheel,
                position: pos,
            };

            app_handle_clone.emit_all("wheelEvent", Some(payload)).unwrap();
        })));

        let app_handle_clone = app_handle.clone();
        controller.lock().unwrap().register_button_event_callback(Arc::new(Mutex::new(move |button: Button| {
            let payload = ButtonEvent {
                button: button,
            };

            app_handle_clone.emit_all("buttonEvent", Some(payload)).unwrap();
        })));

        let ctrl = controller.lock();

        let app_handle_clone = app_handle.clone();
        let _ = match ctrl.unwrap().open() {
            Ok(_) => app_handle_clone.emit_all("diagnostics", Diagnostics {
                messageType: "info".to_string(),
                message: "Device opened successfully".to_string(),
            }),
            Err(err) => app_handle.emit_all("diagnostics", Diagnostics {
                messageType: "error".to_string(),
                message: format!("Failed to open device: {:?}", err),
            }),
        };

        // Keep the main thread alive to continue receiving events
        loop {
        }
    });
}
