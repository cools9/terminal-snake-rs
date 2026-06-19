use germterm::{
    color::Color, crossterm::event::{Event, KeyCode, KeyEvent}, draw::{self, draw_fps_counter, draw_octad, draw_rect, draw_text, draw_twoxel}, engine::{Engine, end_frame, exit_cleanup, init, start_frame}, input::poll_input, layer::{LayerIndex, create_layer}
};
use std::io;
use rand::prelude::*;


struct Snake{
    x:u32,
    y:u32
}

struct Apple{
    x:u32,
    y:u32,
    has_been_eaten:bool
}

enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() -> io::Result<()> {
    let mut snake=Snake{
        x:3,
        y:4
    };
    let TERM_COLS:u16=100;
    let TERM_ROWS:u16=40;
    let mut apples:Vec<Apple>=Vec::new();
    let mut rng = rand::rng();
    for i in 0..5{
        let random_x:u32=rng.random_range(2..40);
        let random_y:u32=rng.random_range(2..20);
        apples.push(
            Apple{
                x:i+1+random_x,
                y:i+1+random_y,
                has_been_eaten:false
            }
        );
    }
    let mut engine = Engine::new(TERM_COLS, TERM_ROWS)
        .limit_fps(30);
    let layer = create_layer(&mut engine, 0);
    let apple_layer:LayerIndex= create_layer(&mut engine, 1);

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
            Direction::UP => snake.y -= 1,
            Direction::DOWN => snake.y += 1,
            Direction::LEFT => snake.x -= 1,
            Direction::RIGHT => snake.x += 1,
        }
        draw_twoxel(&mut engine, layer, snake.x as f32, snake.y as f32, Color::RED);
        draw_fps_counter(&mut engine, layer, 0, 0);
        
        for apple in &apples {
            draw_twoxel(
                &mut engine,
                apple_layer,
                apple.x as f32,
                apple.y as f32 * 0.5,
                Color::BLUE,
            );
            if apple.x==snake.x && apple.y == snake.y{
                &apples.pop();
            }
        }

        
        // End the frame
        end_frame(&mut engine)?;
    }

    // Restore terminal before exiting
    exit_cleanup(&mut engine)?;
    Ok(())
}