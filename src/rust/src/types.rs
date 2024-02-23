/*
 * Copyright (c) 2024. Lars Baunwall. All rights reserved.
 * Use of this source code is governed by an Apache 2.0 license that can be found in the LICENSE file.
 */

use std::fmt;

/// `Button` represents one of the four buttons on the BeoSound 5 controller.
#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Button {
    None,
    Left,
    Right,
    Go,
    Standby,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Button::None => write!(f, "None"),
            Button::Left => write!(f, "Left"),
            Button::Right => write!(f, "Right"),
            Button::Go => write!(f, "Go"),
            Button::Standby => write!(f, "Standby"),
        }
    }
}

/// `Wheel` represents one of the three wheels on the BeoSound 5 controller.
#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Wheel {
    Front,
    Angular,
    Back,
    None,
}

impl fmt::Display for Wheel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Wheel::Front => write!(f, "Front"),
            Wheel::Angular => write!(f, "Angular"),
            Wheel::Back => write!(f, "Back"),
            Wheel::None => write!(f, "None"),
        }
    }
}

/// `SystemEvent` represents a system event (any event) from the BeoSound 5 controller.
/// It includes the event bytes, the last read bytes, the positions of the wheels, and the button pressed.
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemEvent {
    pub event_bytes: [u8; 6],
    pub last_read_bytes: [u8; 6],
    pub front_wheel_pos: u8,
    pub angular_wheel_pos: u8,
    pub back_wheel_pos: u8,
    pub button_pressed: Button,
}
