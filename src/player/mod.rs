use crate::{Card, FakeHand, Hand};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::fmt::Debug;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakeStackSize {
    Shallow,
    Deep,
}
impl Debug for FakeStackSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Shallow => "..75",
                Self::Deep => "75..",
            }
        )
    }
}
impl FakeStackSize {
    pub fn from(value: Decimal) -> Self {
        match value {
            c if c >= dec!(0) && c < dec!(75) => Self::Shallow,
            _ => Self::Deep,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum Position {
    // Order like postflop, because I cmp it in fakepostflop. Was from UTG
    Sb,
    Bb,
    Utg,
    Mp,
    Co,
    Btn,
}
impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        match rng.gen_range(1..=6) {
            1 => Position::Utg,
            2 => Position::Mp,
            3 => Position::Co,
            4 => Position::Btn,
            5 => Position::Sb,
            6 => Position::Bb,
            _ => unreachable!(),
        }
    }
}
impl Position {
    pub fn rnd_position() -> Self {
        rand::random()
    }
    pub fn rnd_two_positions() -> Vec<Self> {
        let positions = vec![Self::rnd_position()];
        // while positions.len() == 1 {
        //     let position = Self::rnd_position();
        //     if !positions.contains(&position) {
        //         positions.push(position);
        //     }
        // }
        positions
    }
    pub fn all_poses() -> Vec<Self> {
        vec![
            Position::Sb,
            Position::Bb,
            Position::Utg,
            Position::Mp,
            Position::Co,
            Position::Btn,
        ]
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Player {
    pub position: Position,
    pub stack_size: Decimal,
    pub hand: Hand,
}
impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}bb)({:?}){:?}",
            self.stack_size, self.position, self.hand
        )
    }
}
impl Player {
    pub fn rnd_player(position: Position, dead_cards: &Vec<Card>) -> Self {
        Self {
            position,
            stack_size: Decimal::new(rand::thread_rng().gen_range(30..=250), 0),
            hand: Hand::rnd_hand(dead_cards),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FakePlayer {
    position: Position,
    stack_size: FakeStackSize,
    hand: FakeHand,
}
impl Debug for FakePlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}bb)({:?}){:?}",
            self.stack_size, self.position, self.hand
        )
    }
}
impl FakePlayer {
    pub fn from(player: &Player) -> Self {
        Self {
            position: player.position,
            stack_size: FakeStackSize::from(player.stack_size),
            hand: FakeHand::from(&player.hand),
        }
    }
}
