use crate::border::BorderChars;
use crate::particle::Particle;
use crate::scene::Scene;
use crate::spatial::SUBPIXEL_SCALE;
use crossterm::QueueableCommand;
use crossterm::cursor::{Hide, MoveTo, MoveToColumn, MoveToRow};
use crossterm::terminal::{Clear, ClearType};
use std::io::{Write, stdout};
use crossterm::style::{Color, SetForegroundColor};

pub const DEFAULT_FOREGROUND_COLOR: Color = Color::White;
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::Black;

pub struct Console {
    pub(crate) cell_width: u16,
    pub(crate) cell_height: u16,
    pub(crate) height: i32,
    pub(crate) width: i32,
    previous_scene: Option<Scene>,
}

impl Console {
    pub fn new(cell_width: u16, cell_height: u16) -> Self {
        Self {
            cell_width,
            cell_height,
            width: cell_width as i32 * SUBPIXEL_SCALE,
            height: cell_height as i32 * SUBPIXEL_SCALE,
            previous_scene: None,
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

    pub fn draw_scene(&mut self, scene: Scene) {
        let mut stdout = stdout();
        stdout.queue(Hide).unwrap();

        // 1) Erase previously drawn cells by overwriting them with spaces
        if let Some(prev) = &self.previous_scene {
            for (cell, _ch, _color) in prev.get_renderable() {
                if cell.x >= 1
                    && cell.x < self.cell_width - 1
                    && cell.y >= 1
                    && cell.y < self.cell_height - 1
                {
                    stdout.queue(MoveTo(cell.x, cell.y)).unwrap();
                    stdout.write(" ".as_bytes()).unwrap();
                }
            }
        }

        // 2) Draw current scene cells
        for (cell, ch, color) in scene.get_renderable() {
            if cell.x >= 1
                && cell.x < self.cell_width - 1
                && cell.y >= 1
                && cell.y < self.cell_height - 1
            {
                stdout.queue(MoveTo(cell.x, cell.y)).unwrap();
                stdout.queue(SetForegroundColor(color)).unwrap();
                let s = ch.to_string();
                stdout.write(s.as_bytes()).unwrap();
                stdout.queue(SetForegroundColor(DEFAULT_FOREGROUND_COLOR)).unwrap();
            }
        }

        stdout.flush().unwrap();

        // 3) Store the current scene as previous for the next frame
        self.previous_scene = Some(scene);
    }

    pub fn display_info(&self, particle: &Particle, pressed_button_str: &str) {
        let mut stdout = stdout();
        stdout.queue(Hide).unwrap();
        stdout.queue(MoveTo(self.cell_width + 1, 0)).unwrap();
        stdout.write("Information.".as_bytes()).unwrap();
        stdout.queue(MoveTo(self.cell_width + 1, 2)).unwrap();
        stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
        stdout.write(pressed_button_str.as_bytes()).unwrap();
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
        stdout.flush().unwrap();
    }
}
