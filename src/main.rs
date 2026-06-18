use germterm::{
    color::Color, crossterm::event::{Event, KeyCode, KeyEvent}, draw::{draw_fps_counter, draw_octad, draw_rect, draw_text, draw_twoxel}, engine::{Engine, end_frame, exit_cleanup, init, start_frame}, input::poll_input, layer::create_layer
};
use std::io;

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
    let mut engine = Engine::new(100, 40)
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
            Direction::UP => snake.y += 1.0,
            Direction::DOWN => snake.y -= 1.0,
            Direction::LEFT => snake.x -= 1.0,
            Direction::RIGHT => snake.x += 1.0,
        }
        draw_twoxel(&mut engine, layer, snake.x, snake.y, Color::RED);
        draw_fps_counter(&mut engine, layer, 0, 0);

        // End the frame
        end_frame(&mut engine)?;
    }

    // Restore terminal before exiting
    exit_cleanup(&mut engine)?;
    Ok(())
}