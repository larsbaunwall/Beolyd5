use std::sync::{Arc, Mutex};
use beolyd5_controller::Beolyd5Controller;
use beolyd5_controller::types::{Button, Wheel};
use tauri::{AppHandle, Manager};

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
            controller_clone.lock().unwrap().register_wheel_event_callback(Arc::new(Mutex::new(move |(wheel, pos): (Wheel, u8)| {
                let payload = WheelEvent {
                    wheel,
                    position: pos,
                };
    
                app_handle_clone.emit_all("wheelEvent", Some(payload)).unwrap();

                Ok(())
            })));
    
            let app_handle_clone = app_handle.clone();
            controller_clone.lock().unwrap().register_button_event_callback(Arc::new(Mutex::new(move |button: Button| {
                let payload = ButtonEvent {
                    button,
                };
    
                app_handle_clone.emit_all("buttonEvent", Some(payload)).unwrap();

                Ok(())
            })));
    

            let app_handle_clone = app_handle.clone();
            let _ = match controller_clone.lock().unwrap().open() {
                Ok(_) => app_handle_clone.emit_all("diagnostics", Diagnostics {
                    message_type: "info".to_string(),
                    message: "Device opened successfully".to_string(),
                }),
                Err(err) => app_handle_clone.emit_all("diagnostics", Diagnostics {
                    message_type: "error".to_string(),
                    message: format!("Failed to open device: {:?}", err),
                }),
            };
    
            // Keep the main thread alive to continue receiving events
            loop {
            }
        });
    }

    pub fn tick(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.controller.lock().unwrap().tick()?;
        Ok(())
    }
}
    


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
    message_type: String,
    message: String,
}