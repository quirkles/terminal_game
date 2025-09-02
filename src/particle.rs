use crate::console::Console;
use crate::spatial::Coordinate;
use crossterm::style::Color;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParticleType {
    Rocket,
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub position: Coordinate, // In subpixel coordinates
    pub velocity: Coordinate, // In subpixel coordinates per frame
    pub acceleration: Coordinate,
    pub color: Color,
    pub kind: ParticleType,
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
            color: Color::White,
            kind: ParticleType::Rocket,
        }
    }

    pub fn get_position(&self) -> Coordinate {
        self.position
    }

    pub fn set_acceleration(&mut self, acceleration: Coordinate) {
        self.acceleration = acceleration;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn update(&mut self, console: &Console, velocity_cap: Coordinate) {
        // self.velocity = self.velocity.add(&self.acceleration);
        self.velocity.add(&self.acceleration);

        // Clamp velocity to the provided cap per axis
        self.velocity.x = self.velocity.x.clamp(-velocity_cap.x, velocity_cap.x);
        self.velocity.y = self.velocity.y.clamp(-velocity_cap.y, velocity_cap.y);

        // Apply half the velocity per tick for smoother motion at doubled tick rate
        self.position
            .add(&Coordinate::new(self.velocity.x / 2, self.velocity.y / 2));

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

    pub fn get_particle_char(&self) -> char {
        match self.kind {
            ParticleType::Rocket => self.get_rocket_char(),
        }
    }

    fn get_rocket_char(&self) -> char {
        let vx = self.velocity.x as f32;
        let vy = self.velocity.y as f32;

        if vx == 0.0 && vy == 0.0 {
            return '•';
        }

        // Angle from -PI..PI, convert to 0..2PI and quantize to 8 sectors
        let mut ang = vy.atan2(vx);
        if ang < 0.0 {
            ang += std::f32::consts::PI * 2.0;
        }

        // Round to nearest sector (PI/4 each)
        let sector =
            ((ang + std::f32::consts::PI / 8.0) / (std::f32::consts::PI / 4.0)).floor() as i32 % 8;

        match sector {
            0 => '→', // East
            1 => '↘', // South-East
            2 => '↓', // South
            3 => '↙', // North-West
            4 => '←', // West
            5 => '↖', // South-West
            6 => '↑', // North
            _ => '↗', // North-East
        }
    }
}
