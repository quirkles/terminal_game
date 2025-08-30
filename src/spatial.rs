use std::cmp::max;

// Spatial utilities: coordinates and console cells
// Subpixel scaling factor - positions and velocities are 16x more precise than terminal cells
pub const SUBPIXEL_SCALE: i32 = 64;

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct ConsoleCell {
    pub x: u16,
    pub y: u16,
}

impl ConsoleCell {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl PartialEq for ConsoleCell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn add(&mut self, other: &Self) -> &Self {
        self.y += other.y;
        self.x += other.x;
        self
    }

    // Convert from subpixel coordinates to terminal cell coordinates
    pub fn to_cell(&self) -> ConsoleCell {
        ConsoleCell::new(
            ((self.x as f32 / SUBPIXEL_SCALE as f32).round() as i32).max(0) as u16,
            ((self.y as f32 / SUBPIXEL_SCALE as f32).round() as i32).max(0) as u16,
        )
    }
}

// Compute braking acceleration from a given velocity.
// Rules:
// - Scale each velocity component's magnitude by 1/10 and floor the result;
//   acceleration opposes the velocity direction.
// - If both |vx| and |vy| are <= 10, apply unit acceleration (1) opposite to the
//   component with the greater magnitude; if equal and non-zero, apply to both.
// - Never overshoot: cap each axis so acceleration never exceeds -velocity on that axis.
pub fn braking_acceleration_from_velocity(vel: Coordinate) -> Coordinate {
    let vx = vel.x;
    let vy = vel.y;

    // If already stationary, no braking needed.
    if vx == 0 && vy == 0 {
        return Coordinate::new(0, 0);
    }

    let absx = vx.abs();
    let absy = vy.abs();

    let mut ax: i32;
    let mut ay: i32;

    if absx <= 10 && absy <= 10 {
        // Small speeds: unit deceleration on the dominant axis (or both if tied and non-zero)
        if absx > absy {
            ax = -vx.signum();
            ay = 0;
        } else if absy > absx {
            ax = 0;
            ay = -vy.signum();
        } else {
            // Equal and non-zero
            ax = -vx.signum();
            ay = -vy.signum();
        }
    } else {
        // General case: magnitude is floor(|v|/10), direction opposite to velocity
        ax = max(-(absx / 20) * vx.signum(), 1);
        ay = max(-(absy / 20) * vy.signum(), 1);
    }

    // Cap so we never cross zero or reverse direction on any axis.
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
