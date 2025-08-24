// Spatial utilities: coordinates and console cells
// Subpixel scaling factor - positions and velocities are 16x more precise than terminal cells
pub const SUBPIXEL_SCALE: i32 = 16;

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
