extern crate beolyd5_controller;

use std::sync::{Arc, Mutex};
use beolyd5_controller::{Beolyd5Controller, SystemEvent};


fn main() {
    let controller = Arc::new(Mutex::new(Beolyd5Controller::new()));
    let controller_clone = Arc::clone(&controller);

    // Register a callback to handle device events
    controller.lock().unwrap().register_device_event_callback(Arc::new(Mutex::new(move |event: SystemEvent| {
        println!("Received event: {:?}", event);

        // Emit click sound
        //controller_clone.lock().unwrap().click().unwrap();
    })));

    // Open the device
    match controller.lock().unwrap().open() {
        Ok(_) => println!("Device opened successfully"),
        Err(err) => eprintln!("Failed to open device: {:?}", err),
    }

    // Keep the main thread alive to continue receiving events
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
