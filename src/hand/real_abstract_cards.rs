use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Rank {
    pub fn rnd_rank() -> Self {
        rand::random()
    }
    pub fn to_vec_from_low() -> Vec<Self> {
        vec![
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
            Self::Ace,
        ]
    }
    pub fn from_discriminant(val: isize) -> Self {
        match val {
            0 => Self::Two,
            1 => Self::Three,
            2 => Self::Four,
            3 => Self::Five,
            4 => Self::Six,
            5 => Self::Seven,
            6 => Self::Eight,
            7 => Self::Nine,
            8 => Self::Ten,
            9 => Self::Jack,
            10 => Self::Queen,
            11 => Self::King,
            12 => Self::Ace,
            _ => unreachable!(),
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "2" => Some(Rank::Two),
            "3" => Some(Rank::Three),
            "4" => Some(Rank::Four),
            "5" => Some(Rank::Five),
            "6" => Some(Rank::Six),
            "7" => Some(Rank::Seven),
            "8" => Some(Rank::Eight),
            "9" => Some(Rank::Nine),
            "T" => Some(Rank::Ten),
            "J" => Some(Rank::Jack),
            "Q" => Some(Rank::Queen),
            "K" => Some(Rank::King),
            "A" => Some(Rank::Ace),
            _ => None,
        }
    }
    pub fn from_str_to_vec(s: &str) -> Vec<Self> {
        let mut result = vec![];
        let st = s.trim();
        for part in st.split_ascii_whitespace() {
            result.push(Self::from_str(part).unwrap());
        }
        result
    }
}
impl Distribution<Rank> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        match rng.gen_range(1..=13) {
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            8 => Rank::Nine,
            9 => Rank::Ten,
            10 => Rank::Jack,
            11 => Rank::Queen,
            12 => Rank::King,
            13 => Rank::Ace,
            _ => unreachable!(),
        }
    }
}
impl Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let present = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", present)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum Suit {
    Daemonds,
    Harts,
    Clubs,
    Spades,
}
impl Suit {
    pub fn rnd_suit() -> Self {
        rand::random()
    }
}
impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.gen_range(1..=4) {
            1 => Suit::Daemonds,
            2 => Suit::Harts,
            3 => Suit::Clubs,
            4 => Suit::Spades,
            _ => unreachable!(),
        }
    }
}
impl Debug for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let present = match self {
            Suit::Clubs => "c",
            Suit::Spades => "s",
            Suit::Harts => "h",
            Suit::Daemonds => "d",
        };
        write!(f, "{}", present)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
impl Card {
    pub fn rnd_card() -> Self {
        Card {
            rank: Rank::rnd_rank(),
            suit: Suit::rnd_suit(),
        }
    }
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }
    pub fn from_string_ui(s: String) -> Self {
        let ss = s.trim();
        if ss.len() != 2 {
            panic!("error: wrong card len, need to be 2")
        }
        let mut chars = ss.chars();
        let rank = chars.next().unwrap();
        let suit = chars.next().unwrap();
        let real_rank = match rank {
            'A' => Rank::Ace,
            'K' => Rank::King,
            'Q' => Rank::Queen,
            'J' => Rank::Jack,
            'T' => Rank::Ten,
            '9' => Rank::Nine,
            '8' => Rank::Eight,
            '7' => Rank::Seven,
            '6' => Rank::Six,
            '5' => Rank::Five,
            '4' => Rank::Four,
            '3' => Rank::Three,
            '2' => Rank::Two,
            symb @ _ => panic!("error: wrong rank: {}", symb),
        };
        let real_suit = match suit {
            's' => Suit::Spades,
            'c' => Suit::Clubs,
            'h' => Suit::Harts,
            'd' => Suit::Daemonds,
            symb @ _ => panic!("error: wrong suit: {}", symb),
        };
        Card {
            rank: real_rank,
            suit: real_suit,
        }
    }
}
impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.rank, self.suit)
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.rank, self.suit)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Hand {
    pub cards: [Card; 4], // always sort from top to low!!!
}
impl Hand {
    pub fn rnd_hand(dead_cards: &Vec<Card>) -> Self {
        let mut set = HashSet::with_capacity(4);
        while set.len() < 4 {
            let card = Card::rnd_card();
            if dead_cards.contains(&card) {
                continue;
            }
            set.insert(card);
        }
        let mut cards = set.into_iter().collect::<Vec<Card>>();
        Hand::new(
            cards.pop().unwrap_or_else(|| unreachable!()),
            cards.pop().unwrap_or_else(|| unreachable!()),
            cards.pop().unwrap_or_else(|| unreachable!()),
            cards.pop().unwrap_or_else(|| unreachable!()),
        )
        .unwrap_or_else(|_| unreachable!())
    }
    pub fn new(card_1: Card, card_2: Card, card_3: Card, card_4: Card) -> Result<Self, String> {
        let mut cards = [card_1, card_2, card_3, card_4];
        let mut uniq_set = HashSet::with_capacity(4);
        let is_uniq_cards = cards.iter().all(|e| uniq_set.insert(e));
        if is_uniq_cards {
            cards.sort_unstable_by(|a, b| b.cmp(a));
            Ok(Self { cards })
        } else {
            Err(String::from(
                "Error: Can't create preflop hand with non-uniq cards!",
            ))
        }
    }
    pub fn has_fd_to_rank(&self, rank: Rank) -> bool {
        let aces = self
            .cards
            .iter()
            .filter(|&x| x.rank == rank)
            .collect::<Vec<_>>();

        for &i in aces.iter() {
            let count = self
                .cards
                .iter()
                .fold(0u8, |acc, e| if i.suit == e.suit { acc + 1 } else { acc });

            if count > 1 {
                return true;
            }
        }
        false
    }
    pub fn is_double_suited(&self) -> bool {
        let mut set = HashMap::new();
        for i in self.cards.iter() {
            let elm = set.entry(&i.suit).or_insert(0u8);
            *elm += 1;
        }
        set.iter().all(|(_, &v)| v == 2)
    }
    pub fn is_off_suited(&self) -> bool {
        let mut set = HashSet::new();
        self.cards.iter().all(|e| set.insert(&e.suit))
    }
    pub fn pairing_status(&self) -> Pairing {
        let mut map = HashMap::new();
        self.cards.iter().for_each(|card| {
            let elm = map.entry(&card.rank).or_insert(0u8);
            *elm += 1;
        });
        let max = map
            .iter()
            .max_by(|&a, &b| a.1.cmp(b.1))
            .unwrap_or_else(|| unreachable!());
        let min = map
            .iter()
            .min_by(|&a, &b| a.1.cmp(b.1))
            .unwrap_or_else(|| unreachable!());

        match (max.1, min.1) {
            (1, _) => Pairing::NoPaired,
            (2, 1) => Pairing::Paired,
            (2, 2) => Pairing::DoublePaired,
            _ => Pairing::TripsCare,
        }
    }
}
impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?} {:?}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3]
        )
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum Pairing {
    TripsCare,
    Paired,
    DoublePaired,
    NoPaired,
}
impl Debug for Pairing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "({})",
            match self {
                Pairing::NoPaired => "np",
                Pairing::Paired => "p",
                Pairing::DoublePaired => "dp",
                Pairing::TripsCare => "-",
            }
        )
    }
}
