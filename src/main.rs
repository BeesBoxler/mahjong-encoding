mod tiles;

use tiles::*;

fn main() {
    let tiles = get_all_tiles();

    let hand = Suit::to_string(&tiles);
    println!("{hand}");

    for tile in tiles {
        println!("{:08b} -> {:?}", tile.to_byte(), tile);
    }
}

fn get_all_tiles() -> Vec<Suit> {
    let mut vec = vec![];

    for suit in [Suit::Dots, Suit::Bamboo, Suit::Characters] {
        for i in 0..0xA {
            vec.push(suit(i));
        }
    }

    for wind in [Wind::South, Wind::East, Wind::North, Wind::West] {
        vec.push(Suit::Wind(wind));
    }

    for dragon in [Dragon::White, Dragon::Red, Dragon::Green] {
        vec.push(Suit::Dragon(dragon));
    }

    vec
}
