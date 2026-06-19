use germterm::{
    color::Color, crossterm::event::{Event, KeyCode, KeyEvent}, draw::{self, draw_fps_counter, draw_octad, draw_rect, draw_text, draw_twoxel}, engine::{Engine, end_frame, exit_cleanup, init, start_frame}, input::poll_input, layer::create_layer
};
use std::io;
use rand::prelude::*;
struct Snake{
    x:f32,
    y:f32
}

enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() -> io::Result<()> {
    let mut snake=Snake{
        x:3.0,
        y:4.0
    };
    let TERM_COLS:u16=100;
    let TERM_ROWS:u16=40;

    let mut engine = Engine::new(TERM_COLS, TERM_ROWS)
        .limit_fps(30);
    let layer = create_layer(&mut engine, 0);

    // Initialize engine and layers
    init(&mut engine)?;
    let mut direction=Direction::RIGHT;
    'update_loop: loop {
        
        // Start the frame
        start_frame(&mut engine);

        for event in poll_input() {
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char('q') => break 'update_loop,

                    KeyCode::Char('w') => direction = Direction::UP,
                    KeyCode::Char('s') => direction = Direction::DOWN,
                    KeyCode::Char('a') => direction = Direction::LEFT,
                    KeyCode::Char('d') => direction = Direction::RIGHT,

                    _ => {}
                }
            }
        }
        match direction{
            Direction::UP => snake.y -= 0.5,
            Direction::DOWN => snake.y += 0.5,
            Direction::LEFT => snake.x -= 0.5,
            Direction::RIGHT => snake.x += 0.5,
        }
        draw_twoxel(&mut engine, layer, snake.x, snake.y, Color::RED);
        draw_fps_counter(&mut engine, layer, 0, 0);
        let mut rng = rand::rng();
        for i in 1..5{
            let x_range:f32=rng.random_range(0..TERM_COLS).into();
            let y_range:f32=rng.random_range(0..TERM_ROWS).into();
            draw_twoxel(&mut engine, layer, x_range,y_range, Color::BLUE);
        }

        // End the frame
        end_frame(&mut engine)?;
    }

    // Restore terminal before exiting
    exit_cleanup(&mut engine)?;
    Ok(())
}