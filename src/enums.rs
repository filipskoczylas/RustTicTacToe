use std::fmt;

#[derive(Clone, Copy)]
pub enum ESymbol{
    None,
    Cross,
    Circle
}

#[derive(Clone, Copy)]
pub enum EPlayer{
    Cross,
    Circle,
}

impl fmt::Display for EPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EPlayer::Cross => write!(f, "Cross"),
            EPlayer::Circle => write!(f, "Circle"),
        }
    }
}