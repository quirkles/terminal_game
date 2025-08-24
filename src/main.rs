mod spatial;
mod border;
mod particle;
mod console;

use crossterm::event::{poll, read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::Duration;

use crate::console::Console;
use crate::particle::Particle;
use crate::spatial::{Coordinate, SUBPIXEL_SCALE};

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");
    // detect length of terminal
    let (term_w, term_h) = crossterm::terminal::size().unwrap();
    let console = Console::new(term_w, term_h);

    // Init the map
    console.draw_borders();

    // Init a vector of mutable particles
    let mut particles: Vec<Particle> = Vec::new();
    let particle = Particle::new(
        Some(Coordinate::new(
            ((rand::random::<u16>() % (term_w - 2)) + 1) as i32 * SUBPIXEL_SCALE,
            ((rand::random::<u16>() % (term_h - 2)) + 1) as i32 * SUBPIXEL_SCALE,
        )),
        Some(Coordinate::new(
            (rand::random::<i32>() % 32) + 16,
            (rand::random::<i32>() % 32) + 16,
        )),
    );
    particles.push(particle);

    let mut interrupt_flag = false;
    // Listener for keydown on escape and exit

    while !interrupt_flag {
        if poll(Duration::from_millis(50)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    if event.code == crossterm::event::KeyCode::Char('q') {
                        interrupt_flag = true;
                    }
                }
                _ => (),
            }
        } else {
            particles
                .iter_mut()
                .for_each(|particle| particle.update(&console));
            console.draw(&particles);
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");
}
