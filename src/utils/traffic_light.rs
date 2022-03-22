use std::fmt::Display;
use crate::Light;

pub struct TrafficLight {
    pub color: TrafficLightColor,
}

impl Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.color
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffic light is {}", self.color)
    }
}

impl Display for TrafficLightColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_string = match self {
            TrafficLightColor::Green => "green",
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
        };
        write!(f, "Traffic light is {}", color_string)
    }
}

#[derive(Debug)]
pub enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }

    pub fn get_state(&self) -> &TrafficLightColor {
        &self.color
    }

    pub fn turn_green(&mut self) {
        self.color = TrafficLightColor::Green
    }

    pub fn turn_yellow(&mut self) {
        self.color = TrafficLightColor::Yellow
    }
}