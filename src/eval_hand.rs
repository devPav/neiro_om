use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{Card, Hand, Rank, MAP_INLINE_REALCOMB};

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum ReadyHand {
    HightCards(Rank, Rank, Rank, Rank, Rank),
    OnePair {
        pair: Rank,
        top_kicker: Rank,
        mid_kicker: Rank,
        low_kicker: Rank,
    },
    TwoPair {
        top: Rank,
        bottom: Rank,
        kicker: Rank,
    },
    Trips {
        trips: Rank,
        top_kicker: Rank,
        low_kicker: Rank,
    },
    Street(Rank),
    Flash(Rank, Rank, Rank, Rank, Rank),
    FullHouse {
        trips: Rank,
        pair: Rank,
    },
    Care(Rank),
    StreetFlash(Rank),
    FlashRoal,
}
impl ReadyHand {
    // Must eq to Display and Debug
    pub fn from_str(s: &str) -> Option<Self> {
        if s.contains("FlashRoal") {
            Some(ReadyHand::FlashRoal)
        } else if s.contains("Streetflash") {
            let temp = s.replace("Streetflash:", "");
            let temp = temp.trim();
            let rank = Rank::from_str(&temp).expect("wrong Rank street flash");
            Some(ReadyHand::StreetFlash(rank))
        } else if s.contains("Care") {
            let temp = s.replace("Care:", "");
            let temp = temp.trim();
            let rank = Rank::from_str(&temp).expect("wrong Rank care");
            Some(ReadyHand::Care(rank))
        } else if s.contains("Full") {
            let temp = s.replace("Full<3,2>:", "");
            let ranks = Rank::from_str_to_vec(&temp);
            Some(ReadyHand::FullHouse {
                trips: ranks[0],
                pair: ranks[1],
            })
        } else if s.contains("Flash") {
            let temp = s.replace("Flash:", "");
            let ranks = Rank::from_str_to_vec(&temp);
            Some(Self::Flash(
                ranks[0], ranks[1], ranks[2], ranks[3], ranks[4],
            ))
        } else if s.contains("Street") {
            let temp = s.replace("Street:", "");
            let ranks = Rank::from_str_to_vec(&temp);
            Some(Self::Street(ranks[0]))
        } else if s.contains("Trips") {
            let temp = s.replace("Trips<trips,tk,lk>:", "");
            let ranks = Rank::from_str_to_vec(&temp);
            Some(Self::Trips {
                trips: ranks[0],
                top_kicker: ranks[1],
                low_kicker: ranks[2],
            })
        } else if s.contains("TwoPair") {
            let temp = s.replace("TwoPair<top,bot,k>:", "");
            let ranks = Rank::from_str_to_vec(&temp);
            Some(Self::TwoPair {
                top: ranks[0],
                bottom: ranks[1],
                kicker: ranks[2],
            })
        } else if s.contains("Pair<pair,tk,mk,lk>:") {
            let temp = s.replace("Pair<pair,tk,mk,lk>:", "");
            let ranks = Rank::from_str_to_vec(&temp);
            Some(Self::OnePair {
                pair: ranks[0],
                top_kicker: ranks[1],
                mid_kicker: ranks[2],
                low_kicker: ranks[3],
            })
        } else if s.contains("HightCards") {
            let temp = s.replace("HightCards:", "");
            let ranks = Rank::from_str_to_vec(&temp);
            Some(Self::HightCards(
                ranks[0], ranks[1], ranks[2], ranks[3], ranks[4],
            ))
        } else {
            None
        }
    }
}
impl Debug for ReadyHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let present = match self {
            Self::FlashRoal => format!("FlashRoal:"),
            Self::StreetFlash(r) => format!("Streetflash: {:?}", r),
            Self::Care(r) => format!("Care: {:?}", r),
            Self::FullHouse { trips: t, pair: p } => format!("Full<3,2>: {:?} {:?}", t, p),
            Self::Flash(a, b, c, d, e) => {
                format!("Flash: {:?} {:?} {:?} {:?} {:?}", a, b, c, d, e)
            }
            Self::Street(r) => format!("Street: {:?}", r),
            Self::Trips {
                trips,
                top_kicker,
                low_kicker,
            } => format!(
                "Trips<trips,tk,lk>: {:?} {:?} {:?}",
                trips, top_kicker, low_kicker
            ),
            Self::TwoPair {
                top,
                bottom,
                kicker,
            } => format!("TwoPair<top,bot,k>: {:?} {:?} {:?}", top, bottom, kicker),
            Self::OnePair {
                pair,
                top_kicker,
                mid_kicker,
                low_kicker,
            } => format!(
                "Pair<pair,tk,mk,lk>: {:?} {:?} {:?} {:?}",
                pair, top_kicker, mid_kicker, low_kicker
            ),
            Self::HightCards(a, b, c, d, e) => {
                format!("HightCards: {:?} {:?} {:?} {:?} {:?}", a, b, c, d, e)
            }
        };
        write!(f, "{}", present)
    }
}
pub fn real_comb(hand: &Hand, board: &Vec<Card>) -> ReadyHand {
    // Get all variants of five cards.
    let mut pair_cards_from_hand = vec![];
    for i in 0..hand.cards.len() {
        for j in i + 1..hand.cards.len() {
            let v = vec![hand.cards[i], hand.cards[j]];
            pair_cards_from_hand.push(v);
        }
    }
    // println!("{:?}->{:?}", hand, pair_cards_from_hand);
    let mut trips_cards_from_board = vec![];
    for i in 0..board.len() {
        for j in i + 1..board.len() {
            for k in j + 1..board.len() {
                let v = vec![board[i], board[j], board[k]];
                trips_cards_from_board.push(v);
            }
        }
    }
    // println!("{:?}->{:?}", board, trips_cards_from_board);
    let mut all_five_cards = vec![];
    for pair in pair_cards_from_hand.iter() {
        for trips in trips_cards_from_board.iter() {
            let mut v = pair.clone();
            v.extend_from_slice(&trips);
            all_five_cards.push(v);
        }
    }
    //println!("{:?}", all_five_cards);
    let mut all_ready_hands = vec![];
    for five_cards in all_five_cards.iter_mut() {
        // all_ready_hands.push(combination(five_cards));

        five_cards.sort_unstable_by(|a, b| b.cmp(a));
        let mut key = five_cards
            .iter()
            .map(|i| format!("{},", i))
            .collect::<String>();
        key.pop();
        let comb = *MAP_INLINE_REALCOMB.get(&key).unwrap();
        all_ready_hands.push(comb);
    }
    all_ready_hands.sort_unstable();

    *all_ready_hands.last().unwrap_or_else(|| unreachable!())
}
pub fn combination(five_cards: &Vec<Card>) -> ReadyHand {
    /* Логика:
    - Начинается с самых сильных, если таковой нет, то переходит к более слабой.
    Потомучто моут быть случаи когда рука подходит под категория ХайКард и Стрит и Флеш, одновременно
    - АПДЕЙТ: для оптимизации скорости меняю логику и некоторые наиболее частые комбинации
    проверяю первыми. Основа оптимизации это map
    Если пара(4), флеш(5), стрит(5), каре(2), фуллхаус(2), трипс(3), две пары(3), старшие карты(5)
    +1. Пара имеет уникальную map и встречается в 55% на флопе, 50% на терне.
    +2. Вынести ранги их сортировку и мэпу из всех процедур.
    */
    let mut ranks = five_cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    ranks.sort_unstable();
    let mut map = HashMap::new();
    ranks.iter().for_each(|&rank| {
        let v = map.entry(rank).or_insert(0u8);
        *v += 1;
    });
    // One pair
    let onepair = is_onepair(&map);
    if let Some((pair, top_kicker, mid_kicker, low_kicker)) = onepair {
        return ReadyHand::OnePair {
            pair,
            top_kicker,
            mid_kicker,
            low_kicker,
        };
    }
    // Flash roal, street flash:
    let street = is_street(&ranks);
    let flash = is_flash(&five_cards, &ranks);
    if let (Some(_), Some(rank_street)) = (flash, street) {
        match rank_street {
            Rank::Ace => return ReadyHand::FlashRoal,
            r => return ReadyHand::StreetFlash(r),
        }
    }
    // Other made hands, sort strong to weak:
    let care = is_care(&map);
    if let Some(r) = care {
        return ReadyHand::Care(r);
    }
    let fullhouse = is_fullhouse(&map);
    if let Some((trips, pair)) = fullhouse {
        return ReadyHand::FullHouse { trips, pair };
    }
    if let Some((a, b, c, d, e)) = flash {
        return ReadyHand::Flash(a, b, c, d, e);
    }
    if let Some(rank_street) = street {
        return ReadyHand::Street(rank_street);
    }
    let trips = is_trips(&map);
    if let Some((trips, top_kicker, low_kicker)) = trips {
        return ReadyHand::Trips {
            trips,
            top_kicker,
            low_kicker,
        };
    }
    let twopairs = is_twopairs(&map);
    if let Some((top, bottom, kicker)) = twopairs {
        return ReadyHand::TwoPair {
            top,
            bottom,
            kicker,
        };
    }
    let hightcards = is_hightcards(&ranks, &map).unwrap_or_else(|| unreachable!());
    ReadyHand::HightCards(
        hightcards.0,
        hightcards.1,
        hightcards.2,
        hightcards.3,
        hightcards.4,
    )
}
fn is_street(ranks: &Vec<Rank>) -> Option<Rank> {
    /* Логика
    - Все карты разного ранга
    - Все пять карт идут по порядку. Использую дискриминант перечисления для простоты кода.
    Последний дискриминант минус первый = 4
     */
    let mut set = HashSet::new();
    if !ranks.iter().all(|&r| set.insert(r)) {
        return None;
    }

    let first_discriminant = *ranks.first()? as isize;
    let last_discriminant = *ranks.last()? as isize;
    if last_discriminant - first_discriminant == 4 {
        Some(*ranks.last().unwrap())
    } else if ranks == &vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace] {
        // A 2 3 4 5
        Some(Rank::Five)
    } else {
        None
    }
}
fn is_flash(
    five_cards: &Vec<Card>,
    ranks_origin: &Vec<Rank>,
) -> Option<(Rank, Rank, Rank, Rank, Rank)> {
    // Sorted from top to low
    let mut ranks = ranks_origin.clone();
    let first_card_suit = five_cards.first()?.suit;
    if five_cards.iter().all(|c| c.suit == first_card_suit) {
        Some((
            ranks.pop().unwrap(),
            ranks.pop().unwrap(),
            ranks.pop().unwrap(),
            ranks.pop().unwrap(),
            ranks.pop().unwrap(),
        ))
    } else {
        None
    }
}
fn is_care(map: &HashMap<Rank, u8>) -> Option<Rank> {
    for (&k, &v) in map.iter() {
        if v == 4 {
            return Some(k);
        }
    }
    None
}
fn is_fullhouse(map: &HashMap<Rank, u8>) -> Option<(Rank, Rank)> {
    // 1-three, 2-two
    if map.len() != 2 {
        return None;
    }
    let top_rank = map
        .iter()
        .find_map(|(&k, &v)| if v == 3 { Some(k) } else { None });
    let bottom_rank = map
        .iter()
        .find_map(|(&k, &v)| if v == 2 { Some(k) } else { None });
    if top_rank.is_some() && bottom_rank.is_some() {
        Some((top_rank.unwrap(), bottom_rank.unwrap()))
    } else {
        None
    }
}
fn is_trips(map_origin: &HashMap<Rank, u8>) -> Option<(Rank, Rank, Rank)> {
    // 1-trips, 2-hight kicker, 3-low kicker
    let mut map = map_origin.clone();
    if map.len() != 3 {
        return None;
    }
    let trips_rank = map
        .iter()
        .find_map(|(&k, &v)| if v == 3 { Some(k) } else { None });
    if trips_rank.is_none() {
        return None;
    }
    let trips_rank = trips_rank.unwrap_or_else(|| unreachable!());
    map.remove(&trips_rank);
    let mut new_ranks = map.keys().collect::<Vec<&Rank>>();
    new_ranks.sort_unstable();
    Some((trips_rank, *new_ranks[1], *new_ranks[0]))
}
fn is_twopairs(map: &HashMap<Rank, u8>) -> Option<(Rank, Rank, Rank)> {
    // 1-top, 2-bottom, 3-kicker
    if map.len() != 3 {
        return None;
    }
    let mut pairs = map
        .iter()
        .filter(|(_, &v)| v == 2)
        .map(|(&k, _)| k)
        .collect::<Vec<Rank>>();
    let kicker = map
        .iter()
        .filter(|(_, &v)| v == 1)
        .map(|(&k, _)| k)
        .collect::<Vec<Rank>>();
    if pairs.len() != 2 && kicker.len() != 1 {
        None
    } else {
        pairs.sort_unstable();
        Some((pairs[1], pairs[0], kicker[0]))
    }
}
fn is_onepair(map: &HashMap<Rank, u8>) -> Option<(Rank, Rank, Rank, Rank)> {
    // 1-pair, 2-4 sort kickers from strong to weak
    if map.len() != 4 {
        return None;
    }
    let pairs = map
        .iter()
        .filter(|(_, &v)| v == 2)
        .map(|(&k, _)| k)
        .collect::<Vec<Rank>>();
    let mut kickers = map
        .iter()
        .filter(|(_, &v)| v == 1)
        .map(|(&k, _)| k)
        .collect::<Vec<Rank>>();
    if pairs.len() != 1 && kickers.len() != 3 {
        None
    } else {
        kickers.sort_unstable();
        Some((pairs[0], kickers[2], kickers[1], kickers[0]))
    }
}
fn is_hightcards(
    ranks: &Vec<Rank>,
    map: &HashMap<Rank, u8>,
) -> Option<(Rank, Rank, Rank, Rank, Rank)> {
    // Sorted from top to low
    if map.len() != 5 {
        return None;
    }
    Some((ranks[4], ranks[3], ranks[2], ranks[1], ranks[0]))
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::Suit;
    #[test]
    fn common_street() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Harts);
        let cart_5 = Card::new(Rank::Queen, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(Some(Rank::Queen), is_street(&ranks));
    }
    #[test]
    fn un_common_street() {
        let card_1 = Card::new(Rank::Two, Suit::Spades);
        let cart_2 = Card::new(Rank::Three, Suit::Spades);
        let card_3 = Card::new(Rank::Four, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Five, Suit::Harts);
        let cart_5 = Card::new(Rank::Ace, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(Some(Rank::Five), is_street(&ranks));
    }
    #[test]
    fn no_street() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Harts);
        let cart_5 = Card::new(Rank::King, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_street(&ranks));

        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Eight, Suit::Clubs);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Harts);
        let cart_5 = Card::new(Rank::Queen, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_ne!(Some(Rank::Queen), is_street(&ranks));

        let card_1 = Card::new(Rank::Two, Suit::Spades);
        let cart_2 = Card::new(Rank::Three, Suit::Spades);
        let card_3 = Card::new(Rank::Five, Suit::Daemonds);
        let cart_4 = Card::new(Rank::King, Suit::Harts);
        let cart_5 = Card::new(Rank::Ace, Suit::Harts);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_ne!(Some(Rank::Five), is_street(&ranks));

        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Eight, Suit::Clubs);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Ten, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_street(&ranks));
    }
    #[test]
    fn common_flash() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Queen, Suit::Spades);
        let cart_4 = Card::new(Rank::Jack, Suit::Spades);
        let cart_5 = Card::new(Rank::Ten, Suit::Spades);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(
            Some((Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine, Rank::Eight)),
            is_flash(&board, &ranks)
        );
    }
    #[test]
    fn no_flash() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Spades);
        let cart_5 = Card::new(Rank::King, Suit::Spades);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_flash(&board, &ranks));
    }
    #[test]
    fn care() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Nine, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(Some(Rank::Nine), is_care(&map));
    }
    #[test]
    fn no_care() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Ten, Suit::Harts);
        let cart_5 = Card::new(Rank::Nine, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_care(&map));
    }
    #[test]
    fn fullhouse() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(Some((Rank::Nine, Rank::Eight)), is_fullhouse(&map));
    }
    #[test]
    fn no_fullhouse() {
        // Two pair
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::King, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_fullhouse(&map));
        // Trips
        let card_1 = Card::new(Rank::Two, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_fullhouse(&map));
        // Random raw
        let card_1 = Card::new(Rank::Two, Suit::Spades);
        let cart_2 = Card::new(Rank::Ten, Suit::Spades);
        let card_3 = Card::new(Rank::Jack, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_fullhouse(&map));
    }
    #[test]
    fn trips() {
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Ace, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(Some((Rank::Nine, Rank::Ace, Rank::Eight)), is_trips(&map));
    }
    #[test]
    fn no_trips() {
        // Fullhouse
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_trips(&map));
        // Care
        let card_1 = Card::new(Rank::Nine, Suit::Clubs);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_trips(&map));
        // Two pair
        let card_1 = Card::new(Rank::Nine, Suit::Clubs);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Jack, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Eight, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_trips(&map));
        //Street
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Harts);
        let cart_5 = Card::new(Rank::Queen, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_trips(&map));
    }
    #[test]
    fn twopairs() {
        // Fullhouse
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_twopairs(&map));
        // Care
        let card_1 = Card::new(Rank::Nine, Suit::Clubs);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_twopairs(&map));
        // Two pair
        let card_1 = Card::new(Rank::Eight, Suit::Clubs);
        let cart_2 = Card::new(Rank::Eight, Suit::Spades);
        let card_3 = Card::new(Rank::Jack, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Nine, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(
            Some((Rank::Nine, Rank::Eight, Rank::Jack)),
            is_twopairs(&map)
        );
        // Street
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Harts);
        let cart_5 = Card::new(Rank::Queen, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_twopairs(&map));
        // Trips
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Ace, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_twopairs(&map));
    }
    #[test]
    fn onepair() {
        // Fullhouse
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_onepair(&map));
        // Care
        let card_1 = Card::new(Rank::Nine, Suit::Clubs);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_onepair(&map));
        // Two pair
        let card_1 = Card::new(Rank::Eight, Suit::Clubs);
        let cart_2 = Card::new(Rank::Eight, Suit::Spades);
        let card_3 = Card::new(Rank::Jack, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Nine, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_onepair(&map));
        // Street
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Harts);
        let cart_5 = Card::new(Rank::Queen, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_onepair(&map));
        // Trips
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Ace, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_onepair(&map));
        // One pair
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Ten, Suit::Harts);
        let cart_5 = Card::new(Rank::Jack, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(
            Some((Rank::Ten, Rank::Jack, Rank::Nine, Rank::Eight)),
            is_onepair(&map)
        );
    }
    #[test]
    fn hightcards() {
        // Fullhouse
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_hightcards(&ranks, &map));
        // Care
        let card_1 = Card::new(Rank::Nine, Suit::Clubs);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Eight, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_hightcards(&ranks, &map));
        // Two pair
        let card_1 = Card::new(Rank::Eight, Suit::Clubs);
        let cart_2 = Card::new(Rank::Eight, Suit::Spades);
        let card_3 = Card::new(Rank::Jack, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Nine, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_hightcards(&ranks, &map));
        // Street or hight cards
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Jack, Suit::Harts);
        let cart_5 = Card::new(Rank::Queen, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(
            Some((Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine, Rank::Eight)),
            is_hightcards(&ranks, &map)
        );
        // Trips
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Nine, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Nine, Suit::Harts);
        let cart_5 = Card::new(Rank::Ace, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_hightcards(&ranks, &map));
        // One pair
        let card_1 = Card::new(Rank::Eight, Suit::Spades);
        let cart_2 = Card::new(Rank::Nine, Suit::Spades);
        let card_3 = Card::new(Rank::Ten, Suit::Daemonds);
        let cart_4 = Card::new(Rank::Ten, Suit::Harts);
        let cart_5 = Card::new(Rank::Jack, Suit::Clubs);
        let board = vec![card_1, cart_2, cart_5, card_3, cart_4];
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        ranks.sort_unstable();
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        assert_eq!(None, is_hightcards(&ranks, &map));
    }
    #[test]
    #[ignore = "It's a visual test"]
    fn comboes() {
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        ];
        let hand = Hand::rnd_hand(&board);
        let ready_comb = real_comb(&hand, &board);
        println!("({:?}) on {:?} is {:?}", hand, board, ready_comb);
        assert!(false);
    }
    #[test]
    fn cmp_diff_group_diff_comboes() {
        // (Js Jh Tc 7h) on [Ac, Kc, Qc, Jc, 9c] is Street: A сравнение стрита и сета
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        ];
        let ready_comb_ = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ace), ready_comb_);
        // (Kd Ts Td 6d) on [Ac, Kc, Qc, Jc, 9c] is Street: A сравнение стрита и пары
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
            Card::new(Rank::Six, Suit::Daemonds),
        )
        .unwrap();
        let ready_comb_ = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ace), ready_comb_);
    }
    #[test]
    #[ignore = "Visual cmp in one group"]
    fn cmp_eq_group_diff_comboes() {
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        ];
        // Сравнение силы рук внутри флеша
        let hand_1 = Hand::new(
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Harts),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        )
        .unwrap();
        let hand_2 = Hand::new(
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Harts),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        let ready_comb_1 = real_comb(&hand_1, &board);
        let ready_comb_2 = real_comb(&hand_2, &board);
        if ready_comb_1 > ready_comb_2 {
            println!("{:?} > {:?}", ready_comb_1, ready_comb_2)
        }
        // Сравнение силы рук внутри двух пар
        let hand_1 = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        )
        .unwrap();
        let hand_2 = Hand::new(
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Three, Suit::Harts),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        let hand_3 = Hand::new(
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        )
        .unwrap();
        let ready_comb_1 = real_comb(&hand_1, &board);
        let ready_comb_2 = real_comb(&hand_2, &board);
        let ready_comb_3 = real_comb(&hand_3, &board);
        if ready_comb_1 > ready_comb_2 && ready_comb_2 > ready_comb_3 {
            println!(
                "{:?} > {:?} > {:?}",
                ready_comb_1, ready_comb_2, ready_comb_3
            )
        }
        // Сравнение силы рук внутри фулл-хауза
        let board = vec![
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Harts),
        ];
        let hand_1 = Hand::new(
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Harts),
            Card::new(Rank::Six, Suit::Clubs),
        )
        .unwrap();
        let hand_2 = Hand::new(
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        let ready_comb_1 = real_comb(&hand_1, &board);
        let ready_comb_2 = real_comb(&hand_2, &board);
        if ready_comb_1 > ready_comb_2 {
            println!("{:?} > {:?}", ready_comb_1, ready_comb_2)
        }

        assert!(false);
    }
}
