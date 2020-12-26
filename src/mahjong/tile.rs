use std::convert::AsRef;
use std::fmt;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr)]
pub enum Suit {
    Pin,
    So,
    Man,
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr)]
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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output;
        if let Some(wind) = self.wind {
            output = format!("{} Wind", wind.as_ref());
        } else if let Some(dragon) = self.dragon {
            output = format!("{} Dragon", dragon.as_ref());
        } else if let (Some(value), Some(suit)) = (self.value, self.suit) {
            output = format!("{} {}", value, suit.as_ref());
        } else {
            output = String::new();
        }

        write!(f, "{}", output)
    }
}
