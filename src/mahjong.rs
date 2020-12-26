mod tile;
use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use tile::{Dragon, Suit, Tile, Wind};

pub struct Mahjong {
    pub tiles: Vec<Tile>,
    index: usize,
}

impl Mahjong {
    pub fn new() -> Self {
        let mut tiles = Vec::new();

        for _ in 0..4 {
            for (suit, i) in iproduct!(Suit::iter(), 1..10) {
                tiles.push(Tile::suit(suit, i));
            }

            for w in Wind::iter() {
                tiles.push(Tile::wind(w));
            }

            for d in Dragon::iter() {
                tiles.push(Tile::dragon(d));
            }
        }

        tiles.shuffle(&mut thread_rng());

        Mahjong { tiles, index: 0 }
    }

    pub fn draw(&mut self) -> Option<&Tile> {
        let tile = self.tiles.get(self.index);
        self.index += 1;
        tile
    }
}
