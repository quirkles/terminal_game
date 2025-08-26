use crate::console::Console;
use crate::spatial::Coordinate;
use std::fmt::{Display, Formatter};

pub struct Particle {
    pub position: Coordinate, // In subpixel coordinates
    pub velocity: Coordinate, // In subpixel coordinates per frame
    pub acceleration: Coordinate,
}

impl Display for Particle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: ({}, {}). Velocity: ({}, {}). Acceleration: ({}, {})",
            self.position.x,
            self.position.y,
            self.velocity.x,
            self.velocity.y,
            self.acceleration.x,
            self.acceleration.y
        )
    }
}

impl Particle {
    pub fn new(
        position: Option<Coordinate>,
        velocity: Option<Coordinate>,
        acceleration: Option<Coordinate>,
    ) -> Self {
        Self {
            position: position.unwrap_or_default(),
            velocity: velocity.unwrap_or_default(),
            acceleration: acceleration.unwrap_or_default(),
        }
    }

    pub fn get_position(&self) -> Coordinate {
        self.position
    }

    pub fn set_acceleration(&mut self, acceleration: Coordinate) {
        self.acceleration = acceleration;
    }

    pub fn update(&mut self, console: &Console) {
        // self.velocity = self.velocity.add(&self.acceleration);
        self.velocity.add(&self.acceleration);
        self.position.add(&self.velocity);

        let as_cell = self.position.to_cell();
        if as_cell.y >= console.cell_height - 1 {
            self.position.y = console.height - (self.position.y - console.height).abs();
            self.velocity.y = self.velocity.y * -1;
        }
        if self.position.y <= 1 {
            self.position.y = self.position.y.abs();
            self.velocity.y = self.velocity.y * -1;
        }
        if as_cell.x >= (console.cell_width - 1) {
            self.position.x = console.width - (self.position.x - console.width).abs();
            self.velocity.x = self.velocity.x * -1;
        }
        if as_cell.x <= 1 {
            self.position.x = self.position.x.abs();
            self.velocity.x = self.velocity.x * -1;
        }
    }
}
