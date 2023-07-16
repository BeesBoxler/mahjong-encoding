#![allow(dead_code)]

const ALPHABET: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2b, 0x2f,
];

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Dots(u8),
    Bamboo(u8),
    Characters(u8),
    Wind(Wind),
    Dragon(Dragon),
}

#[derive(Debug, Copy, Clone)]
pub enum Dragon {
    White,
    Red,
    Green,
}

#[derive(Debug, Copy, Clone)]
pub enum Wind {
    South,
    East,
    North,
    West,
}

pub const R5: u8 = 0xA;

pub trait ToByte {
    fn to_byte(&self) -> u8;
}

impl ToByte for Suit {
    fn to_byte(&self) -> u8 {
        // USE NO VALUE ABOVE 0x3F!!!

        match self {
            Suit::Dots(n) => 0x10 | n & 0xF,
            Suit::Bamboo(n) => 0x20 | n & 0xF,
            Suit::Characters(n) => 0x30 | n & 0xF,
            Suit::Wind(n) => n.to_byte(),
            Suit::Dragon(n) => n.to_byte(),
        }
    }
}

impl ToByte for Wind {
    fn to_byte(&self) -> u8 {
        match self {
            Wind::South => 0x0C,
            Wind::East => 0x1C,
            Wind::North => 0x2C,
            Wind::West => 0x3C,
        }
    }
}

impl ToByte for Dragon {
    fn to_byte(&self) -> u8 {
        match self {
            Dragon::White => 0x0D,
            Dragon::Red => 0x1D,
            Dragon::Green => 0x2D,
        }
    }
}

impl Suit {
    pub fn to_string(hand: &[Suit]) -> String {
        String::from_utf8(
            hand.iter()
                .map(|tile| ALPHABET[tile.to_byte() as usize])
                .collect(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

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

    #[test]
    fn no_value_above_0x3f() {
        get_all_tiles()
            .into_iter()
            .for_each(|tile| assert!(tile.to_byte() <= 0x3F));
    }

    #[test]
    fn no_duplicate_values() {
        let vec = get_all_tiles()
            .iter()
            .map(|tile| tile.to_byte())
            .collect::<Vec<u8>>();
        let hash = get_all_tiles()
            .iter()
            .map(|tile| tile.to_byte())
            .collect::<HashSet<u8>>();

        assert!(vec.len() == hash.len());
    }
}
