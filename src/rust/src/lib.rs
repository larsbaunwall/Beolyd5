/*
 * Copyright (c) 2024. Lars Baunwall. All rights reserved.
 * Use of this source code is governed by an Apache 2.0 license that can be found in the LICENSE file.
 */


use hidapi::HidApi;
use std::error::Error;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use types::{Button, SystemEvent, Wheel};

pub mod types;

/// `Beolyd5Controller` is a struct that represents a BeoSound 5 controller.
/// It provides methods to open the device, send commands, and register callbacks for device events.
pub struct Beolyd5Controller {
    threads: Vec<JoinHandle<Result<(), Box<dyn Error + Send>>>>,
    vendor_id: u16,
    product_id: u16,
    last_read: Arc<Mutex<[u8; 6]>>,
    last_button_pressed: Arc<Mutex<Button>>,
    last_front_wheel_pos: Arc<Mutex<u8>>,
    last_angular_wheel_pos: Arc<Mutex<u8>>,
    last_back_wheel_pos: Arc<Mutex<u8>>,
    is_running: Arc<AtomicBool>,
    device_event_callbacks: Vec<Arc<Mutex<dyn Fn(SystemEvent) -> Result<(), Box<dyn Error + Send>> + Send>>>,
    wheel_event_callbacks: Vec<Arc<Mutex<dyn Fn((Wheel, u8))-> Result<(), Box<dyn Error + Send>> + Send>>>,
    button_event_callbacks: Vec<Arc<Mutex<dyn Fn(Button)-> Result<(), Box<dyn Error + Send>> + Send>>>,
    device: Option<Arc<Mutex<hidapi::HidDevice>>>,
}

impl Beolyd5Controller {
    /// Creates a new `Beolyd5Controller` without opening it.
    pub fn new() -> Beolyd5Controller {
        Beolyd5Controller {
            threads: Vec::new(),
            vendor_id: 0x0cd4,
            product_id: 0x1112,
            last_read: Arc::new(Mutex::new([0u8; 6])),
            last_button_pressed: Arc::new(Mutex::new(Button::None)),
            last_front_wheel_pos: Arc::new(Mutex::new(0)),
            last_angular_wheel_pos: Arc::new(Mutex::new(0)),
            last_back_wheel_pos: Arc::new(Mutex::new(0)),
            is_running: Arc::new(AtomicBool::new(false)),
            device_event_callbacks: Vec::new(),
            wheel_event_callbacks: Vec::new(),
            button_event_callbacks: Vec::new(),
            device: None,
        }
    }

    /// Opens the device and starts a new thread to handle device events.
    /// Returns `Ok(())` if the device was opened successfully, or an `Err` if the device could not be found or accessed.
    pub fn open(&mut self) -> Result<(), Box<dyn Error>> {
        let is_running = self.is_running.clone();

        if self.device.is_none() {
            let api = HidApi::new()?;
            let device = match api.open(self.vendor_id, self.product_id) {
                Ok(device) => device,
                Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "BS5 controller not found"))),
            };
            self.device = Some(Arc::new(Mutex::new(device)));
        }

        self.is_running.store(true, Ordering::Relaxed);
        let device_clone = self.device.clone().unwrap();
        let self_ref = Arc::new(self.clone());

        let t = thread::spawn(move || -> Result<(), Box<dyn Error + Send>> {
            let mut buffer = [0u8; 6];
            while is_running.load(Ordering::Relaxed) {
                let device_lock = device_clone.lock().unwrap();
                let result = device_lock.read(&mut buffer[..]).unwrap();
                drop(device_lock);
                if result > 0 {
                    self_ref.handle_device_event(buffer)?;
                }
            }

            Ok(())
        });
        self.threads.push(t);

        Ok(())
    }

    /// Sends a tick command (the sound!) to the device.
    /// Returns `Ok(())` if the command was sent successfully, or an `Err` if there was a problem sending the command.
    pub fn tick(&self) -> Result<(), Box<dyn Error>> {
        self.send([0x00, 0x31])
    }

    // From: https://github.com/toresbe/neomaster/blob/master/ui/panel.cpp
    /*
     *   0x80:   LED solid
     *   0x40:   LCD backlight
     *   0x20:   ?
     *   0x10:   LED blink
     *   0x0x:   Any bits here will trigger a click
     *           seems to make no difference which or how many
     *
     *  Byte 2:
     *   0x80:   Is set when IR receiver is on
     *   0x40:   Is sometimes set when IR receiver is on?
     *
     */
    //SETTING_CLICK = 0x01
    //uint8_t bar [2] = { 0x00, 0x00 }; // turns off
    //uint8_t bar [2] = { 0x40, 0x00 }; // turns on backlight
    //uint8_t bar [2] = { 0xc0, 0x00 }; // turns on LED
    //uint8_t bar [2] = { 0x80, 0x00 }; // turns off screen, on LED
    //uint8_t bar [2] = { 0xd0, 0x00 }; //  blinking
    
    /// Sends a command to the device to turn on the LCD backlight or the LED.
    /// Commands _could_ be:
    /// - `[0x00, 0x00]` to turn off the LCD backlight and the LED
    /// - `[0x40, 0x00]` to turn on the LCD backlight
    /// - `[0xc0, 0x00]` to turn on the LED
    /// - `[0x80, 0x00]` to turn off the LCD backlight and turn on the LED
    /// - `[0xd0, 0x00]` to make the LED blink
    /// - `[0x01, 0x00]` to make a click sound
    /// Returns `Ok(())` if the command was sent successfully, or an `Err` if there was a problem sending the command.
    pub fn send(&self, data: [u8; 2]) -> Result<(), Box<dyn Error>> {
        let device_clone = self.device.clone().ok_or_else(|| {
            Box::new(std::io::Error::new(
                ErrorKind::NotFound,
                "BS5 controller not found or not accessible",
            ))
        })?;
        let device_lock = device_clone.lock().unwrap();
        device_lock.write(&data[..])?;

        Ok(())
    }

    /// Closes the device and stops handling device events.
    pub fn close(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    /// Registers a callback to be called when any device event occurs.
    pub fn register_device_event_callback(
        &mut self,
        callback: Arc<Mutex<dyn Fn(SystemEvent) -> Result<(), Box<dyn Error + Send>> + Send>>,
    ) {
        self.device_event_callbacks.push(callback);
    }

    /// Registers a callback to be called when a wheel event occurs.
    pub fn register_wheel_event_callback(
        &mut self,
        callback: Arc<Mutex<dyn Fn((Wheel, u8)) -> Result<(), Box<dyn Error + Send>> + Send>>,
    ) {
        self.wheel_event_callbacks.push(callback);
    }

    /// Registers a callback to be called when a button event occurs.
    pub fn register_button_event_callback(&mut self, callback: Arc<Mutex<dyn Fn(Button) -> Result<(), Box<dyn Error + Send>> + Send>>) {
        self.button_event_callbacks.push(callback);
    }

    fn handle_device_event(&self, event: [u8; 6]) -> Result<(), Box<dyn Error + Send>> {
        let wheel_changed = Self::get_wheel_moved(event, self.last_read.lock().unwrap().clone());
        let button_pressed = Self::get_button_pressed(event);

        let top_wheel_pos = event[0];
        let angular_wheel_pos = event[2];
        let back_wheel_pos = event[1];

        if wheel_changed.0 != Wheel::None {
            self.handle_wheel_event(event)?;
        } else if button_pressed != Button::None {
            self.handle_button_event(event)?;
        }

        *self.last_read.lock().unwrap() = event;
        *self.last_button_pressed.lock().unwrap() = button_pressed;
        *self.last_front_wheel_pos.lock().unwrap() = top_wheel_pos;
        *self.last_angular_wheel_pos.lock().unwrap() = angular_wheel_pos;
        *self.last_back_wheel_pos.lock().unwrap() = back_wheel_pos;

        let device_event_callbacks = self.device_event_callbacks.clone();
        let last_read_clone = self.last_read.lock().unwrap().clone();
        let sys_event = SystemEvent {
            event_bytes: event,
            last_read_bytes: last_read_clone,
            front_wheel_pos: top_wheel_pos,
            back_wheel_pos,
            angular_wheel_pos,
            button_pressed,
        };

        for callback in &device_event_callbacks {
            let callback = callback.lock().unwrap();
            callback(sys_event.clone())?;
        }

        Ok(())
    }

    fn handle_wheel_event(&self, event: [u8; 6]) -> Result<(), Box<dyn Error + Send>> {
        let wheel_changed = Self::get_wheel_moved(event, self.last_read.lock().unwrap().clone());
        if wheel_changed.0 != Wheel::None {
            for callback in &self.wheel_event_callbacks {
                let callback = callback.lock().unwrap();
                callback(wheel_changed)?;
            }
        }

        Ok(())
    }

    /*
     * Front and back wheels are only untouched if they are 0
     * Angular wheel is only untouched if it is the same as the last reading
     */
    fn get_wheel_moved(event: [u8; 6], last_read: [u8; 6]) -> (Wheel, u8) {
        let front_wheel_pos = event[0];
        let angular_wheel_pos = event[2];
        let back_wheel_pos = event[1];

        return if front_wheel_pos != 0 {
            (Wheel::Front, front_wheel_pos)
        } else if last_read[2] != angular_wheel_pos {
            (Wheel::Angular, angular_wheel_pos)
        } else if back_wheel_pos != 0 {
            (Wheel::Back, back_wheel_pos)
        } else {
            (Wheel::None, 0)
        };
    }

    fn handle_button_event(&self, event: [u8; 6]) -> Result<(), Box<dyn Error + Send>> {
        let button_pressed = Self::get_button_pressed(event);

        if *self.last_button_pressed.lock().unwrap() != button_pressed {
            *self.last_button_pressed.lock().unwrap() = button_pressed;
            for callback in &self.button_event_callbacks {
                let callback = callback.lock().unwrap();
                callback(button_pressed)?;
            }
        }

        Ok(())
    }

    fn get_button_pressed(event: [u8; 6]) -> Button {
        return match event[3] {
            0x00 => Button::None,
            0x20 => Button::Left,
            0x10 => Button::Right,
            0x40 => Button::Go,
            0x80 => Button::Standby,
            _ => Button::None,
        };
    }
}

impl Drop for Beolyd5Controller {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        while let Some(thread) = self.threads.pop() {
            match thread.join() {
                Ok(res) => match res {
                    Ok(_) => (),
                    Err(err) => eprintln!("Error in thread: {:?}", err),
                },
                Err(err) => eprintln!("Failed to join thread: {:?}", err),
            }
        }
    }
}

impl Clone for Beolyd5Controller {
    fn clone(&self) -> Self {
        Beolyd5Controller {
            threads: Vec::new(),
            vendor_id: self.vendor_id,
            product_id: self.product_id,
            last_read: self.last_read.clone(),
            last_button_pressed: self.last_button_pressed.clone(),
            last_front_wheel_pos: self.last_front_wheel_pos.clone(),
            last_angular_wheel_pos: self.last_angular_wheel_pos.clone(),
            last_back_wheel_pos: self.last_back_wheel_pos.clone(),
            is_running: self.is_running.clone(),
            device_event_callbacks: self.device_event_callbacks.clone(),
            wheel_event_callbacks: self.wheel_event_callbacks.clone(),
            button_event_callbacks: self.button_event_callbacks.clone(),
            device: self.device.clone(),
        }
    }
}
