use crate::{player, Card, Game, Hand, Player, Position};
use rand::Rng;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};
#[derive(PartialEq, Clone, Copy)]
pub enum ActionKind {
    Fold,
    Call(Decimal),
    Raise(Decimal),
    Check,
}
impl Debug for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fold => "F".to_string(),
                Self::Call(x) => format!("C({:?})", x),
                Self::Raise(x) => format!("R({:?})", x),
                Self::Check => "X".to_string(),
            }
        )
    }
}
impl ActionKind {
    pub fn rnd_action_from(acts: &Vec<Self>) -> Option<Self> {
        match acts.is_empty() {
            true => None,
            _ => {
                let index = rand::thread_rng().gen_range(0..=acts.len() - 1);
                Some(*acts.get(index).expect("Error: empty possibale actions"))
            }
        }
        //rand::random()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pot {
    pub value: Decimal,
    pub members: Vec<Position>,
    pub prev_street_end_size: Decimal,
}
pub struct PreflopGame {
    pub players: Vec<Player>,
    pub positions_and_money: HashMap<Position, Decimal>,
    pub folded_positions: HashSet<Position>,
    pub main_pot: Pot,
    pub min_bet: Decimal,
    pub dead_cards: Vec<Card>,
}
impl Game for PreflopGame {
    fn cards(&self) -> Option<&Vec<Card>> {
        None
    }
    fn players(&self) -> &Vec<Player> {
        &self.players
    }
    fn positions_and_money(&self) -> &HashMap<Position, Decimal> {
        &self.positions_and_money
    }
    fn folded_positions(&self) -> &HashSet<Position> {
        &self.folded_positions
    }
    fn main_pot(&self) -> &Pot {
        &self.main_pot
    }
    fn min_bet(&self) -> Decimal {
        self.min_bet
    }
    fn dead_cards(&self) -> &Vec<Card> {
        &self.dead_cards
    }
    fn set_min_bet(&mut self, value: Decimal) {
        self.min_bet = value;
    }
    fn main_pot_as_mut_ref(&mut self) -> &mut Pot {
        &mut self.main_pot
    }
    fn folded_positions_as_mut_ref(&mut self) -> &mut HashSet<Position> {
        &mut self.folded_positions
    }
    fn positions_and_money_as_mut_ref(&mut self) -> &mut HashMap<Position, Decimal> {
        &mut self.positions_and_money
    }
    fn is_preflop_game(&self) -> bool {
        true
    }
}
impl PreflopGame {
    pub fn new_with_lock_cards(lock_cards: &Vec<Card>) -> Self {
        let mut dead_cards = lock_cards.clone();
        let player_utg = Self::make_player_modify_dedcards_after_it(Position::Utg, &mut dead_cards);
        let player_mp = Self::make_player_modify_dedcards_after_it(Position::Mp, &mut dead_cards);
        let player_co = Self::make_player_modify_dedcards_after_it(Position::Co, &mut dead_cards);
        let player_btn = Self::make_player_modify_dedcards_after_it(Position::Btn, &mut dead_cards);
        let player_sb = Self::make_player_modify_dedcards_after_it(Position::Sb, &mut dead_cards);
        let player_bb = Self::make_player_modify_dedcards_after_it(Position::Bb, &mut dead_cards);

        Self {
            players: vec![
                player_utg, player_mp, player_co, player_btn, player_sb, player_bb,
            ],
            positions_and_money: HashMap::from([
                (Position::Utg, dec!(0)),
                (Position::Mp, dec!(0)),
                (Position::Co, dec!(0)),
                (Position::Btn, dec!(0)),
                (Position::Sb, dec!(0.5)),
                (Position::Bb, dec!(1)),
            ]),
            folded_positions: HashSet::new(),
            main_pot: Pot {
                value: dec!(1.5),
                members: vec![Position::Sb, Position::Bb],
                prev_street_end_size: dec!(0),
            },
            min_bet: dec!(1),
            dead_cards,
        }
    }
    pub fn new() -> Self {
        let mut dead_cards = vec![];
        let player_utg = Self::make_player_modify_dedcards_after_it(Position::Utg, &mut dead_cards);
        let player_mp = Self::make_player_modify_dedcards_after_it(Position::Mp, &mut dead_cards);
        let player_co = Self::make_player_modify_dedcards_after_it(Position::Co, &mut dead_cards);
        let player_btn = Self::make_player_modify_dedcards_after_it(Position::Btn, &mut dead_cards);
        let player_sb = Self::make_player_modify_dedcards_after_it(Position::Sb, &mut dead_cards);
        let player_bb = Self::make_player_modify_dedcards_after_it(Position::Bb, &mut dead_cards);

        Self {
            players: vec![
                player_utg, player_mp, player_co, player_btn, player_sb, player_bb,
            ],
            positions_and_money: HashMap::from([
                (Position::Utg, dec!(0)),
                (Position::Mp, dec!(0)),
                (Position::Co, dec!(0)),
                (Position::Btn, dec!(0)),
                (Position::Sb, dec!(0.5)),
                (Position::Bb, dec!(1)),
            ]),
            folded_positions: HashSet::new(),
            main_pot: Pot {
                value: dec!(1.5),
                members: vec![Position::Sb, Position::Bb],
                prev_street_end_size: dec!(0),
            },
            min_bet: dec!(1),
            dead_cards,
        }
    }
    fn make_player_modify_dedcards_after_it(
        position: Position,
        dead_cards: &mut Vec<Card>,
    ) -> Player {
        let player = Player::rnd_player(position, &dead_cards);
        for &card in player.hand.cards.iter() {
            dead_cards.push(card);
        }
        player
    }
    pub fn player_by_position_as_ref(&self, position: Position) -> &Player {
        self.players
            .iter()
            .find(|player| player.position == position)
            .unwrap_or_else(|| unreachable!())
    }
    pub fn player_by_position_as_mut_ref(&mut self, position: Position) -> &mut Player {
        self.players
            .iter_mut()
            .find(|player| player.position == position)
            .unwrap_or_else(|| unreachable!())
    }
}
