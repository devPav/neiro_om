use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{AgroStreet, FakeBoardNew, FakePostflopHand};
use std::fmt::{Debug, Display};

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum Spr {
    Deep,   // In neiro = 10
    Middle, // from 1 to 2. In neiro = 5
    Low,    // In neiro = 1
}
impl Spr {
    pub fn from(val: Decimal) -> Self {
        match val {
            c if c == dec!(200) => Self::Deep,
            c if c == dec!(100) => Self::Middle,
            c if c == dec!(20) => Self::Low,
            _ => unreachable!(),
        }
    }
}
impl Debug for Spr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Deep => "Deep",
                Self::Middle => "Middle",
                Self::Low => "Low",
            }
        )
    }
}
impl Display for Spr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Deep => "Deep",
                Self::Middle => "Middle",
                Self::Low => "Low",
            }
        )
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct FakePostflopNew {
    // river: 4*15*2*2*3*3=2160
    pub fake_board: FakeBoardNew,
    pub my_fake_hand: FakePostflopHand,
    pub blockers: bool,
    pub ch_board_str: bool,
    pub prev_agr: AgroStreet,
    pub spr: Spr,
}
impl Debug for FakePostflopNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "
***Board: {:?}
***Hand {:?}
***Blockers {:?}
***Changed str: {:?}
***Agr prev str: {:?}
***Spr: {:?}
",
            self.fake_board,
            self.my_fake_hand,
            self.blockers,
            self.ch_board_str,
            self.prev_agr,
            self.spr
        );
        write!(f, "{}", s)
    }
}
