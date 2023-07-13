mod tiles;

use tiles::{Suit, ToByte};

fn main() {
    const MAX: u8 = 0b111111;
    println!("{:#x}, {:#08b}, {:#}", MAX, MAX, MAX);

    for suit in [Suit::Dots, Suit::Bamboo, Suit::Characters] {
        for i in 0..0xA {
            println!("{:08b}: {:?}", suit(i).to_byte(), suit(i))
        }
    }
}
