use crossterm::QueueableCommand;
use crossterm::cursor::{Hide, MoveTo, MoveToColumn, MoveToRow};
use crossterm::terminal::{Clear, ClearType};
use std::io::{Write, stdout};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::style::{Colors, SetColors};

use crate::border::BorderChars;
use crate::particle::{Particle, Boost, ParticleType, ParticleId};
use crate::scene::Scene;
use crate::spatial::SUBPIXEL_SCALE;
use crate::collision::Collision;
use crate::game_events::GameEvent; // add: returning events

pub const DEFAULT_FOREGROUND_COLOR: Color = Color::White;
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::Black;

pub struct Console {
    pub(crate) cell_width: u16,
    pub(crate) cell_height: u16,
    pub(crate) height: i32,
    pub(crate) width: i32,
    scene: Scene,
}

impl Console {
    pub fn new(cell_width: u16, cell_height: u16) -> Self {
        Self {
            cell_width,
            cell_height,
            width: cell_width as i32 * SUBPIXEL_SCALE,
            height: cell_height as i32 * SUBPIXEL_SCALE,
            scene: Scene::new(vec![]),
        }
    }

    // Find the index of a particle by its stable UID.
    pub fn find_particle_index_by_id(&self, id: ParticleId) -> Option<usize> {
        self.scene
            .particles
            .iter()
            .position(|q| q.uid == id)
    }

    fn get_border_char(row: u16, col: u16, height: u16, width: u16) -> Option<BorderChars> {
        let is_top_row = row == 0;
        let is_bottom_row = row == height - 1;
        let is_left_column = col == 0;
        let is_right_column = col == width - 1;

        match (is_top_row, is_bottom_row, is_left_column, is_right_column) {
            (true, false, true, false) => Some(BorderChars::TopLeft),
            (true, false, false, true) => Some(BorderChars::TopRight),
            (true, false, false, false) => Some(BorderChars::Horizontal),
            (false, true, true, false) => Some(BorderChars::BottomLeft),
            (false, true, false, true) => Some(BorderChars::BottomRight),
            (false, true, false, false) => Some(BorderChars::Horizontal),
            (false, false, true, false) | (false, false, false, true) => {
                Some(BorderChars::Vertical)
            }
            _ => None,
        }
    }

    pub fn draw_borders(&self) -> &Self {
        let mut stdout = stdout();
        stdout.queue(Clear(ClearType::All)).unwrap();

        for console_j in 0..self.cell_height {
            stdout.queue(MoveToRow(console_j)).unwrap();

            for console_i in 0..self.cell_width {
                stdout.queue(MoveToColumn(console_i)).unwrap();

                if let Some(border_char) =
                    Self::get_border_char(console_j, console_i, self.cell_height, self.cell_width)
                {
                    stdout.write(border_char.to_string().as_bytes()).unwrap();
                }
            }
        }
        stdout.flush().unwrap();
        self
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.scene.add_particle(particle);
    }

    pub fn get_particle(&self, index: usize) -> Option<&Particle> {
        self.scene.particles.get(index)
    }

    // Runs a simulation tick:
    // 1) capture current renderables and erase them;
    // 2) update existing particles in the scene (in-place) with provided boosts;
    // 3) draw the new frame.
    // The boosts vector is applied in scene order; missing entries default to None.
    pub fn tick(&mut self, boosts: Vec<Option<Boost>>) -> Vec<GameEvent> {
        let mut stdout = stdout();
        stdout.queue(Hide).unwrap();

        // 1) Erase previously drawn cells by overwriting them with spaces
        let prev_particles = self.scene.get_renderable(self.cell_width, self.cell_height).cells;
        for (cell, _ch, _color) in prev_particles {
            if cell.x >= 1
                && cell.x < self.cell_width - 1
                && cell.y >= 1
                && cell.y < self.cell_height - 1
            {
                stdout.queue(MoveTo(cell.x, cell.y)).unwrap();
                stdout.queue(SetColors(Colors::new(
                    DEFAULT_FOREGROUND_COLOR,
                    DEFAULT_BACKGROUND_COLOR,
                ))).unwrap();
                stdout.write(" ".as_bytes()).unwrap();
            }
        }

        // 2) Update particles in-place
        let count = self.scene.particles.len();
        let bounds = (self.width, self.height, self.cell_width, self.cell_height);
        for i in 0..count {
            let b = boosts.get(i).cloned().unwrap_or(None);
            self.scene.particles[i].update(bounds, b);
        }

        // 3) Build renderable for the new frame (cells + collisions)
        let renderable_now = self.scene.get_renderable(self.cell_width, self.cell_height);

        // 3a) Draw current scene cells
        for (cell, ch, color) in renderable_now.cells {
            if cell.x >= 1
                && cell.x < self.cell_width - 1
                && cell.y >= 1
                && cell.y < self.cell_height - 1
            {
                stdout.queue(MoveTo(cell.x, cell.y)).unwrap();
                stdout.queue(SetForegroundColor(color.foreground)).unwrap();
                stdout.queue(SetBackgroundColor(color.background)).unwrap();
                let s = ch.to_string();
                stdout.write(s.as_bytes()).unwrap();
                stdout.queue(SetForegroundColor(DEFAULT_FOREGROUND_COLOR)).unwrap();
                stdout.queue(SetBackgroundColor(DEFAULT_BACKGROUND_COLOR)).unwrap();
            }
        }
        stdout.flush().unwrap();

        // 4) Produce events from collisions (per-collision refuel event)
        let mut events: Vec<GameEvent> = Vec::new();
        for coll in renderable_now.collisions {
            match coll {
                Collision::Refuel { participants, .. } => {
                    // pick first rocket and first fuel cell in the group
                    let mut rocket_idx: Option<usize> = None;
                    let mut fuel_idx: Option<usize> = None;

                    for pid in participants {
                        // Resolve stable ParticleId to current scene index
                        if let Some(idx) = self.find_particle_index_by_id(pid) {
                            match self.scene.particles[idx].kind {
                                ParticleType::Rocket if rocket_idx.is_none() => {
                                    rocket_idx = Some(idx)
                                }
                                ParticleType::FuelCell if fuel_idx.is_none() => {
                                    fuel_idx = Some(idx)
                                }
                                _ => {}
                            }
                            if rocket_idx.is_some() && fuel_idx.is_some() {
                                break;
                            }
                        }
                    }

                    if let (Some(ri), Some(fi)) = (rocket_idx, fuel_idx) {
                        events.push(GameEvent::Refuel {
                            rocket_idx: ri,
                            fuel_cell_idx: fi,
                        });
                    }
                }
            }
        }

        // Return generated events
        events
    }

    pub fn display_info(&self, particle: &Particle, pressed_button_str: &str) {
        let mut stdout = stdout();
        stdout.queue(Hide).unwrap();
            // Header
            stdout.queue(MoveTo(self.cell_width + 1, 0)).unwrap();
            stdout.write("Information.".as_bytes()).unwrap();

            // Fuel bar: occupies exactly 32 characters in the margin
            let margin_width: usize = 32;
            let filled = (particle.fuel as usize * margin_width) / 510;
            let bar = format!("{}{}", "#".repeat(filled), " ".repeat(margin_width - filled));
            stdout.queue(MoveTo(self.cell_width + 1, 1)).unwrap();
            stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
            stdout.write(bar.as_bytes()).unwrap();

            // Pressed keys
            stdout.queue(MoveTo(self.cell_width + 1, 2)).unwrap();
            stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
            stdout.write(pressed_button_str.as_bytes()).unwrap();

            // Position / Velocity / Acceleration readouts
            stdout.queue(MoveTo(self.cell_width + 1, 4)).unwrap();
            stdout
                .write(
                    format!(
                        "P: {:04}i, {:04}j",
                        particle.position.y, particle.position.x,
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.queue(MoveTo(self.cell_width + 1, 5)).unwrap();
            stdout
                .write(
                    format!(
                        "V: {:04}i, {:04}j",
                        particle.velocity.y, particle.velocity.x,
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.queue(MoveTo(self.cell_width + 1, 6)).unwrap();
            stdout
                .write(
                    format!(
                        "A: {:04}i, {:04}j",
                        particle.acceleration.y, particle.acceleration.x,
                    )
                    .as_bytes(),
                )
                .unwrap();

        stdout.queue(MoveTo(self.cell_width + 1, 7)).unwrap();
        stdout
            .write(
                format!(
                    "F: {:03}",
                    particle.fuel,
                )
                .as_bytes(),
            )
            .unwrap();
        stdout.flush().unwrap();
    }

    // Set a particle's fuel to the provided amount (no-op if out of bounds).
    pub fn set_particle_fuel(&mut self, idx: usize, amount: u16) {
        if let Some(part) = self.scene.particles.get_mut(idx) {
            part.fuel = amount;
        }
    }

    // Remove a particle at the given index (no-op if out of bounds).
    pub fn remove_particle(&mut self, idx: usize) {
        if idx < self.scene.particles.len() {
            self.scene.particles.remove(idx);
        }
    }
}
