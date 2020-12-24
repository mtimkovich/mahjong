mod mahjong;
use mahjong::Mahjong;

fn main() {
    let mahjong = Mahjong::new();
    println!("{:?}", mahjong.tiles);
}
