use crate::particle::{Particle, ParticleColors, ParticleId};
use crate::spatial::ConsoleCell;
use crate::collision::Collision;

#[derive(Clone)]
pub struct Scene {
    pub particles: Vec<Particle>,
    pub next_id: u64,
}

pub struct RenderableScene {
    pub cells: Vec<(ConsoleCell, char, ParticleColors)>,
    pub collisions: Vec<Collision>,
}

impl Scene {
    pub fn new(particles: Vec<Particle>) -> Self {
        Self { particles, next_id: 1 }
    }

    pub fn add_particle(&mut self, mut particle: Particle) {
        if particle.uid.0 == 0 {
            particle.uid = ParticleId(self.next_id);
            self.next_id += 1;
        } else if particle.uid.0 >= self.next_id {
            self.next_id = particle.uid.0 + 1;
        }
        self.particles.push(particle);
    }

    // Returns both renderable data and collisions for this scene.
    pub fn get_renderable(&self, cell_width: u16, cell_height: u16) -> RenderableScene {
        // Renderable particles (cell, char, colors)
        let mut cells_to_render: Vec<(ConsoleCell, char, ParticleColors)> = Vec::new();
        for p in self.particles.iter() {
            let base_cell = p.get_position().to_cell();
            let sprite = p.get_particle_char(); // now returns Sprite
            let bg = p.get_colors().background;

            for (rel_cell, ch, fg) in sprite.cells {
                // Place the sprite so that its anchor lands at the particle's base cell.
                // abs = base + (rel - anchor)
                let abs_x_i = base_cell.x as i32 + rel_cell.x as i32 - sprite.anchor.x as i32;
                let abs_y_i = base_cell.y as i32 + rel_cell.y as i32 - sprite.anchor.y as i32;

                // Filter out-of-bounds cells (respect the interior drawable area)
                if abs_x_i >= 1
                    && abs_x_i < (cell_width as i32 - 1)
                    && abs_y_i >= 1
                    && abs_y_i < (cell_height as i32 - 1)
                {
                    let abs_cell = ConsoleCell::new(abs_x_i as u16, abs_y_i as u16);
                    let colors = ParticleColors {
                        foreground: fg,
                        background: bg,
                    };
                    cells_to_render.push((abs_cell, ch, colors));
                }
            }
        }

        // Collisions grouped by console cell
        let mut collisions: Vec<Collision> = Vec::new();
        let mut processed_cells: Vec<ConsoleCell> = Vec::new();

        for i in 0..self.particles.len() {
            let cell_i = self.particles[i].get_position().to_cell();
            if processed_cells.contains(&cell_i) {
                continue;
            }
            // Collect indices of all particles occupying the same cell
            let mut group: Vec<usize> = vec![i];
            for j in (i + 1)..self.particles.len() {
                if self.particles[j].get_position().to_cell() == cell_i {
                    group.push(j);
                }
            }
            if group.len() > 1 {
                // For now, every detected collision is considered a Refuel type.
                let participant_ids = group.iter().map(|&idx| self.particles[idx].uid).collect();
                collisions.push(Collision::Refuel {
                    participants: participant_ids,
                });
            }
            processed_cells.push(cell_i);
        }

        RenderableScene {
            cells: cells_to_render,
            collisions,
        }
    }
}
