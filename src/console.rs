use crossterm::cursor::{Hide, MoveTo, MoveToColumn, MoveToRow};
use crossterm::style::Print;
use crossterm::{QueueableCommand};
use std::io::{stdout, Write};
use crossterm::terminal::{Clear, ClearType};
use crate::border::BorderChars;
use crate::particle::Particle;
use crate::spatial::{SUBPIXEL_SCALE};

pub struct Console {
    pub(crate) cell_width: u16,
    pub(crate) cell_height: u16,
    pub(crate) height: i32,
    pub(crate) width: i32,
}

impl Console {
    pub fn new(cell_width: u16, cell_height: u16) -> Self {
        Self {
            cell_width,
            cell_height,
            width: cell_width as i32 * SUBPIXEL_SCALE,
            height: cell_height as i32 * SUBPIXEL_SCALE,
        }
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
            (false, false, true, false) | (false, false, false, true) => Some(BorderChars::Vertical),
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

                if let Some(border_char) = Self::get_border_char(console_j, console_i, self.cell_height, self.cell_width) {
                    stdout.write(border_char.to_string().as_bytes()).unwrap();
                }
            }
        }
        stdout.flush().unwrap();
        self
    }

    pub fn draw_particle(&self, particle: &Particle) {
        let mut stdout = stdout();

        stdout.queue(Hide).unwrap();

        let particle_coordinate = particle.get_position().to_cell();

        for console_j in 1..self.cell_height - 1 {
            stdout.queue(MoveToRow(console_j)).unwrap();
            for console_i in 1..self.cell_width - 1 {
                stdout.queue(MoveToColumn(console_i)).unwrap();
                let is_particle_here = particle_coordinate.x == console_i && particle_coordinate.y == console_j;
                if is_particle_here {
                    stdout.write("•".as_bytes()).unwrap();
                } else {
                    stdout.write(" ".as_bytes()).unwrap();
                }
                stdout.flush().unwrap();
            }
        }

        // Display velocity and acceleration information underneath the box
        stdout.flush().unwrap();
    }
    
    pub fn display_info(
        &self,
        particle: &Particle,
        pressed_button_str: &str,
    ) {
        let mut stdout = stdout();
        stdout.queue(Hide).unwrap();
        stdout.queue(MoveTo(self.cell_width + 1, 0)).unwrap();
        stdout.write("Information.".as_bytes()).unwrap();
        stdout.queue(MoveTo(self.cell_width + 1, 2)).unwrap();
        stdout.write(pressed_button_str.as_bytes()).unwrap();
        stdout.queue(MoveTo(self.cell_width + 1, 4)).unwrap();
        stdout.write(format!(
            "P: {:04}i, {:04}j",
            particle.position.y,
            particle.position.x,
        ).as_bytes()).unwrap();
        stdout.queue(MoveTo(self.cell_width + 1, 5)).unwrap();
        stdout.write(format!(
            "V: {:04}i, {:04}j",
            particle.velocity.y,
            particle.velocity.x,
        ).as_bytes()).unwrap();
        stdout.queue(MoveTo(self.cell_width + 1, 6)).unwrap();
        stdout.write(format!(
            "A: {:04}i, {:04}j",
            particle.acceleration.y,
            particle.acceleration.x,
        ).as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}
