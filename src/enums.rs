pub enum Direction {
    Right,
    Left,
}
pub enum Kind {
    Statement,
    Expression(String),
    Cmd,
}
pub enum Arrow {
    Up,
    Down,
}
pub enum KeyWords {
    Reset,
    Code,
    Show,
    Add,
}
