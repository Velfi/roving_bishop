use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Move::*;
        write!(
            f,
            "{}",
            match self {
                SouthEast => "↘",
                NorthWest => "↖",
                NorthEast => "↗",
                SouthWest => "↙",
            }
        )
    }
}
