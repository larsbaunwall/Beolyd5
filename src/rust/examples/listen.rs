extern crate beolyd5_controller;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use beolyd5_controller::Beolyd5Controller;
use beolyd5_controller::types::{Button, SystemEvent, Wheel};


fn main() {
    let controller = Arc::new(Mutex::new(Beolyd5Controller::new()));
    let controller_clone = controller.clone();

    // Spawn a new thread
    thread::spawn(move || {
        // Register a callback to handle device events
        controller.lock().unwrap().register_device_event_callback(Arc::new(Mutex::new(move |event: SystemEvent| -> Result<(), Box<dyn Error + Send>> {
            println!("** Received SystemEvent: {:?}", event);

            Ok(())
        })));

        controller.lock().unwrap().register_wheel_event_callback(Arc::new(Mutex::new(move |(wheel, pos): (Wheel, u8)| -> Result<(), Box<dyn Error + Send>>  {
            println!("   Received WheelEvent: {:?} at position {}", wheel, pos);

            Ok(())
        })));

        controller.lock().unwrap().register_button_event_callback(Arc::new(Mutex::new(move |button: Button| -> Result<(), Box<dyn Error + Send>>  {
            println!("   Received ButtonEvent: {:?}", button);

            if button != Button::None {
                // Emit click sound
                controller_clone.lock().unwrap().tick().unwrap();
            }

            Ok(())
        })));

        // Open the device
        match controller.lock().unwrap().open() {
            Ok(_) => println!("Device opened successfully"),
            Err(err) => eprintln!("Failed to open device: {:?}", err),
        }

        // Keep the main thread alive to continue receiving events
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }).join().unwrap();
}
