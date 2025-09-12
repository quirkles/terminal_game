use crate::particle::{Particle, ParticleColors};
use crate::spatial::ConsoleCell;

pub struct Scene {
    pub particles: Vec<Particle>,
}

pub struct RenderableScene {
    pub particles: Vec<(ConsoleCell, char, ParticleColors)>,
    pub collisions: Vec<(ConsoleCell, Vec<Particle>)>,
}

impl Scene {
    pub fn new(particles: Vec<Particle>) -> Self {
        Self { particles }
    }


    // Returns both renderable data and collisions for this scene.
    pub fn get_renderable(&self) -> RenderableScene {
        // Renderable particles (cell, char, colors)
        let particles_render: Vec<(ConsoleCell, char, ParticleColors)> = self
            .particles
            .iter()
            .map(|p| (p.get_position().to_cell(), p.get_particle_char(), p.get_colors()))
            .collect();

        // Collisions grouped by console cell
        let mut collisions: Vec<(ConsoleCell, Vec<Particle>)> = Vec::new();
        let mut processed_cells: Vec<ConsoleCell> = Vec::new();

        for (i, p_i) in self.particles.iter().enumerate() {
            let cell_i = p_i.get_position().to_cell();
            if processed_cells.contains(&cell_i) {
                continue;
            }
            let mut group: Vec<Particle> = vec![*p_i];
            for p_j in self.particles.iter().skip(i + 1) {
                if p_j.get_position().to_cell() == cell_i {
                    group.push(*p_j);
                }
            }
            if group.len() > 1 {
                collisions.push((cell_i, group));
            }
            processed_cells.push(cell_i);
        }

        RenderableScene {
            particles: particles_render,
            collisions,
        }
    }
}
