extern crate hidapi;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use hidapi::HidApi;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Button {
    None,
    Left,
    Right,
    Go,
    Standby
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Wheel {
    Front,
    Angular,
    Back,
    None
}

#[derive(Debug, Copy, Clone)]
pub struct SystemEvent {
    pub event_bytes: [u8; 6],
    pub last_read_bytes: [u8; 6],
    pub front_wheel_pos: u8,
    pub angular_wheel_pos: u8,
    pub back_wheel_pos: u8,
    pub button_pressed: Button
}

pub struct Beolyd5Controller {
    self_ref: Option<Arc<Self>>,
    threads: Vec<JoinHandle<()>>,
    vendor_id: u16,
    product_id: u16,
    last_read: Arc<Mutex<[u8; 6]>>,
    last_button_pressed: Arc<Mutex<Button>>,
    last_front_wheel_pos: Arc<Mutex<u8>>,
    last_angular_wheel_pos: Arc<Mutex<u8>>,
    last_back_wheel_pos: Arc<Mutex<u8>>,
    is_running: Arc<AtomicBool>,
    device_event_callbacks: Vec<Arc<Mutex<dyn Fn(SystemEvent) + Send>>>,
    wheel_event_callbacks: Vec<Arc<Mutex<dyn Fn((Wheel, u8)) + Send>>>,
    button_event_callbacks: Vec<Arc<Mutex<dyn Fn(Button) + Send>>>,
    device: Option<Arc<Mutex<hidapi::HidDevice>>>
}

impl Beolyd5Controller {
    pub fn new() -> Beolyd5Controller {
        Beolyd5Controller {
            self_ref: None,
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
            device: None
        }
    }

    pub fn open(&mut self) -> Result<(), Box<dyn Error>> {
        let is_running = self.is_running.clone();

        if self.device.is_none() {
            let api = HidApi::new()?;
            let device = api.open(self.vendor_id, self.product_id)?;
            self.device = Some(Arc::new(Mutex::new(device)));
        }

        self.is_running.store(true, Ordering::Relaxed);
        let device_clone = self.device.clone().unwrap();
        let self_ref = Arc::new(self.clone());
        self.self_ref = Some(self_ref.clone());

        let t = thread::spawn(move || {
            let mut buffer = [0u8; 6];
            while is_running.load(Ordering::Relaxed) {
                let device_lock = device_clone.lock().unwrap();
                let result = device_lock.read(&mut buffer[..]).unwrap();
                drop(device_lock);
                if result > 0 {
                    self_ref.handle_device_event(buffer);
                }
            }
        });
        self.threads.push(t);

        Ok(())
    }

    pub fn click(&self) -> Result<(), Box<dyn Error>> {
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
    fn send(&self, data: [u8; 2]) -> Result<(), Box<dyn Error>> {
        let device_clone = self.device.clone().unwrap();
        let device_lock = device_clone.lock().unwrap();
        device_lock.write(&data[..])?;
        Ok(())
    }

    pub fn close(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    pub fn register_device_event_callback(&mut self, callback: Arc<Mutex<dyn Fn(SystemEvent) + Send>>) {
        self.device_event_callbacks.push(callback);
    }

    pub fn register_wheel_event_callback(&mut self, callback: Arc<Mutex<dyn Fn((Wheel, u8)) + Send>>) {
        self.wheel_event_callbacks.push(callback);
    }

    pub fn register_button_event_callback(&mut self, callback: Arc<Mutex<dyn Fn(Button) + Send>>) {
        self.button_event_callbacks.push(callback);
    }

    fn handle_device_event(&self, event: [u8; 6]) {
        let _wheel_changed = self.handle_wheel_event(event);
        let _button_pressed = self.handle_button_event(event);

        let device_event_callbacks = self.device_event_callbacks.clone();
        let last_read_clone = self.last_read.lock().unwrap().clone();

        let button_pressed = match event[3] {
            0x00 => Button::None,
            0x20 => Button::Left,
            0x10 => Button::Right,
            0x40 => Button::Go,
            0x80 => Button::Standby,
            _ => Button::None
        };

        let top_wheel_pos = event[0];
        let angular_wheel_pos = event[2];
        let back_wheel_pos = event[1];

        if *self.last_button_pressed.lock().unwrap() != button_pressed ||
            *self.last_front_wheel_pos.lock().unwrap() != top_wheel_pos ||
            *self.last_angular_wheel_pos.lock().unwrap() != angular_wheel_pos ||
            *self.last_back_wheel_pos.lock().unwrap() != back_wheel_pos {

            let sys_event = SystemEvent {
                event_bytes: event,
                last_read_bytes: last_read_clone,
                front_wheel_pos: top_wheel_pos,
                back_wheel_pos: back_wheel_pos,
                angular_wheel_pos: angular_wheel_pos,
                button_pressed: button_pressed
            };

            for callback in &device_event_callbacks {
                let callback = callback.lock().unwrap();
                callback(sys_event.clone());
            }
            *self.last_read.lock().unwrap() = event;
            *self.last_button_pressed.lock().unwrap() = button_pressed;
            *self.last_front_wheel_pos.lock().unwrap() = top_wheel_pos;
            *self.last_angular_wheel_pos.lock().unwrap() = angular_wheel_pos;
            *self.last_back_wheel_pos.lock().unwrap() = back_wheel_pos;
        }
    }

    fn handle_wheel_event(&self, event: [u8; 6]) -> (Wheel, u8) {
        let top_wheel_pos = event[0];
        let angular_wheel_pos = event[2];
        let back_wheel_pos = event[1];

        let wheel_changed = if *self.last_front_wheel_pos.lock().unwrap() != top_wheel_pos {
            *self.last_front_wheel_pos.lock().unwrap() = top_wheel_pos;
            (Wheel::Front, top_wheel_pos)
        } else if *self.last_angular_wheel_pos.lock().unwrap() != angular_wheel_pos {
            *self.last_angular_wheel_pos.lock().unwrap() = angular_wheel_pos;
            (Wheel::Angular, angular_wheel_pos)
        } else if *self.last_back_wheel_pos.lock().unwrap() != back_wheel_pos {
            *self.last_back_wheel_pos.lock().unwrap() = back_wheel_pos;
            (Wheel::Back, back_wheel_pos)
        } else {
            (Wheel::None, 0)
        };
        if wheel_changed.0 != Wheel::None {
            for callback in &self.wheel_event_callbacks {
                let callback = callback.lock().unwrap();
                callback(wheel_changed);
            }
        }

        wheel_changed
    }

    fn handle_button_event(&self, event: [u8; 6]) -> Button {
        let button_pressed = match event[3] {
            0x00 => Button::None,
            0x20 => Button::Left,
            0x10 => Button::Right,
            0x40 => Button::Go,
            0x80 => Button::Standby,
            _ => Button::None
        };

        if *self.last_button_pressed.lock().unwrap() != button_pressed {
            *self.last_button_pressed.lock().unwrap() = button_pressed;
            for callback in &self.button_event_callbacks {
                let callback = callback.lock().unwrap();
                callback(button_pressed);
            }
        }

        button_pressed
    }
}

impl Drop for Beolyd5Controller {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        while let Some(thread) = self.threads.pop() {
            if let Err(err) = thread.join() {
                eprintln!("Failed to join thread: {:?}", err);
            }
        }
    }
}

impl Clone for Beolyd5Controller {
    fn clone(&self) -> Self {
        Beolyd5Controller {
            self_ref: None,
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
            device: self.device.clone()
        }
    }
}