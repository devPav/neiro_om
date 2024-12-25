use crate::{Card, Game, Player, Position, Pot, Rank, MAP_INLINE_RANKS_RIVER};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use super::fake_postflop::Utils;
#[derive(Clone)]
pub struct PostflopGame {
    //pub cards: [Card; 3],
    pub cards: Vec<Card>, // always sort from top to low!!! Eval fake hand, strret fake etc
    pub players: Vec<Player>,
    pub positions_and_money: HashMap<Position, Decimal>,
    pub folded_positions: HashSet<Position>,
    pub main_pot: Pot,
    pub min_bet: Decimal,
    pub dead_cards: Vec<Card>,
}
impl Debug for PostflopGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::new();
        self.cards.iter().for_each(|&card| {
            board_str = format!("{}{:?} ", board_str, card);
        });
        write!(
            f,
            "{}pot:{:?} pre_end {}",
            board_str, self.main_pot.value, self.main_pot.prev_street_end_size
        )
    }
}
impl Game for PostflopGame {
    fn cards(&self) -> Option<&Vec<Card>> {
        Some(&self.cards)
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
    fn dead_cards(&self) -> &Vec<Card> {
        &self.dead_cards
    }
    fn min_bet(&self) -> Decimal {
        self.min_bet
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
        false
    }
}
impl PostflopGame {
    pub fn from(init_game: &impl Game) -> Self {
        let postflop_players = PostflopGame::recalc_player_stacks(init_game, &init_game.players());
        let mut new_dead_cards = init_game.dead_cards().clone();
        Self {
            cards: Self::rnd_board_and_modify_deadcards(init_game, &mut new_dead_cards),
            players: postflop_players,
            positions_and_money: HashMap::from([
                (Position::Utg, dec!(0)),
                (Position::Mp, dec!(0)),
                (Position::Co, dec!(0)),
                (Position::Btn, dec!(0)),
                (Position::Sb, dec!(0)),
                (Position::Bb, dec!(0)),
            ]),
            folded_positions: init_game.folded_positions().clone(),
            main_pot: {
                let mut pot = init_game.main_pot().clone();
                pot.prev_street_end_size = pot.value;
                pot
            },
            min_bet: dec!(1),
            dead_cards: new_dead_cards,
        }
    }
    pub fn rnd_board_and_modify_deadcards(
        game: &impl Game,
        dead_cards: &mut Vec<Card>,
    ) -> Vec<Card> {
        let size_gen = if game.is_preflop_game() { 3 } else { 1 };
        let mut set = HashSet::with_capacity(size_gen);
        while set.len() < size_gen {
            let card = Card::rnd_card();
            if dead_cards.contains(&card) {
                continue;
            }
            set.insert(card);
        }
        let mut cards = set.into_iter().collect::<Vec<Card>>();
        dead_cards.extend_from_slice(&cards);
        if let Some(v) = game.cards() {
            cards.extend_from_slice(v)
        }
        PostflopGame::new_sorted_board_cards(cards).unwrap_or_else(|_| unreachable!())
    }
    pub fn new_sorted_board_cards(mut cards: Vec<Card>) -> Result<Vec<Card>, String> {
        let mut uniq_set = HashSet::new();
        let is_uniq_cards = cards.iter().all(|e| uniq_set.insert(e));
        if is_uniq_cards {
            cards.sort_unstable_by(|a, b| b.cmp(a));
            Ok(cards)
        } else {
            Err(String::from(
                "Error: Can't create postflop with non-uniq cards!",
            ))
        }
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
    pub fn street_blockers_to_board(&self) -> Option<Vec<Rank>> {
        // Find blockers to street.
        let cards = &self.cards;
        let ranks = cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();

        /* Основной случай, самый частый. Берем все возможные сочетания из трех карт борда
        и считаем дырки.
        - Если хоть в одной тройке есть спарка, то нужно игнорировать эту тройку, потомучто она точно без стрита
        - Если хоть в одной тройке есть стрит, то проверим на натсовость стрита, если натсовый, то запоминаем эту тройку карт
        - Получив натсовую тройку стритовых карт обхожу ее по рангам и ищу два гэпа это и есть искомые блокеры
        */
        let mut best_three_for_street = None;
        let mut blockers = vec![];
        for i in 0..ranks.len() {
            for j in i + 1..ranks.len() {
                for k in j + 1..ranks.len() {
                    let v = vec![ranks[i], ranks[j], ranks[k]];
                    // Обязательно исключить тройки со спаркой, в них нет стрита и стритдро никогда.
                    let mut set = HashSet::new();
                    if !v.iter().all(|&element| set.insert(element)) {
                        continue;
                    }

                    let first_discriminant = *v.first().unwrap() as isize;
                    let last_discriminant = *v.last().unwrap() as isize;
                    let gap_counts = first_discriminant - last_discriminant - 2;
                    match gap_counts {
                        val if val >= 0 && val <= 2 => {
                            if best_three_for_street.is_some() {
                                let best = best_three_for_street.clone().unwrap();
                                if v > best {
                                    best_three_for_street = Some(v.clone());
                                }
                            } else {
                                best_three_for_street = Some(v.clone());
                            };
                        }
                        _ => {}
                    }
                    // Особый случай стрита и стритдро с тузом снизу.
                    if best_three_for_street.is_none() {
                        let v1 = vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five];
                        let low_street = v.iter().all(|rank| v1.contains(rank));
                        if v[0] == Rank::Ace && low_street {
                            best_three_for_street = Some(v.clone());
                        }
                    }
                }
            }
        }
        if best_three_for_street.is_some() {
            let v = best_three_for_street.unwrap();
            let first_discriminant = v[0] as isize;
            let second_discriminant = v[1] as isize;
            // Для лоу стритов разница между первым и вторым дискриминантом будет равна 9-и и больше (А5*). Для обычных не больше 4-х.
            if first_discriminant == 12 && first_discriminant - second_discriminant >= 9 {
                let set_v1 =
                    HashSet::from([Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five]);
                let set_v = v.into_iter().collect::<HashSet<_>>();
                let diff = set_v1.difference(&set_v);
                let b = diff.collect::<Vec<_>>();
                let mut b = b.iter().map(|&x| *x).collect::<Vec<_>>();
                blockers.append(&mut b);
            } else {
                let full_set = Rank::to_vec_from_low().into_iter().collect::<HashSet<_>>();
                let set = v.into_iter().collect::<HashSet<_>>();
                let diff = full_set.difference(&set);
                let mut max_after_diff = diff
                    .filter(|&&rank| rank as isize <= first_discriminant + 2)
                    .map(|r| *r)
                    .collect::<Vec<_>>();
                max_after_diff.sort_unstable_by(|a, b| b.cmp(a));
                // dbg!(&max_after_diff);
                blockers.push(max_after_diff[0]);
                blockers.push(max_after_diff[1]);
            }
        }
        // Если борд содержит какой-то блокер, то блокеров на стрит нет AQJT9, A5433. Но A5533 нормально.3
        blockers.retain(|x| !ranks.contains(x));
        if blockers.len() < 2 {
            None
        } else {
            Some(blockers)
        }
    }
    pub fn flash_blockers_to_board(&self) -> Option<Card> {
        let mut map_count_of_suits = HashMap::with_capacity(4);
        self.cards.iter().for_each(|&card| {
            map_count_of_suits
                .entry(card.suit)
                .and_modify(|e| *e += 1)
                .or_insert(1u8);
        });
        let flash_suit = map_count_of_suits.into_iter().find(|(_, v)| *v == 3);
        if flash_suit.is_some() {
            let flash_suit = flash_suit.unwrap().0;
            for cur_disc in (0..=12).rev() {
                let cur_rank = Rank::from_discriminant(cur_disc);
                let cur_card = Card::new(cur_rank, flash_suit);
                if !self.cards.contains(&cur_card) {
                    return Some(cur_card);
                }
            }
            unreachable!()
        } else {
            None
        }
    }
    fn recalc_player_stacks(game: &impl Game, players: &Vec<Player>) -> Vec<Player> {
        let mut new_players = players.clone();
        new_players.iter_mut().for_each(|player| {
            player.stack_size -= game
                .positions_and_money()
                .get(&player.position)
                .unwrap_or_else(|| unreachable!());
        });
        new_players
    }
}

mod tests_flop;
