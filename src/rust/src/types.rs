#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Button {
    None,
    Left,
    Right,
    Go,
    Standby,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Wheel {
    Front,
    Angular,
    Back,
    None,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemEvent {
    pub event_bytes: [u8; 6],
    pub last_read_bytes: [u8; 6],
    pub front_wheel_pos: u8,
    pub angular_wheel_pos: u8,
    pub back_wheel_pos: u8,
    pub button_pressed: Button,
}
