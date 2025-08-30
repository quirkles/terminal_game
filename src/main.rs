mod border;
mod console;
mod particle;
mod spatial;
mod scene;

use crate::console::{Console, DEFAULT_BACKGROUND_COLOR, DEFAULT_FOREGROUND_COLOR};
use crate::particle::Particle;
use crate::scene::Scene;
use crate::spatial::{Coordinate, SUBPIXEL_SCALE, braking_acceleration_from_velocity};
use crossterm::event::{
    Event, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags, poll, read,
};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{Write, stdout};
use std::time::Duration;
use crossterm::style::{Color, Colors, SetColors};

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().expect("Failed to enable raw mode");
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES),
        SetColors(Colors::new(
            DEFAULT_FOREGROUND_COLOR,
            DEFAULT_BACKGROUND_COLOR
        ))
    )
    .expect("Failed to set keyboard enhancement flags");
    let margin = 25;
    // detect the length of terminal
    let (term_w, term_h) = crossterm::terminal::size().unwrap();
    let mut console = Console::new(term_w - margin, term_h);

    // Init the map
    console.draw_borders();

    // Init a single particle
    let mut particle = Particle::new(
        Some(Coordinate::new(
            ((rand::random::<u16>() % (term_w - 2)) + 1) as i32 * SUBPIXEL_SCALE,
            ((rand::random::<u16>() % (term_h - 2)) + 1) as i32 * SUBPIXEL_SCALE,
        )),
        None,
        None,
    );

    particle.set_color(Color::Red);

    let velocity_cap = Coordinate::new(200, 200);

    let mut interrupt_flag = false;
    // Listener for keydown on escape and exit

    let mut acc_x: i32 = 0;
    let mut acc_y: i32 = 0;

    let mut up_held = false;
    let mut down_held = false;
    let mut left_held = false;
    let mut right_held = false;
    let mut braking_held = false;

    let mut cycle_phase: u8 = 0; // 0..=15

    while !interrupt_flag {
        if poll(Duration::from_millis(25)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        interrupt_flag = true;
                    }
                    KeyCode::Char('b') => {
                        if event.kind == KeyEventKind::Press {
                            braking_held = true;
                        } else if event.kind == KeyEventKind::Release {
                            braking_held = false;
                        }
                    }
                    KeyCode::Up => {
                        if event.kind == KeyEventKind::Press {
                            up_held = true;
                        } else if event.kind == KeyEventKind::Release {
                            up_held = false;
                        }
                    }
                    KeyCode::Down => {
                        if event.kind == KeyEventKind::Press {
                            down_held = true;
                        } else if event.kind == KeyEventKind::Release {
                            down_held = false;
                        }
                    }
                    KeyCode::Left => {
                        if event.kind == KeyEventKind::Press {
                            left_held = true;
                        } else if event.kind == KeyEventKind::Release {
                            left_held = false;
                        }
                    }
                    KeyCode::Right => {
                        if event.kind == KeyEventKind::Press {
                            right_held = true;
                        } else if event.kind == KeyEventKind::Release {
                            right_held = false;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        } else {
            let mut pressed_str = String::from("");

            // Handle vertical movement and acceleration
            let d_a_y = match (up_held, down_held) {
                (true, false) => {
                    pressed_str.push_str("↑  ");
                    -1
                }
                (false, true) => {
                    pressed_str.push_str("  ↓");
                    1
                }
                (true, true) => {
                    pressed_str.push_str("↑ ↓");
                    0 // Both pressed, cancel out
                }
                (false, false) => {
                    pressed_str.push_str("   ");
                    0
                }
            };

            // Handle horizontal movement and acceleration
            let d_a_x = match (left_held, right_held) {
                (true, false) => {
                    pressed_str.push_str(" ←  ");
                    -1
                }
                (false, true) => {
                    pressed_str.push_str("  → ");
                    1
                }
                (true, true) => {
                    pressed_str.push_str(" ← →");
                    0 // Both pressed, cancel out
                }
                (false, false) => {
                    pressed_str.push_str("   ");
                    0
                }
            };

            cycle_phase = (cycle_phase + 1) % 16;
            if braking_held {
                pressed_str = String::from("Braking");
                acc_x = 0;
                acc_y = 0;
                let braking = braking_acceleration_from_velocity(particle.velocity);
                particle.set_acceleration(braking);
            } else if cycle_phase % 4 == 0 {
                // Update accelerations (throttled to every 8th tick in a 0-15 phase cycle)
                if d_a_y != 0 {
                    acc_y += d_a_y;
                } else {
                    acc_y = 0;
                }

                if d_a_x != 0 {
                    acc_x += d_a_x;
                } else {
                    acc_x = 0;
                }

                particle.set_acceleration(Coordinate::new(acc_x, acc_y));
            }
            particle.update(&console, velocity_cap);

            let scene = Scene::new(vec![particle]);
            console.draw_scene(&scene);
            console.display_info(&particle, &pressed_str);
            stdout.flush().unwrap();
        }
    }
    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(stdout, PopKeyboardEnhancementFlags)
        .expect("Failed to unset set keyboard enhancement flags");
}
