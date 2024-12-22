use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum ESymbol{
    None,
    Cross,
    Circle
}

#[derive(Clone, Copy, PartialEq)]
pub enum EPlayer{
    Cross,
    Circle,
}

pub enum EGameResult{
    None,
    Win,
    Draw,
}

impl fmt::Display for EPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EPlayer::Cross => write!(f, "Cross"),
            EPlayer::Circle => write!(f, "Circle"),
        }
    }
}