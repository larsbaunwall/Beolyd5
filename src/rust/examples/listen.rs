extern crate beolyd5_controller;

use std::sync::{Arc, Mutex};
use beolyd5_controller::{Beolyd5Controller, SystemEvent};


fn main() {
    let mut controller = Beolyd5Controller::new();

    // Register a callback to handle device events
    controller.register_device_event_callback(Arc::new(Mutex::new(|event: SystemEvent| {
        println!("Received event: {:?}", event);
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
