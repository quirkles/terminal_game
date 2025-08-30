use crossterm::style::Color;
use crate::particle::{Particle};
use crate::spatial::ConsoleCell;

#[derive(Clone)]
pub struct Scene {
    pub particles: Vec<Particle>,
}

impl Scene {
    pub fn new(particles: Vec<Particle>) -> Self {
        Self { particles }
    }


    // Returns the cells, characters, and colors that should be drawn for this scene.
    // For now, each particle is represented by a directional arrow based on its velocity.
    pub fn get_renderable(&self) -> Vec<(ConsoleCell, char, Color)> {
        self.particles
            .iter()
            .map(|p| (p.get_position().to_cell(), p.get_particle_char(), p.color))
            .collect()
    }
}
