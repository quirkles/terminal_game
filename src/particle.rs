use crate::console::Console;
use crate::spatial::Coordinate;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
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

    pub fn update(&mut self, console: &Console, velocity_cap: Coordinate) {
        // self.velocity = self.velocity.add(&self.acceleration);
        self.velocity.add(&self.acceleration);

        // Clamp velocity to the provided cap per axis
        self.velocity.x = self.velocity.x.clamp(-velocity_cap.x, velocity_cap.x);
        self.velocity.y = self.velocity.y.clamp(-velocity_cap.y, velocity_cap.y);

        // Apply half the velocity per tick for smoother motion at doubled tick rate
        self.position.add(&Coordinate::new(self.velocity.x / 2, self.velocity.y / 2));

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
