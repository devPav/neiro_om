use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::time::Instant;
use std::{fs, io};

use crate::eval_hand::{self, real_comb};
use crate::postflop_game::fake_postflop::{FakeBoardStruct, FakeSuitPostFlop, Utils};
use crate::postflop_game::{flop, FakeBoard, FakeStreet, PostflopGame};
use crate::{preflop, Card, FakeHand, Game, Position, PreflopGame, Rank, ReadyHand, Suit};

pub fn test() {
    let MAP_INLINE_RANKS_RIVER: BTreeMap<String, FakeBoard> =
        from_inline_fakeboard_ranks().expect("Didn't find fakeboard_ranks_river.txt");
    let MAP_INLINE_SUITS_RIVER: BTreeMap<String, FakeBoard> =
        from_inline_fakeboard_suits().expect("Didn't find fakeboard_suits_river.txt");
    let preflop = PreflopGame::new();
    let flop = PostflopGame::from(&preflop);
    let turn = PostflopGame::from(&flop);
    let river = PostflopGame::from(&turn);
    let time = Instant::now();
    for _ in 1..=1_000_000 {
        let fake_board = Utils::fake_flop_board(&river);
    }
    println!("Seconds gone old: {}", time.elapsed().as_secs());

    let time = Instant::now();
    for _ in 1..=1_000_000 {
        let fake_board =
            Utils::fake_flop_board_inline(&river, &MAP_INLINE_RANKS_RIVER, &MAP_INLINE_SUITS_RIVER);
    }
    println!("Seconds gone new: {}", time.elapsed().as_secs());
}
pub fn inline_fakeboard() {
    let mut map = BTreeMap::new();
    let mut map_suit = BTreeMap::new();
    for _ in 1..=10_000_000 {
        let preflop = PreflopGame::new();
        let flop = PostflopGame::from(&preflop);
        let turn = PostflopGame::from(&flop);
        let river = PostflopGame::from(&turn);

        let fake_board = Utils::fake_flop_board(&river);

        let ranks = river.cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        map.entry(ranks.clone()).or_insert(fake_board.clone());

        let suits = river.cards.iter().map(|c| c.suit).collect::<Vec<Suit>>();
        map_suit.entry(suits.clone()).or_insert(fake_board.clone());
    }
    // for (key, fake_board) in map {
    //     let print_key = key.iter().map(|&r| format!("{:?}", r)).collect::<String>();
    //     println!(
    //         "{}|{:?}|{:?}|{:?}",
    //         print_key, fake_board.street_kind, fake_board.paired, fake_board.rank_struct
    //     );
    // }
    // for (key, fake_board) in map_suit {
    //     let print_key = key.iter().map(|&r| format!("{:?}", r)).collect::<String>();
    //     println!("{}|{:?}", print_key, fake_board.suit_kind);
    // }
}
pub fn inline_real_combination() {
    /*
    Убрать повторения
    Разбить как-то расчет, т.к. будет 4 000 000 000 записей
     */
    let time = Instant::now();
    let mut map = BTreeMap::new();

    while map.len() < 2_598_960 {
        let mut set = HashSet::with_capacity(5);
        while set.len() < 5 {
            set.insert(Card::rnd_card());
        }
        let mut cards = set.into_iter().collect::<Vec<_>>();
        cards.sort_unstable_by(|a, b| b.cmp(a));

        if !map.contains_key(&cards) {
            let comb = eval_hand::combination(&cards);
            map.insert(cards, comb);
        }
    }
    map.iter().for_each(|(k, v)| {
        let s = format!(
            "{:?},{:?},{:?},{:?},{:?}|{:?}",
            k[0], k[1], k[2], k[3], k[4], v
        );
        println!("{}", s);
    });

    println!("Seconds gone: {}", time.elapsed().as_secs());
}
pub fn from_inline_real_combination() -> io::Result<BTreeMap<String, ReadyHand>> {
    let file = File::open("real_com_river.txt")?;
    let reader = io::BufReader::new(file);

    let mut map = BTreeMap::new();
    for line in reader.lines() {
        let ln = line?;
        let mut iter = ln.split('|');
        let key = iter.next().unwrap();
        let key_comb = iter.next().unwrap();

        let hand = ReadyHand::from_str(key_comb);

        map.insert(key.to_owned(), hand.expect("Can't parse ready hand"));
    }
    Ok(map)
}
pub fn from_inline_fakeboard_ranks() -> io::Result<BTreeMap<String, FakeBoard>> {
    let file = File::open("fakeboard_ranks_river.txt")?;
    let reader = io::BufReader::new(file);

    let mut map = BTreeMap::new();
    for line in reader.lines() {
        let ln = line?;
        let mut iter = ln.split('|');
        let key = iter.next().unwrap();

        let s = iter.next().unwrap();
        let street_kind = FakeStreet::from_str(s);

        let s = iter.next().unwrap();
        let paired = if s == "true" { true } else { false };

        let s = iter.next().unwrap();
        let rank_struct = FakeBoardStruct::from_str(s);

        let suit_kind = FakeSuitPostFlop::Rainbow;

        let fake_hand = FakeBoard {
            suit_kind,
            street_kind,
            paired,
            rank_struct,
        };
        map.insert(key.to_owned(), fake_hand);
    }
    Ok(map)
}
pub fn from_inline_fakeboard_suits() -> io::Result<BTreeMap<String, FakeBoard>> {
    let file = File::open("fakeboard_suits_river.txt")?;
    let reader = io::BufReader::new(file);

    let mut map = BTreeMap::new();
    for line in reader.lines() {
        let ln = line?;
        let mut iter = ln.split('|');
        let key = iter.next().unwrap();

        let street_kind = FakeStreet::Dry;
        let paired = false;
        let rank_struct = FakeBoardStruct::X;

        let s = iter.next().unwrap();
        let suit_kind = FakeSuitPostFlop::from_str(s);

        let fake_hand = FakeBoard {
            suit_kind,
            street_kind,
            paired,
            rank_struct,
        };
        map.insert(key.to_owned(), fake_hand);
    }
    Ok(map)
}
pub fn fake_board_from_inline(
    map_r: &BTreeMap<String, FakeBoard>,
    map_s: &BTreeMap<String, FakeBoard>,
    cards: &Vec<Card>,
) -> FakeBoard {
    let ranks = cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    let print_key = ranks
        .iter()
        .map(|&r| format!("{:?}", r))
        .collect::<String>();
    let mut fake_board = *map_r
        .get(&print_key)
        .expect("Didn't find key-rank from inline map");

    let suits = cards.iter().map(|c| c.suit).collect::<Vec<Suit>>();
    let print_key = suits
        .iter()
        .map(|&r| format!("{:?}", r))
        .collect::<String>();
    fake_board.suit_kind = map_s
        .get(&print_key)
        .expect("Didn't find key-suit from inline map")
        .suit_kind;
    // fake_board.suit_kind = FakeSuitPostFlop::Rainbow;
    fake_board
}
