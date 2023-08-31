//! A library for handling mahjong tiles and suits and exporting in a plain text format
//!
//! Provides enums for each suit and each tile within that, any suit can then be
//! exported in a plain text format or reimported from text. This is extra useful
//! when transmitting over plain text formats such as email or sms.
//!

#![warn(missing_docs)]
#![doc(html_logo_url = "https://boxler.me/img/red_reagon.jpg")]
mod lookup;

use lookup::{ALPHABET, INDEX};

/// 数牌 _(suupai)_,
/// used to define a tile
/// 
/// The Suit used to define a tile. A hand, for example should be a `Vec<Suit>`.
/// Used in conjunction with [RED_FIVE], [Dragon] or [Wind].
/// 
/// ```rust
/// let hand = vec![
///     Suit::Dots(RED_FIVE),
///     Suit::Dots(6u8),
///     Suit::Dots(7u8),
/// ];
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    /// 餅子 _(pinzu)_
    Dots(u8),
    /// 索子 _(so-zu)_
    Bamboo(u8),
    /// 萬子 _(manzu)_
    Characters(u8),
    /// Wind, must contain a [Wind]
    /// 風牌 _(fompai)_
    Wind(Wind),
    /// Dragon, must contain a [Dragon]
    /// 三元牌 _(sangempai)_
    Dragon(Dragon),
}

/// 三元牌 _(sangempai)_,
/// Dragon honours, to be used as part of a suit 
/// ```rust
/// Suit::Dragon(Dragon::Green)
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dragon {
    /// 白 _(shiro)_
    White,
    /// 中 _(chun)_
    Red,
    /// 發 _(hatsu)_
    Green,
}

/// 風牌 _(fompai)_,
/// Wind honours, to be used as part of a suit
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Wind {
    /// 南 _(nan)_
    South,
    /// 東 _(ton)_
    East,
    /// 北 _(pei)_
    North,
    /// 西 _(sha)_
    West,
}

/// Red Five
/// 赤牌 _(akapai)_
pub const RED_FIVE: u8 = 0xA;

/// Errors that can be thrown when converting from string -> tiles
pub enum DecodeErr {
    /// The character you used does not refer to a tile
    InvalidCharacter,
}

/// Defines what can be converted from `T` into a [u8]
pub trait ToByte {
    /// Converts from `T` into [u8]
    /// 
    /// ```
    /// Suit::Dots(5u8).to_byte();
    /// ```
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
    /// Converts an array or vec of [Suit] into a plain text string
    pub fn to_string(hand: &[Suit]) -> String {
        String::from_utf8(
            hand.iter()
                .map(|tile| ALPHABET[tile.to_byte() as usize])
                .collect(),
        )
        .unwrap()
    }

    /// Converts from a plain text string into a hand. Can throw a [DecodeErr]
    /// 
    /// ```
    /// Suit::from_string("yz0123UVWXXklm");
    /// ```
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

        zip(tiles, input).for_each(|(a, b)| assert_eq!(a, b));
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
