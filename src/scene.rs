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

    // Returns the cells and characters that should be drawn for this scene.
    // For now, each particle is represented by a single centered dot.
    pub fn render_cells(&self) -> Vec<(ConsoleCell, char)> {
        self.particles
            .iter()
            .map(|p| (p.get_position().to_cell(), '•'))
            .collect()
    }
}
