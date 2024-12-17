#[cfg(test)]
pub mod imba_full_house {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_ready;
    use crate::{eval_hand::*, Card, FakePostReadyHand, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn full_J9332_JJT7_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Jack,
                pair: Rank::Three
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_QJ332_JJT7_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Jack,
                pair: Rank::Three
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_A7722_AA72_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Two, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Seven, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Ace,
                pair: Rank::Seven
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_A7722_7332_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Three, Suit::Harts),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Seven, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Seven,
                pair: Rank::Two
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_77442_AK74_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Four, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Seven, Suit::Daemonds),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Seven,
                pair: Rank::Four
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_JJ982_J993_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Nine, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Jack,
                pair: Rank::Nine
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_JJ982_J883_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Jack,
                pair: Rank::Eight
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_AAA52_KK93_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Ace,
                pair: Rank::King
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_A7772_AA93_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Seven, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Ace,
                pair: Rank::Seven
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn flop_J33_JJT7_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        ];

        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Jack,
                pair: Rank::Three
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn flop_QQJ_JJT7_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Jack, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Jack,
                pair: Rank::Queen
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn flop_AAA_KK93_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Ace,
                pair: Rank::King
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_K2222_KK93_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Two, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::King,
                pair: Rank::Two
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn full_77772_AA93_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Seven, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Seven,
                pair: Rank::Ace
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn turn_2222_AA93_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Two, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Two,
                pair: Rank::Ace
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn turn_AAAA_KK93_imba() {
        let ready_hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Ace,
                pair: Rank::King
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn turn_AAA2_KK93_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Ace,
                pair: Rank::King
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_ne!(fake_comb, FakePostReadyHand::Imba);
    }
}
#[cfg(test)]
pub mod nut_no_nut_street {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_ready;
    use crate::{eval_hand::*, Card, FakePostReadyHand, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn street_T98_7655_nonut() {
        let hand = Hand::new(
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ten), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NoNutStreet);
    }
    #[test]
    fn street_T87_9655_nonut() {
        let hand = Hand::new(
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ten), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NoNutStreet);
    }
    #[test]
    fn street_T76_9855_nut() {
        let hand = Hand::new(
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ten), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutStreet);
    }
    #[test]
    fn street_KQT76_9852_nonut() {
        let hand = Hand::new(
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ten), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NoNutStreet);
    }
    #[test]
    fn street_T9877_J752_nonut() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Jack), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NoNutStreet);
    }
    #[test]
    fn street_T9877_QJ52_nut() {
        let hand = Hand::new(
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Queen), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutStreet);
    }
    #[test]
    fn street_TT776_9852_nut() {
        let hand = Hand::new(
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ten), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutStreet);
    }
    #[test]
    fn street_AKQ98_JT52_nut() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ace), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutStreet);
    }
    #[test]
    fn street_AJT8_KQ52_nut() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ace), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutStreet);
    }
    #[test]
    fn street_AKQ54_5432_nonut() {
        let hand = Hand::new(
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Harts),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Five), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NoNutStreet);
    }
    #[test]
    fn street_A9843_5432_nut() {
        let hand = Hand::new(
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Harts),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Five), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutStreet);
    }
}
#[cfg(test)]
pub mod set_trips {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_ready;
    use crate::{eval_hand::*, Card, FakePostReadyHand, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn set_on_nonpaired() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Five, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Five,
                top_kicker: Rank::Ten,
                low_kicker: Rank::Nine
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Set);
    }
    #[test]
    fn noset_on_nonpaired() {
        let hand = Hand::new(
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Five, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ten), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NoNutStreet);
    }
    #[test]
    fn noset_on_paired() {
        let hand = Hand::new(
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Five, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::FullHouse {
                trips: Rank::Five,
                pair: Rank::Nine
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::LowFullHouse);
    }
    #[test]
    fn nut_trips() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Six, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Six,
                top_kicker: Rank::Ace,
                low_kicker: Rank::Ten
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TripsNutKicker);
    }
    #[test]
    fn nut_trips_advanced_cases1() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Ace,
                top_kicker: Rank::King,
                low_kicker: Rank::Seven
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TripsNutKicker);
    }
    #[test]
    fn nut_trips_advanced_cases2() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Ace,
                top_kicker: Rank::King,
                low_kicker: Rank::Queen,
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TripsLess);
    }
    #[test]
    fn nonut_trips() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Six, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Six,
                top_kicker: Rank::King,
                low_kicker: Rank::Ten
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TripsLess);
    }
    #[test]
    fn nonut_trips_advanced_cases1() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Ace,
                top_kicker: Rank::King,
                low_kicker: Rank::Queen,
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TripsLess);
    }
    #[test]
    fn nonut_trips_advanced_cases2() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Seven,
                top_kicker: Rank::Ace,
                low_kicker: Rank::King,
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TripsLess);
    }
    #[test]
    fn b_TTT_h_AK32() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Harts),
            Card::new(Rank::Ten, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Ten,
                top_kicker: Rank::Ace,
                low_kicker: Rank::King
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Nothing);
    }
    #[test]
    fn turn_2222_AK93_noimba() {
        let ready_hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Two, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let ready_comb = real_comb(&ready_hand, &board);
        assert_eq!(
            ReadyHand::Trips {
                trips: Rank::Two,
                top_kicker: Rank::Ace,
                low_kicker: Rank::King
            },
            ready_comb
        );
        let fake_comb = fake_comb_side_ready(&ready_hand, ready_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Nothing);
    }
}
#[cfg(test)]
pub mod two_pairs {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_ready;
    use crate::{eval_hand::*, Card, FakePostReadyHand, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn toptwo_AAK92_K877() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::King,
                kicker: Rank::Eight
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn no_toptwo_AAK92_9877() {
        let hand = Hand::new(
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::Nine,
                kicker: Rank::Eight
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::BottomBottom);
    }
    #[test]
    fn toptwo_AAQQ9_KK77() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::King,
                kicker: Rank::Queen
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn no_toptwo_AAQQ9_JJ77() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::Jack,
                kicker: Rank::Queen
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::BottomBottom);
    }
    #[test]
    fn toptwo_KKQ92_AA77() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::King,
                kicker: Rank::Queen
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn no_toptwo_KKQ92_Q877() {
        let hand = Hand::new(
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::King,
                bottom: Rank::Queen,
                kicker: Rank::Eight
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::BottomBottom);
    }
    #[test]
    fn toptwo_AK992_AK77() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::King,
                kicker: Rank::Nine
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn no_toptwo_AK992_KQ77() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::King,
                bottom: Rank::Nine,
                kicker: Rank::Queen
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::BottomBottom);
    }
    #[test]
    fn toptwo_55443_AA77() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::Five,
                kicker: Rank::Four
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn no_toptwo_55443_KK7() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::King,
                bottom: Rank::Five,
                kicker: Rank::Four
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::BottomBottom);
    }
    #[test]
    fn toptwo_322_AA77() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::Two,
                kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn toptwo_K5544_AA77() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Four, Suit::Harts),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::Five,
                kicker: Rank::King
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn toptwo_3322_AA77() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Harts),
            Card::new(Rank::Ace, Suit::Daemonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Ace,
                bottom: Rank::Three,
                kicker: Rank::Two
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn toptwo_K8532_K876() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::King,
                bottom: Rank::Eight,
                kicker: Rank::Five
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopTwo);
    }
    #[test]
    fn topbottom_K8532_K765() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::King,
                bottom: Rank::Five,
                kicker: Rank::Eight
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TopBottom);
    }
    #[test]
    fn bottombottom_K8532_8765() {
        let hand = Hand::new(
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Harts),
            Card::new(Rank::Five, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::Eight,
                bottom: Rank::Five,
                kicker: Rank::King
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::BottomBottom);
    }
}
#[cfg(test)]
pub mod top_pair {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_ready;
    use crate::{eval_hand::*, Card, FakePostReadyHand, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn toppair_KJ532_KT84() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Four, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::King,
                top_kicker: Rank::Jack,
                mid_kicker: Rank::Ten,
                low_kicker: Rank::Five
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TPOP);
    }
    #[test]
    fn toppair_KJ532_AA83() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Three, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ace,
                top_kicker: Rank::King,
                mid_kicker: Rank::Jack,
                low_kicker: Rank::Five
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::TPOP);
    }
    #[test]
    fn no_toppair_KJ532_JT84() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Four, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Jack,
                top_kicker: Rank::King,
                mid_kicker: Rank::Ten,
                low_kicker: Rank::Five
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Nothing);
    }
    #[test]
    fn no_toppair_KJ522_KT84() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Harts),
            Card::new(Rank::Four, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::TwoPair {
                top: Rank::King,
                bottom: Rank::Two,
                kicker: Rank::Ten
            },
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::BottomBottom);
    }
}
#[cfg(test)]
pub mod flash_draw {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_fd;
    use crate::{eval_hand::*, Card, FakePostflopFD, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn b_AsKsJd2d_h_Qs9s4d3d() {
        let hand = Hand::new(
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::HightCards(Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine),
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::TwoFdWithNut);
    }
    #[test]
    fn b_AsTs3c2c_h_KsJcTc4s() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ten,
                top_kicker: Rank::Ace,
                mid_kicker: Rank::King,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::TwoFdWithNut);
    }
    #[test]
    fn b_AsTs3c2c_h_QsJcTc4s() {
        let hand = Hand::new(
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ten,
                top_kicker: Rank::Ace,
                mid_kicker: Rank::Queen,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::TwoFD);
    }
    #[test]
    fn b_QsTs3c2c_h_KsKcTc4s() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::King,
                top_kicker: Rank::Queen,
                mid_kicker: Rank::Ten,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::TwoFD);
    }
    #[test]
    fn b_AsTs3c2c_h_KsJdTc4s() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ten,
                top_kicker: Rank::Ace,
                mid_kicker: Rank::King,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::OneNutFD);
    }
    #[test]
    fn b_QsTs3c_h_AsJdTc4s() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ten,
                top_kicker: Rank::Ace,
                mid_kicker: Rank::Queen,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::OneNutFD);
    }
    #[test]
    fn b_AsTs3c2c_h_QdTd9s4s() {
        let hand = Hand::new(
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ten,
                top_kicker: Rank::Ace,
                mid_kicker: Rank::Queen,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::OneSecondThree);
    }
    #[test]
    fn b_QsTs3c_h_KsJdTc4s() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ten,
                top_kicker: Rank::King,
                mid_kicker: Rank::Queen,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::OneSecondThree);
    }
    #[test]
    fn b_9s8s3c2c_h_TsTdTc4s() {
        let hand = Hand::new(
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Ten, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Ten,
                top_kicker: Rank::Nine,
                mid_kicker: Rank::Eight,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::Low);
    }
    #[test]
    fn b_Qs9c3c_h_KsKdTc4c() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::King,
                top_kicker: Rank::Queen,
                mid_kicker: Rank::Nine,
                low_kicker: Rank::Three
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::Low);
    }
    #[test]
    fn b_7s6s5s_h_KsJdTc4d() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Four, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::HightCards(Rank::King, Rank::Jack, Rank::Seven, Rank::Six, Rank::Five),
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::Nothing);
    }
    #[test]
    fn b_7s6c5d4h_h_KsJdTs4d() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Five, Suit::Daemonds),
            Card::new(Rank::Four, Suit::Harts),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::OnePair {
                pair: Rank::Four,
                top_kicker: Rank::King,
                mid_kicker: Rank::Seven,
                low_kicker: Rank::Six
            },
            real_comb
        );
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::Nothing);
    }
    #[test]
    fn b_AsKcQd_h_Js5s4d3d_BD() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::TwoBD);
    }
    #[test]
    fn b_AsKcQd_h_Js5s4s3d_BD() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::Nothing);
    }
    #[test]
    fn b_AsKcQd_h_Js5h4h3d_BD() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Five, Suit::Harts),
            Card::new(Rank::Four, Suit::Harts),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::Nothing);
    }
    #[test]
    fn b_AsKcQdJh_h_Js5s4d3d_BD() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Daemonds),
            Card::new(Rank::Jack, Suit::Harts),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_fd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopFD::Nothing);
    }
}
#[cfg(test)]
pub mod street_draw {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_sd;
    use crate::{eval_hand::*, Card, FakePostflopSD, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn b_QsTs6s5s_h_AcKcJc2c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NutWrap);
    }
    #[test]
    fn b_QsTs6s5s_h_8c7c4c2c() {
        let hand = Hand::new(
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NoNutWrap);
    }
    #[test]
    fn b_QsTs6s5s_h_9c8c7c2c() {
        let hand = Hand::new(
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NutWrap);
    }
    #[test]
    fn b_QsTs6s5s_h_KcJc9c2c() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NoNutWrap);
    }
    #[test]
    fn b_QsTs6s4s_h_AcKc5c3c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Oesd);
    }
    #[test]
    fn b_QsTs6s4s_h_AcKc5c2c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Nothing);
    }
    #[test]
    fn b_QsTs6s4s_h_Ac7c5c3c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NoNutWrap);
    }
    #[test]
    fn b_QsTs6s4s_h_Ac8c7c5c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NutWrap);
    }
    #[test]
    fn b_AsTs9s2s_h_Jc5c4c3c() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NutWrap);
    }
    #[test]
    fn b_AsTs9s2s_h_Jc9c4c3c() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Nothing);
    }
    #[test]
    fn b_Ts9s3s2s_h_Ac5c4c3c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NoNutWrap);
    }
    #[test]
    fn b_AsKs3s2s_h_QcJcTc2c() {
        let hand = Hand::new(
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NutWrap);
    }
    #[test]
    fn b_KsKs3s2s_h_Ac5c4c2c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NoNutWrap);
    }
    #[test]
    fn b_KsKc3s2s_h_Kh5c4c4h() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Four, Suit::Harts),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Oesd);
    }
    #[test]
    fn b_AsKs3s2s_h_QcJc9c2c() {
        let hand = Hand::new(
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Nothing);
    }
    #[test]
    fn b_Js8s6s_h_AcKcTc9c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Oesd);
    }
    #[test]
    fn b_8s6s3s_h_AcKc5c4c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Oesd);
    }
    #[test]
    fn b_Js8s6s3s_h_AcTc9c7c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::NutWrap);
    }
    #[test]
    fn b_Kc8c4d_h_JhTs9c7d_debug() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Harts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Four, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Nothing);
    }
    #[test]
    fn b_Js8s6s3s2s_h_AcTc9c7c() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        let fake_comb = fake_comb_side_sd(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostflopSD::Nothing);
    }
}
#[cfg(test)]
pub mod flash {
    use crate::postflop_game::eval_fake_hand::fake_comb_side_ready;
    use crate::{eval_hand::*, Card, FakePostReadyHand, Hand, Rank, ReadyHand, Suit};
    #[test]
    fn b_AsKsQs_h_Js5s4d3d() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Five),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutFlash);
    }
    #[test]
    fn b_AsKsQs_h_Ts5s4d3d() {
        let hand = Hand::new(
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::Ace, Rank::King, Rank::Queen, Rank::Ten, Rank::Five),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::SecondThreeFlash);
    }
    #[test]
    fn b_AsKsQs_h_8s5s4d3d() {
        let hand = Hand::new(
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::Ace, Rank::King, Rank::Queen, Rank::Eight, Rank::Five),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::SecondThreeFlash);
    }
    #[test]
    fn b_AsKsQs_h_7s5s4d3d() {
        let hand = Hand::new(
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::Ace, Rank::King, Rank::Queen, Rank::Seven, Rank::Five),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::LowFlash);
    }
    #[test]
    fn b_AsKsQs_h_JsTh4d3d() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Harts),
            Card::new(Rank::Four, Suit::Daemonds),
            Card::new(Rank::Three, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::Street(Rank::Ace), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutStreet);
    }
    #[test]
    fn b_7s6s5s3d2d_h_AsKdKh3s() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Three, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::Ace, Rank::Seven, Rank::Six, Rank::Five, Rank::Three),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::NutFlash);
    }
    #[test]
    fn b_7s6s5s3d2d_h_KsKdKh3s() {
        let hand = Hand::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Daemonds),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Three, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::King, Rank::Seven, Rank::Six, Rank::Five, Rank::Three),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::SecondThreeFlash);
    }
    #[test]
    fn b_7s6s5s3d2d_h_JsJd9s2s() {
        let hand = Hand::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Daemonds),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::Jack, Rank::Nine, Rank::Seven, Rank::Six, Rank::Five),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::SecondThreeFlash);
    }
    #[test]
    fn b_7s6s5s3d2d_h_Ts9s8s4s() {
        let hand = Hand::new(
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Three, Suit::Daemonds),
            Card::new(Rank::Two, Suit::Daemonds),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(ReadyHand::StreetFlash(Rank::Nine), real_comb);
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::Imba);
    }
    #[test]
    fn b_Js4s3s2s_h_Ts9s8s2d() {
        let hand = Hand::new(
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Two, Suit::Daemonds),
        )
        .unwrap();
        // board always sorted
        let board = vec![
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let real_comb = real_comb(&hand, &board);
        assert_eq!(
            ReadyHand::Flash(Rank::Jack, Rank::Ten, Rank::Nine, Rank::Four, Rank::Three),
            real_comb
        );
        let fake_comb = fake_comb_side_ready(&hand, real_comb, &board);
        assert_eq!(fake_comb, FakePostReadyHand::SecondThreeFlash);
    }
}
