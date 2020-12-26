mod mahjong;
use mahjong::Mahjong;

fn main() {
    let mut mahjong = Mahjong::new();
    for _ in 0..13 {
        let tile = mahjong.draw().expect("out of tiles");
        println!("{}", tile);
    }
}
