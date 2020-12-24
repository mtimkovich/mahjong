use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Suit {
    Pin,
    So,
    Man,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Dragon {
    Green,
    Red,
    White,
}

#[derive(Debug)]
pub struct Tile {
    value: Option<u32>,
    suit: Option<Suit>,
    wind: Option<Wind>,
    dragon: Option<Dragon>,
}

impl Tile {
    pub fn suit(suit: Suit, value: u32) -> Tile {
        Tile {
            value: Some(value),
            suit: Some(suit),
            wind: None,
            dragon: None,
        }
    }

    pub fn wind(wind: Wind) -> Tile {
        Tile {
            value: None,
            suit: None,
            wind: Some(wind),
            dragon: None,
        }
    }

    pub fn dragon(dragon: Dragon) -> Tile {
        Tile {
            value: None,
            suit: None,
            wind: None,
            dragon: Some(dragon),
        }
    }
}
