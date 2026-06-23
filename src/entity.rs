use crossterm::event::KeyCode;

pub struct Snake {
    pub x: i32,
    pub y: i32,
}

pub struct Apple {
    pub x: i32,
    pub y: i32,
    pub has_been_eaten: bool,
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    pub fn from(self, value: KeyCode) -> Self {
        match value {
            KeyCode::Left | KeyCode::Char('a') => Direction::LEFT,
            KeyCode::Right | KeyCode::Char('d') => Direction::RIGHT,
            KeyCode::Up | KeyCode::Char('w') => Direction::UP,
            KeyCode::Down | KeyCode::Char('s')=> Direction::DOWN,

            _ => self
        }
    }
}
