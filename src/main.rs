pub mod game_error;
pub mod entity;

use crossterm::event::KeyCode;
use germterm::{
    color::{Color, ColorGradient, GradientStop},
    crossterm::event::{Event, KeyEvent},
    draw::{draw_fps_counter, draw_rect, draw_text, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::{LayerIndex, create_layer},
    particle::{ParticleEmitter, ParticleSpec, spawn_particles},
    rich_text::{Attributes, RichText},
};
use rand::prelude::*;
use crossterm::terminal;
use crate::entity::{Apple, Direction, Snake};
use crate::game_error::GameError;
use crate::game_error::GameError::TerminalSizeError;

const TERM_COLS: u16 = 100;
const TERM_ROWS: u16 = 40;

fn start_game(engine: &mut Engine) -> Result<(), GameError> {
    let mut snake = Snake { x: 3, y: 4 };

    let mut apples: Vec<Apple> = Vec::new();
    let mut rng = rand::rng();
    for i in 0..5 {
        let random_x: i32 = rng.random_range(2..30);
        let random_y: i32 = rng.random_range(2..20);
        apples.push(Apple {
            x: i + 1 + random_x,
            y: i + 1 + random_y,
            has_been_eaten: false,
        });
    }
    let layer = create_layer(engine, 0);
    let apple_layer: LayerIndex = create_layer(engine, 1);
    let mut won = false;

    let (col, row) = terminal::size()?;
    if !(col >= TERM_COLS && row >= TERM_ROWS) {
        return Err(TerminalSizeError("Terminal size too small expand it and rerun this please😭".into()))
    }

    // Initialize engine and layers
    init(engine)?;
    let mut direction = Direction::RIGHT;
    loop {
        let events = poll_input();
        // Start the frame
        start_frame(engine);

        if won {
            draw_text(engine, layer, 10, 10,
                      RichText::new("YOU WON")
                          .with_fg(Color::RED)
                          .with_attributes(Attributes::BOLD),
            );

            // waiting player's reply
            for e in events {
                if e.is_key_press() {
                    return Ok(())
                }
            }
        } else {
            // game is still not end
            for event in events {
                match event {
                    Event::Key(KeyEvent { code, .. })  => {
                        if code == KeyCode::Char('q') || code == KeyCode::Esc {
                            return Err(GameError::GameOver(apples.is_empty()))
                        }
                        direction = direction.from(code)
                    },
                    _ => {},
                }
            }
            match direction {
                Direction::UP => snake.y -= 1,
                Direction::DOWN => snake.y += 1,
                Direction::LEFT => snake.x -= 1,
                Direction::RIGHT => snake.x += 1,
            }

            if snake.x <= 0
                || snake.x >= TERM_COLS as i32 - 1
                || snake.y <= 0
                || snake.y >= TERM_ROWS as i32 - 1
            {
                snake.x=3;
                snake.y=4;
            }
            draw_twoxel(
                engine,
                layer,
                snake.x as f32,
                snake.y as f32,
                Color::RED,
            );
            draw_fps_counter(engine, layer, 0, 0);

            for apple in &apples {
                draw_twoxel(
                    engine,
                    apple_layer,
                    apple.x as f32,
                    apple.y as f32,
                    Color::BLUE,
                );
            }
            let mut explosion_pos = None;

            apples.retain(|apple| {
                let eaten = apple.x == snake.x && apple.y == snake.y;
                if eaten {
                    explosion_pos = Some((apple.x, apple.y));
                }

                !eaten
            });

            if let Some((x, y)) = explosion_pos {
                spawn_explosion(engine, apple_layer, x as f32, y as f32);
            }

            won = apples.is_empty();
            draw_border(engine, layer, TERM_COLS, TERM_ROWS);
        }
        end_frame(engine)?;
    }

}

fn main() {
    let mut engine = Engine::new(TERM_COLS, TERM_ROWS).limit_fps(10);

    match start_game(&mut engine) {
        Ok(()) => {},
        Err(GameError::GameOver(_win)) => {}
        Err(TerminalSizeError(msg)) => println!("{}", msg),
        Err(GameError::FrameError(msg)) => println!("{}", msg),
    }

    // Restore terminal before exiting
    exit_cleanup(&mut engine).unwrap()
}

fn spawn_explosion(engine: &mut Engine, layer: LayerIndex, x: f32, y: f32) {
    spawn_particles(
        engine,
        layer,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.1,
            speed: 20.0..=70.0,
            lifetime_sec: 2.0,
            color: germterm::particle::ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color::WHITE),
                GradientStop::new(0.05, Color::RED),
                GradientStop::new(1.0, Color::VIOLET.with_alpha(0)),
            ])),
        },
        &ParticleEmitter {
            count: 30,
            ..Default::default()
        },
    );
}

fn draw_border(engine: &mut Engine, layer: LayerIndex, size_x: u16, size_y: u16) {
    // Left
    draw_rect(engine, layer, 0, 0, 1, size_y as i16, Color::CYAN);

    // Right
    draw_rect(
        engine,
        layer,
        size_x as i16 - 1,
        0,
        1,
        size_y as i16,
        Color::CYAN,
    );

    // Top
    draw_rect(engine, layer, 0, 0, size_x as i16, 1, Color::CYAN);

    // Bottom
    draw_rect(
        engine,
        layer,
        0,
        size_y as i16 - 1,
        size_x as i16,
        1,
        Color::CYAN,
    );
}
