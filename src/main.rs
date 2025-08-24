mod border;
mod console;
mod particle;
mod spatial;

use crate::console::Console;
use crate::particle::Particle;
use crate::spatial::{Coordinate, SUBPIXEL_SCALE};
use crossterm::cursor::{MoveTo};
use crossterm::event::{
    Event, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags, poll, read,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{QueueableCommand, execute};
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().expect("Failed to enable raw mode");
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )
    .expect("Failed to set keyboard enhancement flags");
    // detect the length of terminal
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
        None,
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
                            if event.kind == KeyEventKind::Press {
                                up_held = true;
                            } else if event.kind == KeyEventKind::Release {
                                up_held = false;
                            }
                        },
                        KeyCode::Down => {
                            if event.kind == KeyEventKind::Press {
                                down_held = true;
                            } else if event.kind == KeyEventKind::Release {
                                down_held = false;
                            }
                        },
                        KeyCode::Left => {
                            if event.kind == KeyEventKind::Press {
                                left_held = true;
                            } else if event.kind == KeyEventKind::Release {
                                left_held = false;
                            }
                        },
                        KeyCode::Right => {
                            if event.kind == KeyEventKind::Press {
                                right_held = true;
                            } else if event.kind == KeyEventKind::Release{
                                right_held = false;
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        } else {
            let mut pressed_str = String::from("____");
            let mut d_a_y = 0;
            if up_held | down_held {
                if up_held {
                pressed_str.replace_range(0..1,  "↑");
                    d_a_y += 1;
                }
                if down_held {
                pressed_str.replace_range(1..2,  "↓");
                    d_a_y -= 1;
                }
                acc_y += d_a_y;
            } else {
                acc_y = 0
            }
            let mut d_a_x = 0;
            if left_held | right_held {
                if left_held {
                pressed_str.replace_range(2..3,  "←");
                    d_a_x -= 1;
                }
                if right_held {
                pressed_str.replace_range(3..4,  "→");
                    d_a_x += 1;
                }
                acc_x += d_a_x;
            }
            particles.iter_mut().for_each(|particle| {
                particle
                    .update(&console)
                    .set_acceleration(Coordinate::new(acc_x, acc_y))
            });
            console.draw(&particles);
            stdout.queue(MoveTo(0, term_h + 5)).unwrap();
            stdout.write(pressed_str.as_bytes()).unwrap();
            stdout.flush().unwrap();
            sleep(Duration::from_millis(100));
        }
    }
    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(stdout, PopKeyboardEnhancementFlags)
        .expect("Failed to unset set keyboard enhancement flags");
}
