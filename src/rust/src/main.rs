use std::sync::{Arc, Mutex};
use beolyd5_controller::Beolyd5Controller;
pub mod beolyd5_controller;

fn main() {
    let mut controller = Beolyd5Controller::new();

    // Register a callback to handle device events
    controller.register_device_event_callback(Arc::new(Mutex::new(|event, last_read| {
        println!("Received event: {:?}, Last read: {:?}", event, last_read);
    })));

    // Open the device
    match controller.open() {
        Ok(_) => println!("Device opened successfully"),
        Err(err) => eprintln!("Failed to open device: {:?}", err),
    }

    // Keep the main thread alive to continue receiving events
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
