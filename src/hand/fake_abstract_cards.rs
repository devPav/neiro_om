use super::real_abstract_cards::{Hand, Pairing, Rank};
use std::fmt::Debug;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum FakeRank {
    GarbageCard,     // 2-4
    SemiGarbageCard, // 5-6
    LowCard,         // 7-8
    MiddleCard,      // 9-T
    BigCard,         // J,Q
    King,
    Ace,
}
impl Debug for FakeRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let present = match self {
            FakeRank::GarbageCard => "X",
            FakeRank::SemiGarbageCard => "Z",
            FakeRank::LowCard => "L",
            FakeRank::MiddleCard => "M",
            FakeRank::BigCard => "B",
            FakeRank::King => "K",
            FakeRank::Ace => "A",
        };
        write!(f, "{}", present)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct FakeCard {
    pub rank: FakeRank,
}
impl Debug for FakeCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.rank)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum FakeSuitKind {
    Os,
    Ss,
    Kss,
    Ass,
    Ds,
}
impl Debug for FakeSuitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            match self {
                FakeSuitKind::Os => "os",
                FakeSuitKind::Ss => "ss",
                FakeSuitKind::Kss => "Ks",
                FakeSuitKind::Ass => "As",
                FakeSuitKind::Ds => "ds",
            }
        )
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct FakeHand {
    pub cards: [FakeCard; 4],
    pub kind: FakeSuitKind,
    pub paired: Pairing,
}
impl FakeHand {
    pub fn from(hand: &Hand) -> Self {
        let mut fake_cards: [FakeCard; 4] = hand
            .cards
            .iter()
            .cloned()
            .map(|e| FakeCard {
                rank: match e.rank {
                    Rank::Ace => FakeRank::Ace,
                    Rank::King => FakeRank::King,
                    Rank::Queen | Rank::Jack => FakeRank::BigCard,
                    Rank::Ten | Rank::Nine => FakeRank::MiddleCard,
                    Rank::Eight | Rank::Seven => FakeRank::LowCard,
                    Rank::Six | Rank::Five => FakeRank::SemiGarbageCard,
                    Rank::Four | Rank::Three | Rank::Two => FakeRank::GarbageCard,
                },
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!());
        fake_cards.sort_unstable_by(|a, b| b.cmp(a));

        let kind = if hand.is_off_suited() {
            FakeSuitKind::Os
        } else if hand.is_double_suited() {
            FakeSuitKind::Ds
        } else if hand.has_fd_to_rank(Rank::Ace) {
            FakeSuitKind::Ass
        } else if hand.has_fd_to_rank(Rank::King) {
            FakeSuitKind::Kss
        } else {
            FakeSuitKind::Ss
        };
        Self {
            cards: fake_cards,
            kind,
            paired: hand.pairing_status(),
        }
    }
}
impl Debug for FakeHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}{:?}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.kind, self.paired
        )
    }
}
