extern crate beolyd5_controller;

use std::sync::{Arc, Mutex};
use beolyd5_controller::{Beolyd5Controller, Button, SystemEvent, Wheel};


fn main() {
    let controller = Arc::new(Mutex::new(Beolyd5Controller::new()));
    let controller_clone = Arc::clone(&controller);

    // Register a callback to handle device events
    controller.lock().unwrap().register_device_event_callback(Arc::new(Mutex::new(move |event: SystemEvent| {
        println!("** Received SystemEvent: {:?}", event);

        // Emit click sound
        //controller_clone.lock().unwrap().click().unwrap();
    })));

    controller.lock().unwrap().register_wheel_event_callback(Arc::new(Mutex::new(move |(wheel, pos): (Wheel, u8)| {
        println!("   Received WheelEvent: {:?} at position {}", wheel, pos);
    })));

    controller.lock().unwrap().register_button_event_callback(Arc::new(Mutex::new(move |button: Button| {
        println!("   Received ButtonEvent: {:?}", button);
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
