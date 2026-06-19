use germterm::{
    color::{Color, ColorGradient, GradientStop},
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{self, draw_fps_counter, draw_octad, draw_rect, draw_text, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::{LayerIndex, create_layer},
    particle::{ParticleEmitter, ParticleSpec, spawn_particles},
    rich_text::{Attributes, RichText},
};
use rand::prelude::*;
use std::io;

struct Snake {
    x: i32,
    y: i32,
}

struct Apple {
    x: i32,
    y: i32,
    has_been_eaten: bool,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() -> io::Result<()> {
    let mut snake = Snake { x: 3, y: 4 };
    let TERM_COLS: u16 = 100;
    let TERM_ROWS: u16 = 40;

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
    let mut engine = Engine::new(TERM_COLS, TERM_ROWS).limit_fps(10);
    let layer = create_layer(&mut engine, 0);
    let apple_layer: LayerIndex = create_layer(&mut engine, 1);
    let mut has_not_won: bool = true;
    // Initialize engine and layers
    init(&mut engine)?;
    let mut direction = Direction::RIGHT;
    'update_loop: loop {
        if has_not_won {
            // Start the frame
            start_frame(&mut engine);

            for event in poll_input() {
                if let Event::Key(KeyEvent { code, .. }) = event {
                    match code {
                        KeyCode::Char('q') => {
                            break 'update_loop;
                            has_not_won = false;
                        }
                        KeyCode::Char('w') => direction = Direction::UP,
                        KeyCode::Char('s') => direction = Direction::DOWN,
                        KeyCode::Char('a') => direction = Direction::LEFT,
                        KeyCode::Char('d') => direction = Direction::RIGHT,

                        _ => {}
                    }
                }
            }
            match direction {
                Direction::UP => snake.y -= 1,
                Direction::DOWN => snake.y += 1,
                Direction::LEFT => snake.x -= 1,
                Direction::RIGHT => snake.x += 1,
            }
            draw_twoxel(
                &mut engine,
                layer,
                snake.x as f32,
                snake.y as f32,
                Color::RED,
            );
            draw_fps_counter(&mut engine, layer, 0, 0);

            for apple in &apples {
                draw_twoxel(
                    &mut engine,
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
                spawn_explosion(&mut engine, apple_layer, x as f32, y as f32);
            }

            if apples.is_empty() {
                has_not_won = false;
            }
            border(&mut engine, layer, TERM_COLS, TERM_ROWS);
            // End the frame
            end_frame(&mut engine)?;
        } else {
            start_frame(&mut engine);

            draw_text(
                &mut engine,
                layer,
                10,
                10,
                RichText::new("YOU WON")
                    .with_fg(Color::RED)
                    .with_attributes(Attributes::BOLD),
            );

            end_frame(&mut engine);
        }
    }
    // Restore terminal before exiting
    exit_cleanup(&mut engine)?;
    return Ok(());
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

fn border(engine: &mut Engine, layer: LayerIndex, size_x: u16, size_y: u16) {
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
