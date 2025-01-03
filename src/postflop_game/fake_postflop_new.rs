use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{eval_hand::real_comb, AgroStreet, FakeBoardNew, FakePostflopHand, Position};
use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use super::{
    eval_fake_hand::{fake_comb_side_fd, fake_comb_side_ready, fake_comb_side_sd},
    fake_postflop::Utils,
    PostflopGame,
};

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Spr {
    Deep,   // In neiro = 10
    Middle, // from 1 to 2. In neiro = 5
    Low,    // In neiro = 1
}
impl Spr {
    pub fn from(val: Decimal) -> Self {
        match val {
            c if c == dec!(200) => Self::Deep,
            c if c == dec!(53) => Self::Middle,
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

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Serialize, Deserialize)]
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
impl FakePostflopNew {
    pub fn from(game: &PostflopGame, position: Position) -> Self {
        let player = game.player_by_position_as_ref(position);
        let combination = real_comb(&player.hand, &game.cards);

        let fake_hand = FakePostflopHand {
            ready: fake_comb_side_ready(&player.hand, combination, &game.cards),
            flash_draw: fake_comb_side_fd(&player.hand, combination, &game.cards),
            street_draw: fake_comb_side_sd(&player.hand, combination, &game.cards),
        };

        let blockers = Utils::we_have_blockers(
            &player.hand.cards,
            &Utils::new_fake_flop_board(&game),
            &game,
        );

        let spr = player.stack_size;

        FakePostflopNew {
            // river: 4*15*2*2*3*3=2160
            fake_board: Utils::new_fake_flop_board(&game),
            my_fake_hand: fake_hand,
            blockers,
            ch_board_str: false,
            prev_agr: AgroStreet::NoOne,
            spr: Spr::from(spr),
        }
    }
}
