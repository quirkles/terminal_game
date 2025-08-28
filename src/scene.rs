use crate::particle::Particle;
use crate::spatial::ConsoleCell;

#[derive(Clone)]
pub struct Scene {
    pub particles: Vec<Particle>,
}

impl Scene {
    pub fn new(particles: Vec<Particle>) -> Self {
        Self { particles }
    }

    // Determine the character to draw for a particle based on its velocity direction.
    // Returns one of 8 directional arrows (↑, ↓, ←, →, and diagonals). If stationary, returns '•'.
    pub fn get_particle_char(p: &Particle) -> char {
        let vx = p.velocity.x as f32;
        let vy = p.velocity.y as f32;

        if vx == 0.0 && vy == 0.0 {
            return '•';
        }

        // Angle from -PI..PI, convert to 0..2PI and quantize to 8 sectors
        let mut ang = vy.atan2(vx);
        if ang < 0.0 {
            ang += std::f32::consts::PI * 2.0;
        }

        // Round to nearest sector (PI/4 each)
        let sector = ((ang + std::f32::consts::PI / 8.0) / (std::f32::consts::PI / 4.0)).floor() as i32 % 8;

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

    // Returns the cells and characters that should be drawn for this scene.
    // For now, each particle is represented by a directional arrow based on its velocity.
    pub fn render_cells(&self) -> Vec<(ConsoleCell, char)> {
        self.particles
            .iter()
            .map(|p| (p.get_position().to_cell(), Self::get_particle_char(p)))
            .collect()
    }
}
