use crate::spatial::{Coordinate, ConsoleCell};
use crossterm::style::Color;
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ParticleId(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParticleType {
    Rocket,
    FuelCell,
}

#[derive(Copy, Clone, Debug)]
pub struct ParticleColors {
    pub foreground: Color,
    pub background: Color,
}

#[derive(Clone, Debug)]
pub struct Sprite {
    // Relative cells to the anchor along with the character and its foreground color.
    pub cells: Vec<(ConsoleCell, char, Color)>,
    // The anchor cell; must be one of the entries in `cells`.
    pub anchor: ConsoleCell,
}

#[derive(Copy, Clone, Debug)]
pub enum Boost {
    Brake,
    Coordinate(Coordinate),
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub uid: ParticleId,
    pub position: Coordinate,   // In subpixel coordinates
    pub velocity: Coordinate,   // In subpixel coordinates per frame
    pub acceleration: Coordinate,
    pub color: Color,
    pub kind: ParticleType,
    pub fuel: u16,               // Remaining fuel units (0..=255)
    pub velocity_cap: Coordinate,
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

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Particle {
    pub fn new(
        position: Option<Coordinate>,
        velocity: Option<Coordinate>,
        acceleration: Option<Coordinate>,
        kind: ParticleType,
        velocity_cap: Coordinate,
    ) -> Self {
        Self {
            uid: ParticleId(0), // will be set when added to the scene
            position: position.unwrap_or_default(),
            velocity: velocity.unwrap_or_default(),
            acceleration: acceleration.unwrap_or_default(),
            color: Color::White,
            kind,
            fuel: 510,
            velocity_cap,
        }
    }

    pub fn get_position(&self) -> Coordinate {
        self.position
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn get_colors(&self) -> ParticleColors {
        match self.kind {
            ParticleType::FuelCell => ParticleColors {
                foreground: Color::DarkBlue,
                background: Color::Yellow,
            },
            ParticleType::Rocket => ParticleColors {
                foreground: self.color,
                background: Color::Black,
            },
        }
    }

    pub fn update(&mut self, bounds: (i32, i32, u16, u16), boost: Option<Boost>) {
        let (console_width, console_height, cell_width, cell_height) = bounds;

        // 1) Handle boost: apply as first step, or reset to 0,0 if none.
        //    If out of fuel, ignore any boost (treated as None).
        if self.fuel == 0 {
            self.acceleration = Coordinate::new(0, 0);
        } else {
            match boost {
                Some(Boost::Brake) => {
                    // Apply braking acceleration based on current velocity
                    self.acceleration = self.braking_acceleration_from_velocity();
                }
                Some(Boost::Coordinate(delta)) => {
                    // Apply provided acceleration vector for this frame
                    self.acceleration += Coordinate::new(delta.x, delta.y);
                }
                None => {
                    // No boost provided: reset acceleration
                    self.acceleration = Coordinate::new(0, 0);
                }
            }
        }

        // 2) Consume fuel (once per frame when accelerating) and apply acceleration to velocity
        if self.acceleration.x != 0 || self.acceleration.y != 0 {
            self.fuel = self.fuel.saturating_sub(1);
        }
        self.velocity += self.acceleration;

        // 3) Clamp velocity to the cap per axis
        self.velocity.x = self.velocity.x.clamp(-self.velocity_cap.x, self.velocity_cap.x);
        self.velocity.y = self.velocity.y.clamp(-self.velocity_cap.y, self.velocity_cap.y);

        // 4) Integrate position with half-velocity for smoother motion
        //    Fuel cells move slower by integrating with a larger divisor.
        let divisor = match self.kind {
            ParticleType::FuelCell => 4,
            _ => 2,
        };
        self.position
            .add(&Coordinate::new(self.velocity.x / divisor, self.velocity.y / divisor));

        // 5) Bounce off borders
        let as_cell = self.position.to_cell();
        if as_cell.y >= cell_height - 1 {
            self.position.y = console_height - (self.position.y - console_height).abs();
            self.velocity.y = self.velocity.y * -1;
        }
        if self.position.y <= 1 {
            self.position.y = self.position.y.abs();
            self.velocity.y = self.velocity.y * -1;
        }
        if as_cell.x >= (cell_width - 1) {
            self.position.x = console_width - (self.position.x - console_width).abs();
            self.velocity.x = self.velocity.x * -1;
        }
        if as_cell.x <= 1 {
            self.position.x = self.position.x.abs();
            self.velocity.x = self.velocity.x * -1;
        }
    }

    pub fn get_particle_char(&self) -> Sprite {
        let ch = match self.kind {
            ParticleType::Rocket => self.get_rocket_char(),
            ParticleType::FuelCell => 'F',
        };
        let anchor = ConsoleCell::new(0, 0);
        Sprite {
            anchor,
            cells: vec![(anchor, ch, self.get_colors().foreground)],
        }
    }

    fn get_rocket_char(&self) -> char {
        let vx = self.velocity.x as f32;
        let vy = self.velocity.y as f32;

        if vx == 0.0 && vy == 0.0 {
            return '•';
        }

        // Angle from - PI..PI, convert to 0..2PI and quantize to 8 sectors
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

impl Particle {
    // Compute braking acceleration from the current velocity (private helper).
    // Rules:
    // - Scale each velocity component's magnitude by 1/10 and floor the result;
    //   acceleration opposes the velocity direction.
    // - If both |vx| and |vy| are <= 10, apply unit acceleration (1) opposite to the
    //   component with the greater magnitude; if equal and non-zero, apply to both.
    // - Never overshoot: cap each axis so acceleration never exceeds -velocity on that axis.
    fn braking_acceleration_from_velocity(&self) -> Coordinate {
        let vx = self.velocity.x;
        let vy = self.velocity.y;

        if vx == 0 && vy == 0 {
            return Coordinate::new(0, 0);
        }

        let absx = vx.abs();
        let absy = vy.abs();

        let mut ax: i32;
        let mut ay: i32;

        if absx <= 10 && absy <= 10 {
            if absx > absy {
                ax = -vx.signum();
                ay = 0;
            } else if absy > absx {
                ax = 0;
                ay = -vy.signum();
            } else {
                ax = -vx.signum();
                ay = -vy.signum();
            }
        } else {
            ax = max(-(absx / 20) * vx.signum(), 1);
            ay = max(-(absy / 20) * vy.signum(), 1);
        }

        ax = match vx.cmp(&0) {
            std::cmp::Ordering::Greater => ax.clamp(-vx, 0),
            std::cmp::Ordering::Less => ax.clamp(0, -vx),
            std::cmp::Ordering::Equal => 0,
        };
        ay = match vy.cmp(&0) {
            std::cmp::Ordering::Greater => ay.clamp(-vy, 0),
            std::cmp::Ordering::Less => ay.clamp(0, -vy),
            std::cmp::Ordering::Equal => 0,
        };

        Coordinate::new(ax, ay)
    }
}
