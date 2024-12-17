use inline::fakeboard;
use lazy_static::lazy_static;
use postflop_game::FakeBoard;
use rust_decimal_macros::dec;
use std::collections::{BTreeMap, HashMap, HashSet};

pub use action::*;
pub use eval_hand::ReadyHand;
pub use hand::{Card, Hand, Rank, Suit};
pub use hand::{FakeCard, FakeHand, FakeRank};
pub use player::{FakePlayer, FakeStackSize, Player, Position};
pub use postflop_game::{
    AgroStreet, FakePostReadyHand, FakePostflopFD, FakePostflopPause, FakePostflopSD, PostflopGame,
};
pub use preflop_game::{ActionKind, *};
use rust_decimal::Decimal;

pub mod action;
pub mod eval_hand;
pub mod eval_result;
pub mod hand;
pub mod inline;
pub mod player;
pub mod postflop_game;
pub mod preflop_game;
pub mod redis;

lazy_static! {
    pub static ref MAP_INLINE_RANKS_RIVER: BTreeMap<String, FakeBoard> =
        fakeboard::from_inline_fakeboard_ranks().expect("Didn't find fakeboard_ranks_river.txt");
    pub static ref MAP_INLINE_SUITS_RIVER: BTreeMap<String, FakeBoard> =
        fakeboard::from_inline_fakeboard_suits().expect("Didn't find fakeboard_suits_river.txt");
    pub static ref MAP_INLINE_REALCOMB: BTreeMap<String, ReadyHand> =
        fakeboard::from_inline_real_combination().expect("Didn't find real_com_river.txt");
}

/*
- This traits need to implemet polimorphism for action on any street types: PreflopGame, FlopGame, TurnGame, RiverGame.
- Also games have a lot of equal significant behavior.
*/
pub trait Game {
    fn cards(&self) -> Option<&Vec<Card>>;
    fn players(&self) -> &Vec<Player>;
    fn min_bet(&self) -> Decimal;
    fn main_pot(&self) -> &Pot;
    fn folded_positions(&self) -> &HashSet<Position>;
    fn positions_and_money(&self) -> &HashMap<Position, Decimal>;
    fn dead_cards(&self) -> &Vec<Card>;
    fn is_preflop_game(&self) -> bool;
    // For mutate:
    fn main_pot_as_mut_ref(&mut self) -> &mut Pot;
    fn folded_positions_as_mut_ref(&mut self) -> &mut HashSet<Position>;
    fn positions_and_money_as_mut_ref(&mut self) -> &mut HashMap<Position, Decimal>;
    fn set_min_bet(&mut self, value: Decimal);
    // Default:
    fn player_by_position_as_ref(&self, position: Position) -> &Player {
        self.players()
            .iter()
            .find(|player| player.position == position)
            .unwrap_or_else(|| unreachable!())
    }
    // fn player_by_position_as_mut_ref(&mut self, position: Position) -> &mut Player {
    //     self.players()
    //         .iter_mut()
    //         .find(|player| player.position == position)
    //         .unwrap_or_else(|| unreachable!())
    // }
    // Realy important Default for action:
    fn all_fold_or_allin_nomatter_position(&self, position: Position) -> bool {
        self.positions_and_money()
            .iter()
            .filter(|(&pos, _)| pos != position)
            .all(|(&pos, &money)| {
                self.player_by_position_as_ref(pos).stack_size == money
                    || self.folded_positions().contains(&pos)
            })
    }
    fn end_of_street(&self, possible_act: &Vec<ActionKind>, position: Position) -> bool {
        // Если все находятся в алине, кроме тех, кто сфолдил, то конец игры. Иначе если человек в фолде/алине, то идем дальше.
        // Напоминание для тех кто в олине и в фолде список возможных действий пуст.
        let all_fold_or_allin = self.positions_and_money().iter().all(|(&pos, &money)| {
            self.player_by_position_as_ref(pos).stack_size == money
                || self.folded_positions().contains(&pos)
        });
        if possible_act.is_empty() && all_fold_or_allin {
            // println!(
            //     "All fold or allin, and current too. Folded {}",
            //     self.folded_positions().len()
            // );
            return true;
        }
        if possible_act.is_empty()
            && !self.folded_positions().contains(&position)
            && !self.position_in_allin(position)
        {
            // println!(
            //     "I'm not in fold or alin, but can't do action, so end. Folded {}",
            //     self.folded_positions().len()
            // );
            return true;
        }
        false
    }
    fn end_of_hand_five_foldes(&self) -> bool {
        self.folded_positions().len() >= 5
    }
    fn no_money_in_game(&self) -> bool {
        self.positions_and_money().values().sum::<Decimal>() == Decimal::ZERO
    }
    fn position_in_allin(&self, position: Position) -> bool {
        let player = self.player_by_position_as_ref(position);
        let money = *self
            .positions_and_money()
            .get(&position)
            .unwrap_or_else(|| unreachable!());
        player.stack_size == money
    }
    fn do_action_on_position(&mut self, choosen_act: Option<ActionKind>, position: Position) {
        let act = match choosen_act {
            Some(a) => a,
            _ => return,
        };
        match act {
            ActionKind::Raise(x) => {
                self.positions_and_money_as_mut_ref().insert(position, x);
                self.recalculate_min_bet(position);
                self.recalculate_main_pot();
            }
            ActionKind::Call(x) => {
                self.positions_and_money_as_mut_ref().insert(position, x);
                self.recalculate_main_pot();
            }
            ActionKind::Fold => {
                self.folded_positions_as_mut_ref().insert(position);
            }
            ActionKind::Check => {}
        }
    }
    fn recalculate_main_pot(&mut self) {
        let positions_and_money_sum: Decimal = self.positions_and_money().values().sum();
        self.main_pot_as_mut_ref().value =
            positions_and_money_sum + self.main_pot().prev_street_end_size;

        //self.main_pot_as_mut_ref().value = self.positions_and_money().values().sum();
    }
    fn recalculate_min_bet(&mut self, position: Position) {
        let mut map = self.positions_and_money().clone();
        let act_val = map.remove(&position).unwrap_or_else(|| unreachable!());

        let max_commit = map
            .values()
            .max()
            .map(|x| *x)
            .unwrap_or_else(|| unreachable!());

        //self.min_bet() = act_val - max_commit;
        self.set_min_bet(act_val - max_commit);

        if self.min_bet() < dec!(1) {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::hand::*;
    use super::player::*;
    use super::postflop_game::PostflopGame;
    use super::preflop_game::*;
    use std::collections::HashMap;
    use std::collections::HashSet;
    #[test]
    fn create_card_from_string() {
        let s = String::from("As\n");
        let card = Card::from_string_ui(s);
        assert_eq!(card, Card::new(Rank::Ace, Suit::Spades));

        let s = String::from("\nKc\n");
        let card = Card::from_string_ui(s);
        assert_eq!(card, Card::new(Rank::King, Suit::Clubs));

        let s = String::from("        2h       ");
        let card = Card::from_string_ui(s);
        assert_eq!(card, Card::new(Rank::Two, Suit::Harts));

        let s = String::from("3d");
        let card = Card::from_string_ui(s);
        assert_eq!(card, Card::new(Rank::Three, Suit::Daemonds));
    }
    #[test]
    #[should_panic(expected = "error: wrong rank:")]
    fn create_card_from_string_wrong_rank() {
        let s = String::from("Yd");
        let _ = Card::from_string_ui(s);
    }
    #[test]
    #[should_panic(expected = "error: wrong suit:")]
    fn create_card_from_string_wrong_suit() {
        let s = String::from("AS");
        let _ = Card::from_string_ui(s);
    }
    #[test]
    #[should_panic(expected = "error: wrong card len, need to be 2")]
    fn create_card_from_string_wrong_len() {
        let s = String::from("Ass");
        let _ = Card::from_string_ui(s);
    }
    #[test]
    fn cmp_two_cards_with_rank_igore_suit() {
        let card_1 = Card::new(Rank::King, Suit::Spades);
        let cart_2 = Card::new(Rank::Ace, Suit::Spades);
        assert!(card_1 < cart_2);
    }
    #[test]
    fn cmp_two_cards_with_suit_ignore_rank() {
        let card_1 = Card::new(Rank::Ace, Suit::Spades);
        let cart_2 = Card::new(Rank::Ace, Suit::Harts);
        assert!(card_1 > cart_2);
    }
    #[test]
    fn cmp_two_cards_with_rank_and_suit_prioritet_is_rank() {
        let card_1 = Card::new(Rank::Ace, Suit::Daemonds);
        let cart_2 = Card::new(Rank::King, Suit::Spades);
        assert!(card_1 > cart_2);
    }
    #[test]
    fn create_hand_uniq_cards() -> Result<(), String> {
        let _ = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Ten, Suit::Spades),
        )?;
        Ok(())
    }
    #[test]
    fn cant_create_hand_nouniq_cards() {
        let failed_create = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ten, Suit::Clubs),
        )
        .is_err();
        assert!(failed_create)
    }
    #[test]
    fn correct_sort_preflop_hand() {
        let hand_sorted = Hand::new(
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
        )
        .unwrap();
        assert_eq!(
            hand_sorted,
            Hand {
                cards: [
                    Card::new(Rank::Ace, Suit::Clubs),
                    Card::new(Rank::Ten, Suit::Daemonds),
                    Card::new(Rank::Two, Suit::Spades),
                    Card::new(Rank::Two, Suit::Clubs),
                ]
            }
        )
    }
    #[test]
    fn rnd_hand_unreachable_never_panic() {
        for _ in 1..1_000 {
            Hand::rnd_hand(&vec![]);
        }
    }
    #[test]
    fn rnd_hand_and_fakehand_unreachable_never_panic() {
        for _ in 1..1_000 {
            let hand = Hand::rnd_hand(&vec![]);
            FakeHand::from(&hand);
        }
    }
    #[test]
    #[ignore = "IS IMPORTANT: too slow"]
    fn rnd_real_hand_correct_total_count() {
        let mut v = vec![];
        for _ in 1..5_000_000 {
            v.push(Hand::rnd_hand(&vec![]));
        }
        v.sort();
        v.dedup();
        println!("{}", v.len());
        assert!(v.len() > 250_000 && v.len() < 280_000)
    }
    #[test]
    #[ignore = "IS IMPORTANT: too slow"]
    fn rnd_fake_preflop_hand_total_count() {
        let mut set = HashSet::new();
        for _ in 1..10_000_00 {
            let preflop_game = PreflopGame::new();
            preflop_game.players.iter().for_each(|player| {
                let fake_hand = FakeHand::from(&player.hand);
                set.insert(fake_hand);
            })
        }
        println!("Preflop fake hands count: {}", set.len());
        assert!(false);
    }
    #[test]
    fn fd_in_hand_to_rank() {
        let hand = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert!(hand.unwrap().has_fd_to_rank(Rank::Ten));

        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert!(hand.unwrap().has_fd_to_rank(Rank::Ace));

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert!(hand.unwrap().has_fd_to_rank(Rank::Two));

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert!(hand.unwrap().has_fd_to_rank(Rank::King));

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Harts),
        );
        assert!(!hand.unwrap().has_fd_to_rank(Rank::King));

        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert!(!hand.unwrap().has_fd_to_rank(Rank::Jack));
    }
    #[test]
    fn correct_fake_hand() {
        let hand = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert_eq!(
            FakeHand::from(&hand.unwrap()),
            FakeHand {
                cards: [
                    FakeCard {
                        rank: FakeRank::MiddleCard
                    },
                    FakeCard {
                        rank: FakeRank::GarbageCard
                    },
                    FakeCard {
                        rank: FakeRank::GarbageCard
                    },
                    FakeCard {
                        rank: FakeRank::GarbageCard
                    }
                ],
                kind: FakeSuitKind::Ss,
                paired: Pairing::TripsCare,
            }
        );

        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert_eq!(
            FakeHand::from(&hand.unwrap()),
            FakeHand {
                cards: [
                    FakeCard {
                        rank: FakeRank::Ace
                    },
                    FakeCard {
                        rank: FakeRank::Ace
                    },
                    FakeCard {
                        rank: FakeRank::MiddleCard
                    },
                    FakeCard {
                        rank: FakeRank::GarbageCard
                    }
                ],
                kind: FakeSuitKind::Ass,
                paired: Pairing::Paired,
            }
        );

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert_eq!(
            FakeHand::from(&hand.unwrap()),
            FakeHand {
                cards: [
                    FakeCard {
                        rank: FakeRank::King
                    },
                    FakeCard {
                        rank: FakeRank::King
                    },
                    FakeCard {
                        rank: FakeRank::MiddleCard
                    },
                    FakeCard {
                        rank: FakeRank::GarbageCard
                    }
                ],
                kind: FakeSuitKind::Kss,
                paired: Pairing::Paired,
            }
        );

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert_eq!(
            FakeHand::from(&hand.unwrap()),
            FakeHand {
                cards: [
                    FakeCard {
                        rank: FakeRank::King
                    },
                    FakeCard {
                        rank: FakeRank::BigCard
                    },
                    FakeCard {
                        rank: FakeRank::BigCard
                    },
                    FakeCard {
                        rank: FakeRank::MiddleCard
                    }
                ],
                kind: FakeSuitKind::Kss,
                paired: Pairing::NoPaired,
            }
        );

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Harts),
        );
        assert_eq!(
            FakeHand::from(&hand.unwrap()),
            FakeHand {
                cards: [
                    FakeCard {
                        rank: FakeRank::King
                    },
                    FakeCard {
                        rank: FakeRank::BigCard
                    },
                    FakeCard {
                        rank: FakeRank::BigCard
                    },
                    FakeCard {
                        rank: FakeRank::MiddleCard
                    }
                ],
                kind: FakeSuitKind::Os,
                paired: Pairing::NoPaired,
            }
        );

        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert_eq!(
            FakeHand::from(&hand.unwrap()),
            FakeHand {
                cards: [
                    FakeCard {
                        rank: FakeRank::Ace
                    },
                    FakeCard {
                        rank: FakeRank::Ace
                    },
                    FakeCard {
                        rank: FakeRank::MiddleCard
                    },
                    FakeCard {
                        rank: FakeRank::GarbageCard
                    }
                ],
                kind: FakeSuitKind::Ds,
                paired: Pairing::Paired,
            }
        );
    }
    #[test]
    fn hand_is_ds() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Harts),
        );
        assert!(!hand.unwrap().is_double_suited());

        let hand = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert!(!hand.unwrap().is_double_suited());

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
        );
        assert!(!hand.unwrap().is_double_suited());

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
        );
        assert!(hand.unwrap().is_double_suited());
    }
    #[test]
    fn correct_pairing_status() {
        let hand = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Ten, Suit::Spades),
        )
        .unwrap();
        assert_eq!(hand.pairing_status(), Pairing::TripsCare);

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
        )
        .unwrap();
        assert_eq!(hand.pairing_status(), Pairing::Paired);

        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Harts),
        )
        .unwrap();
        assert_eq!(hand.pairing_status(), Pairing::NoPaired);

        let hand = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Harts),
            Card::new(Rank::Ten, Suit::Spades),
        )
        .unwrap();
        assert_eq!(hand.pairing_status(), Pairing::DoublePaired);
    }
    #[test]
    fn rnd_player_and_fakeplayer() {
        let player = Player::rnd_player(Position::Btn, &vec![]);
        let _ = FakePlayer::from(&player);
    }
    #[test]
    fn rnd_preflopgame_unreachable_never_panic() {
        let _ = PreflopGame::new();
    }
    #[test]
    fn all_players_in_game_have_different_real_cards() {
        let game = PreflopGame::new();
        let mut map_cards = HashMap::new();
        for player in game.players.iter() {
            player.hand.cards.iter().for_each(|card| {
                let k = map_cards.entry(card.clone()).or_insert(0u8);
                *k += 1;
            })
        }
        println!("{:?}", map_cards);
        assert_eq!(map_cards.len(), 24);
        assert_eq!(map_cards.values().sum::<u8>(), 24u8);
        assert_eq!(map_cards.values().max(), Some(&1u8));
    }
    #[test]
    fn rnd_flopgame_unreachable_never_panic() {
        for _ in 1..=100 {
            let preflop_game = PreflopGame::new();
            let _ = PostflopGame::from(&preflop_game);
        }
    }
    #[test]
    fn cant_create_flop_turn_cards_nouniq_cards() {
        let failed_create = PostflopGame::new_sorted_board_cards(vec![
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
        ])
        .is_err();
        assert!(failed_create);
        let failed_create = PostflopGame::new_sorted_board_cards(vec![
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Harts),
        ])
        .is_err();
        assert!(failed_create)
    }
    #[test]
    fn rnd_flop_turn_river_cards_always_uniq() {
        for _ in 1..=100 {
            let preflop_game = PreflopGame::new();
            let flop_game = PostflopGame::from(&preflop_game);
            let mut set = HashSet::new();
            assert!(flop_game.cards.iter().all(|&x| set.insert(x)));
        }
        for _ in 1..=100 {
            let preflop_game = PreflopGame::new();
            let flop_game = PostflopGame::from(&preflop_game);
            let turn_game = PostflopGame::from(&flop_game);
            let mut set = HashSet::new();
            assert!(turn_game.cards.iter().all(|&x| set.insert(x)));
        }
        for _ in 1..=100 {
            let preflop_game = PreflopGame::new();
            let flop_game = PostflopGame::from(&preflop_game);
            let turn_game = PostflopGame::from(&flop_game);
            let river_game = PostflopGame::from(&turn_game);
            let mut set = HashSet::new();
            assert!(river_game.cards.iter().all(|&x| set.insert(x)));
        }
    }
    #[test]
    fn rnd_flop_turn_cards_plus_players_hands_always_uniq() {
        for _ in 1..=100 {
            let preflop_game = PreflopGame::new();
            let flop_game = PostflopGame::from(&preflop_game);
            let turn_game = PostflopGame::from(&flop_game);
            let mut set = HashSet::new();
            turn_game.cards.iter().all(|&x| set.insert(x));
            for player in turn_game.players.iter() {
                player.hand.cards.iter().all(|&x| set.insert(x));
            }
            assert_eq!(28, set.len());
            assert_eq!(28, turn_game.dead_cards.len());
            let mut v = turn_game.dead_cards.clone();
            v.sort();
            v.dedup();
            assert_eq!(28, v.len());
        }
    }
}
