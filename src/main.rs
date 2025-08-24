mod spatial;
mod border;
mod particle;
mod console;

use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::Duration;

use crate::console::Console;
use crate::particle::Particle;
use crate::spatial::{Coordinate, SUBPIXEL_SCALE};

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");
    // detect length of terminal
    // let (term_w, term_h) = crossterm::terminal::size().unwrap();
    let (term_w, term_h) = (45, 15);
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
        None
    );
    particles.push(particle);

    let mut interrupt_flag = false;
    // Listener for keydown on escape and exit

    let mut acc_x: i32 = 0;
    let mut acc_y: i32 = 0;

    let mut up_held = false;
    let mut down_held = false;
    let mut left_held = false;
    let mut right_held = false;

    while !interrupt_flag {
        if poll(Duration::from_millis(50)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q') => {
                            interrupt_flag = true;
                        }
                        KeyCode::Up => {
                            up_held = event.is_press()
                        }
                        KeyCode::Down => {
                            down_held = event.is_press()
                        }
                        KeyCode::Left => {
                            left_held = event.is_press()
                        }
                        KeyCode::Right => {
                            right_held = event.is_press()
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        } else {
            if up_held {
                acc_y -= 1;
            }
            if down_held {
                acc_y += 1;
            }
            if left_held {
                acc_x -= 1;
            }
            if right_held {
                acc_x += 1;
            }
            particles
                .iter_mut()
                .for_each(|particle| particle.update(&console).set_acceleration(Coordinate::new(acc_x, acc_y)));
            console.draw(&particles);
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");
}
