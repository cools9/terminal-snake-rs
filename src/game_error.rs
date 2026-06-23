pub enum GameError {
    FrameError(String),
    TerminalSizeError(String),
    /// Is it won when games finished
    GameOver(bool),
}

impl From<std::io::Error> for GameError {
    fn from(error: std::io::Error) -> Self {
        GameError::FrameError(error.to_string())
    }
}