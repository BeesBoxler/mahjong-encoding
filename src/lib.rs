#![allow(dead_code)]
mod lookup;

use lookup::{ALPHABET, INDEX};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Dots(u8),
    Bamboo(u8),
    Characters(u8),
    Wind(Wind),
    Dragon(Dragon),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dragon {
    White,
    Red,
    Green,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Wind {
    South,
    East,
    North,
    West,
}

pub const R5: u8 = 0xA;

pub enum DecodeErr {
    InvalidCharacter,
}

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

    pub fn from_string(input: &str) -> Result<Vec<Suit>, DecodeErr> {
        input
            .as_bytes()
            .iter()
            .map(|tile| match INDEX[*tile as usize] {
                Some(v) => Ok(v),
                None => Err(DecodeErr::InvalidCharacter),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use std::iter::zip;

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

    #[test]
    fn deserializes_hand_correctly() {
        let tiles = [
            Suit::Characters(2),
            Suit::Characters(3),
            Suit::Characters(4),
            Suit::Characters(5),
            Suit::Characters(6),
            Suit::Characters(7),
            Suit::Dots(4),
            Suit::Dots(5),
            Suit::Dots(6),
            Suit::Dots(7),
            Suit::Dots(7),
            Suit::Bamboo(4),
            Suit::Bamboo(5),
            Suit::Bamboo(6),
        ];
        let input = Suit::from_string("yz0123UVWXXklm").ok().unwrap();

        zip(tiles, input).for_each(|(a,b)| assert_eq!(a,b));
    }

    #[test]
    fn serializes_hand_correctly() {
        let tiles = [
            Suit::Characters(2),
            Suit::Characters(3),
            Suit::Characters(4),
            Suit::Characters(5),
            Suit::Characters(6),
            Suit::Characters(7),
            Suit::Dots(4),
            Suit::Dots(5),
            Suit::Dots(6),
            Suit::Dots(7),
            Suit::Dots(7),
            Suit::Bamboo(4),
            Suit::Bamboo(5),
            Suit::Bamboo(6),
        ];
        assert_eq!(Suit::to_string(&tiles), "yz0123UVWXXklm");
    }
}
