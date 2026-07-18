use beolyd5_controller::types::{Button, Wheel};
use beolyd5_controller::Beolyd5Controller;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

#[derive(Clone)]
pub struct HWController {
    app_handle: Option<tauri::AppHandle>,
    controller: Arc<Mutex<Beolyd5Controller>>,
}

impl Default for HWController {
    fn default() -> Self {
        HWController {
            app_handle: None,
            controller: Arc::new(Mutex::new(Beolyd5Controller::new())),
        }
    }
}

impl HWController {
    pub fn new(app_handle: AppHandle) -> Self {
        HWController {
            app_handle: Some(app_handle),
            controller: Arc::new(Mutex::new(Beolyd5Controller::new())),
        }
    }

    pub fn init(&self) {
        let controller_clone = self.controller.clone();
        let app_handle = self.app_handle.clone().unwrap();

        tauri::async_runtime::spawn(async move {
            let app_handle_clone = app_handle.clone();
            controller_clone
                .lock()
                .unwrap()
                .register_wheel_event_callback(Arc::new(Mutex::new(
                    move |(wheel, pos): (Wheel, u8)| {
                        let payload = HardwareEvent {
                            kind: "wheel".to_string(),
                            source: wheel.to_string(),
                            value: pos,
                        };

                        app_handle_clone.emit("hardwareEvent", Some(payload)).unwrap();

                        Ok(())
                    },
                )));

            let app_handle_clone = app_handle.clone();
            controller_clone
                .lock()
                .unwrap()
                .register_button_event_callback(Arc::new(Mutex::new(move |button: Button| {
                    let payload = HardwareEvent {
                        kind: "button".to_string(),
                        source: button.to_string(),
                        value: 0,
                    };

                    app_handle_clone.emit("hardwareEvent", Some(payload)).unwrap();

                    Ok(())
                })));

            let app_handle_clone = app_handle.clone();
            let _ = match controller_clone.lock().unwrap().open() {
                Ok(_) => app_handle_clone.emit(
                    "diagnostics",
                    Diagnostics {
                        message_type: "info".to_string(),
                        message: "Device opened successfully".to_string(),
                    },
                ),
                Err(err) => app_handle_clone.emit(
                    "diagnostics",
                    Diagnostics {
                        message_type: "error".to_string(),
                        message: format!("Failed to open device: {:?}", err),
                    },
                ),
            };

            std::future::pending::<()>().await;
        });
    }

    pub fn tick(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.controller.lock().unwrap().tick()?;
        Ok(())
    }
}
#[derive(Clone, serde::Serialize)]
struct HardwareEvent {
    kind: String,
    source: String,
    value: u8,
}

#[derive(Clone, serde::Serialize)]
struct Diagnostics {
    message_type: String,
    message: String,
}
