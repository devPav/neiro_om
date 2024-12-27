#![allow(non_snake_case)]
#[cfg(test)]
pub mod street_blockers {
    use std::collections::HashSet;

    use crate::{Card, PostflopGame, PreflopGame, Rank, Suit};
    #[test]
    fn AKQJT() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
        ];
        let answer = vec![Rank::Jack, Rank::Ten];

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_none());
    }
    #[test]
    fn A5432() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Daemonds),
        ];
        let answer = vec![Rank::Seven, Rank::Six];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AKQJ9() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Nine, Suit::Daemonds),
        ];
        let answer = vec![Rank::Jack, Rank::Ten];

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_none());
    }
    #[test]
    fn A5433() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
        ];
        let answer = vec![Rank::Seven, Rank::Six];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn A5533() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
        ];
        let answer = vec![Rank::Four, Rank::Two];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AKQ98() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
        ];
        let answer = vec![Rank::Jack, Rank::Ten];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AQJ98() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
        ];
        let answer = vec![Rank::King, Rank::Ten];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AJT98() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
        ];
        let answer = vec![Rank::King, Rank::Queen];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AQT98() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
        ];
        let answer = vec![Rank::King, Rank::Jack];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AK853() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
        ];
        let answer = vec![Rank::Four, Rank::Two];
        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AK852() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Two, Suit::Daemonds),
        ];
        let answer = vec![Rank::Four, Rank::Three];
        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AK854() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
        ];
        let answer = vec![Rank::Six, Rank::Seven];
        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn QJT98() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
        ];
        let answer = vec![Rank::Ace, Rank::King];
        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AQ944() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
        ];

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_none());
    }
    #[test]
    fn AQQQ9() {
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Nine, Suit::Daemonds),
        ];

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_none());
    }
    #[test]
    fn AQQQJ() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Daemonds),
        ];
        let answer = vec![Rank::King, Rank::Ten];
        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AQ963() {
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
        ];

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_none());
    }
    #[test]
    fn AJ975() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Five, Suit::Daemonds),
        ];
        let answer = vec![Rank::Eight, Rank::Ten];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());
        dbg!(&street_blockers);

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
    #[test]
    fn AJ654() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Six, Suit::Daemonds),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
        ];
        let answer = vec![Rank::Eight, Rank::Seven];

        let answer_set = answer.into_iter().collect::<HashSet<_>>();

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let street_blockers = river_game.street_blockers_to_board();
        assert!(street_blockers.is_some());
        dbg!(&street_blockers);

        let street_blockers = street_blockers.unwrap();
        let street_blockers_set = street_blockers.into_iter().collect::<HashSet<_>>();

        let diff1 = answer_set
            .difference(&street_blockers_set)
            .collect::<HashSet<_>>();
        let diff2 = street_blockers_set
            .difference(&answer_set)
            .collect::<HashSet<_>>();

        assert!(diff1.is_empty());
        assert!(diff2.is_empty());
    }
}

pub mod flash_blockers {
    use crate::{Card, PostflopGame, PreflopGame, Rank, Suit};

    #[test]
    fn AsKsQdJsTd() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
        ];
        let answer = Card::new(Rank::Queen, Suit::Spades);

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let flash_blocker = river_game.flash_blockers_to_board();
        assert!(flash_blocker.is_some());

        let flash_blocker = flash_blocker.unwrap();
        assert_eq!(flash_blocker, answer);
    }
    #[test]
    fn AsKdQs9s8d() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Daemonds),
        ];
        let answer = Card::new(Rank::King, Suit::Spades);

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let flash_blocker = river_game.flash_blockers_to_board();
        assert!(flash_blocker.is_some());

        let flash_blocker = flash_blocker.unwrap();
        assert_eq!(flash_blocker, answer);
    }
    #[test]
    fn KsKdJd9s8s() {
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
        ];
        let answer = Card::new(Rank::Ace, Suit::Spades);

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let flash_blocker = river_game.flash_blockers_to_board();
        assert!(flash_blocker.is_some());

        let flash_blocker = flash_blocker.unwrap();
        assert_eq!(flash_blocker, answer);
    }
    #[test]
    fn AsJsTd9d8c() {
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let answer = None;

        let preflop_game = PreflopGame::new();
        let flop_game = PostflopGame::from(&preflop_game);
        let turn_game = PostflopGame::from(&flop_game);
        let mut river_game = PostflopGame::from(&turn_game);
        river_game.cards = board;

        let flash_blocker = river_game.flash_blockers_to_board();
        assert_eq!(flash_blocker, answer);
    }
}
