use crate::{Position, ReadyHand};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::{BTreeMap, HashMap};

/* Логика:
Данные:
1. Вектор для суммаризации вложений в пот на всех улицах.
2. Мапа для хранения соответствия позиция - готовая рука. (все кроме сфолженных позиций)
3. Отсортированная мапа
    - Во-первых автоматическая сортировка за счет BTreeMap, от слабой комбы к сильной.
    - Во-вторых каждая строчка мапы содержит вектор позиций, которая содержит эту комбинацию.
        !Вектор обязательно отсортирован в порядке роста внесенных денег игрока (от малого к большему).
    - В-третьих в ней нет позиций, которые в фолде, но есть те, которые в алине.
Результат:
    Мапа содержит только те позиции, которые борются за пот, все кроме сфолдивших к риверу.
 */
pub fn eval_clear_win_loose(
    all_positions_and_money: Vec<HashMap<Position, Decimal>>,
    real_hands_end: &HashMap<Position, ReadyHand>,
    extra_money: Option<Decimal>,
) -> HashMap<Position, Decimal> {
    let take_back_from_pot =
        eval_take_back_from_pot(&all_positions_and_money, real_hands_end, extra_money);
    // let take_back_from_pot = decrease_rake(take_back_from_pot, &all_positions_and_money);
    let summarize_position_money = summarize_position_money(&all_positions_and_money);
    let mut win_loose = HashMap::new();
    summarize_position_money.iter().for_each(|(&pos, &money)| {
        let win_money = *take_back_from_pot
            .get(&pos)
            .unwrap_or_else(|| &Decimal::ZERO);
        win_loose.insert(pos, win_money - money);
    });
    win_loose
}
pub fn eval_clear_win_loose_five_fold(
    all_positions_and_money: Vec<HashMap<Position, Decimal>>,
    extra_money: Option<Decimal>,
) -> HashMap<Position, Decimal> {
    /* Логика:
    - Если все сфолдили на какой-то улице, то ВСЕ ЧУЖИЕ деньги вложенные в банк выиграл один игрок.
    - Определить этого игрока просто, это тот, кто внес в игру наибольшие деньги, только так можно заставить
    всех сфолдить.
    */
    let summarize_position_money = summarize_position_money(&all_positions_and_money);
    let winner_possition = summarize_position_money
        .iter()
        .max_by_key(|&entry| entry.1)
        .map(|(&pos, _)| pos)
        .unwrap_or_else(|| unreachable!());
    let mut winner_amount = summarize_position_money.values().sum();
    if let Some(e_m) = extra_money {
        winner_amount += e_m;
    }
    let take_back_from_pot = HashMap::from([(winner_possition, winner_amount)]);
    // let take_back_from_pot = decrease_rake(take_back_from_pot, &all_positions_and_money);
    let mut win_loose = HashMap::new();
    summarize_position_money.iter().for_each(|(&pos, &money)| {
        let win_money = *take_back_from_pot
            .get(&pos)
            .unwrap_or_else(|| &Decimal::ZERO);
        win_loose.insert(pos, win_money - money);
    });
    win_loose
}
fn eval_take_back_from_pot(
    all_positions_and_money: &Vec<HashMap<Position, Decimal>>,
    real_hands_end: &HashMap<Position, ReadyHand>,
    extra_money: Option<Decimal>,
) -> HashMap<Position, Decimal> {
    let mut take_back_map = HashMap::new();
    // Get summarize "position and money" for all streets.
    let mut summ_pos_money = summarize_position_money(all_positions_and_money);
    // Get double sorted spacial hash map.
    let double_sorted_map = winners_map(&summ_pos_money, real_hands_end);
    // println!("{:?}", double_sorted_map);
    // Get result.
    double_sorted_map
        .iter()
        .rev()
        .for_each(|(_, poses_sorted)| {
            let mut counter_side_potes = Decimal::from(poses_sorted.len());
            let mut add_money = dec!(0);
            poses_sorted.iter().for_each(|&win_pose| {
                let win_wait = *summ_pos_money.get(&win_pose).unwrap();
                let mut win_money = dec!(0);
                summ_pos_money.iter_mut().for_each(|(_, money)| {
                    let money_decrease = if *money > win_wait { win_wait } else { *money };
                    *money -= money_decrease;
                    win_money += money_decrease;
                });
                let val = take_back_map.entry(win_pose).or_insert(dec!(0));
                *val += win_money / counter_side_potes + add_money;
                summ_pos_money.remove(&win_pose);
                add_money += win_money / counter_side_potes;
                counter_side_potes -= dec!(1);
            })
        });
    // Add extra money only to the best winning hands.
    if extra_money.is_some() {
        let poses_bests = double_sorted_map.last_key_value().unwrap().1;
        let count_bests = poses_bests.len();
        let average_extra_summ = (extra_money.unwrap() / Decimal::from(count_bests)).round_dp(2);
        for (k, mut v) in take_back_map.iter_mut() {
            if poses_bests.contains(&k) {
                v += average_extra_summ;
            }
        }
    }

    take_back_map
}
fn summarize_position_money(
    all_positions_and_money: &Vec<HashMap<Position, Decimal>>,
) -> HashMap<Position, Decimal> {
    // Get summarize "position and money" for all streets.
    let mut summ_pos_money = HashMap::new();
    all_positions_and_money
        .iter()
        .for_each(|positions_and_money| {
            positions_and_money.iter().for_each(|(&k, &v)| {
                let new_val = summ_pos_money.entry(k).or_insert(dec!(0));
                *new_val += v;
            })
        });
    summ_pos_money
}
fn winners_map(
    summ_pos_money: &HashMap<Position, Decimal>,
    real_hands_end: &HashMap<Position, ReadyHand>,
) -> BTreeMap<ReadyHand, Vec<Position>> {
    // Get double sorted spacial hash map.
    let mut double_sorted_map = BTreeMap::new();
    real_hands_end.iter().for_each(|(&pos, &ready_comb)| {
        let val = double_sorted_map.entry(ready_comb).or_insert(vec![]);
        val.push(pos)
    });
    double_sorted_map.iter_mut().for_each(|(_, v_poses)| {
        v_poses.sort_by(|&a, &b| {
            let a_money = summ_pos_money.get(&a).unwrap();
            let b_money = summ_pos_money.get(&b).unwrap();
            a_money.cmp(b_money)
        })
    });
    double_sorted_map
}
#[allow(dead_code)]
fn decrease_rake(
    mut take_back_from_pot: HashMap<Position, Decimal>,
    all_positions_and_money: &Vec<HashMap<Position, Decimal>>,
) -> HashMap<Position, Decimal> {
    /* Логика:
    !!! Идиальная, кроме очень редкого случая, когда ктото поставил рейз не последним ходом сверх всех аллинов,
    тогда рейк будет слегка занижен, но несущественно

    - Рейк для plo50, plo25 rush на ГГ (на обычных столах рейк чуть меньше по рассчетам):
        - Если банк больше 30 ББ, то 2ББ рейка с банка на кэшдроп и на джекпот
        - No flop no drop in rush (only when see flop) или был трибет на префлопе.
            Короче говоря все это похоже на границу пота, хотя нет
    Трибет на префлопе:
        - Макс пот при опен рейз, чтобы не было флопа 5ББ - 6ББ
        - Мин пот при минимальном трибете 6.5ББ (очень редкий случай 5ББ флет рейз СБ и флет 3-бет на ББ)
    */
    let mut rake_amount = dec!(0);
    let pot = take_back_from_pot.values().sum::<Decimal>();
    let rake_cap = dec!(3);

    if all_positions_and_money.len() > 1 || pot > dec!(6.5) {
        if pot * dec!(0.05) < rake_cap {
            rake_amount += (pot * dec!(0.05)).round_dp(2);
        } else {
            rake_amount += rake_cap
        };
    };

    let take_spec_rake = if pot > dec!(30) { true } else { false };
    if take_spec_rake {
        rake_amount += dec!(2);
    }

    //dbg!(&rake_amount);
    let rake_rate_real = (rake_amount / pot).round_dp(3);
    //dbg!(&rake_rate_real);

    // dbg!(&take_back_from_pot);
    for (_, val) in take_back_from_pot.iter_mut() {
        let rake = (*val * rake_rate_real).round_dp(2);
        // dbg!(&rake);
        // dbg!(&val);
        *val -= rake;
        // dbg!(&val);
    }
    // dbg!(&take);
    take_back_from_pot
}
#[cfg(test)]
mod eval_result {
    use super::*;
    use crate::Rank;
    #[test]
    #[ignore = "Without rake"]
    fn hard_one() {
        let all_positions_and_money = vec![HashMap::from([
            (Position::Sb, dec!(30)),
            (Position::Bb, dec!(20)),
            (Position::Utg, dec!(10)),
            (Position::Mp, dec!(40)),
            (Position::Co, dec!(50)),
            (Position::Btn, dec!(60)),
        ])];
        let real_hands_end = HashMap::from([
            (Position::Sb, ReadyHand::FlashRoal),
            (Position::Bb, ReadyHand::FlashRoal),
            (Position::Utg, ReadyHand::FlashRoal),
            (
                Position::Mp,
                ReadyHand::Trips {
                    trips: Rank::Three,
                    top_kicker: Rank::Ace,
                    low_kicker: Rank::Five,
                },
            ),
            (
                Position::Co,
                ReadyHand::Trips {
                    trips: Rank::Three,
                    top_kicker: Rank::Ace,
                    low_kicker: Rank::Five,
                },
            ),
            (
                Position::Btn,
                ReadyHand::TwoPair {
                    top: Rank::Seven,
                    bottom: Rank::Two,
                    kicker: Rank::Ace,
                },
            ),
        ]);
        let real_wins = eval_clear_win_loose(all_positions_and_money, &real_hands_end, None);
        let suppose_wins_w_rake = HashMap::from([
            (Position::Sb, dec!(55)),
            (Position::Bb, dec!(25)),
            (Position::Utg, dec!(10)),
            (Position::Mp, dec!(-25)),
            (Position::Co, dec!(-15)),
            (Position::Btn, dec!(-50)),
        ]);
        assert_eq!(real_wins, suppose_wins_w_rake);
    }
    #[test]
    #[ignore = "Without rake"]
    fn normal_one() {
        let all_positions_and_money = vec![HashMap::from([
            (Position::Sb, dec!(10)),
            (Position::Bb, dec!(20)),
            (Position::Utg, dec!(30)),
            (Position::Mp, dec!(100)),
            (Position::Co, dec!(100)),
            (Position::Btn, dec!(100)),
        ])];
        let real_hands_end = HashMap::from([
            (Position::Sb, ReadyHand::FlashRoal),
            (Position::Bb, ReadyHand::FlashRoal),
            (Position::Utg, ReadyHand::FlashRoal),
            (
                Position::Mp,
                ReadyHand::Trips {
                    trips: Rank::Three,
                    top_kicker: Rank::Ace,
                    low_kicker: Rank::Five,
                },
            ),
            (
                Position::Co,
                ReadyHand::Trips {
                    trips: Rank::Two,
                    top_kicker: Rank::Ace,
                    low_kicker: Rank::Five,
                },
            ),
            (
                Position::Btn,
                ReadyHand::TwoPair {
                    top: Rank::Seven,
                    bottom: Rank::Two,
                    kicker: Rank::Ace,
                },
            ),
        ]);
        let real_wins = eval_clear_win_loose(all_positions_and_money, &real_hands_end, None);
        let suppose_wins = HashMap::from([
            (Position::Sb, dec!(10)),
            (Position::Bb, dec!(25)),
            (Position::Utg, dec!(55)),
            (Position::Mp, dec!(110)),
            (Position::Co, dec!(-100)),
            (Position::Btn, dec!(-100)),
        ]);
        assert_eq!(real_wins, suppose_wins);
    }
    #[test]
    #[ignore = "Without rake"]
    fn easy_one() {
        let all_positions_and_money = vec![HashMap::from([
            (Position::Sb, dec!(0.5)),
            (Position::Bb, dec!(38)),
            (Position::Utg, dec!(99)),
            (Position::Mp, dec!(147)),
            (Position::Co, dec!(11)),
            (Position::Btn, dec!(0)),
        ])];
        let real_hands_end = HashMap::from([
            (
                Position::Utg,
                ReadyHand::TwoPair {
                    top: Rank::Seven,
                    bottom: Rank::Two,
                    kicker: Rank::Ace,
                },
            ),
            (
                Position::Mp,
                ReadyHand::Trips {
                    trips: Rank::Three,
                    top_kicker: Rank::Ace,
                    low_kicker: Rank::Five,
                },
            ),
        ]);
        let real_wins = eval_clear_win_loose(all_positions_and_money, &real_hands_end, None);
        let suppose_wins = HashMap::from([
            (Position::Sb, dec!(-0.5)),
            (Position::Bb, dec!(-38)),
            (Position::Utg, dec!(-99)),
            (Position::Mp, dec!(148.5)),
            (Position::Co, dec!(-11)),
            (Position::Btn, dec!(0)),
        ]);
        assert_eq!(real_wins, suppose_wins);
    }
    #[test]
    #[ignore = "Without rake"]
    fn easy_two() {
        let all_positions_and_money = vec![HashMap::from([
            (Position::Sb, dec!(100)),
            (Position::Bb, dec!(100)),
            (Position::Utg, dec!(5)),
            (Position::Mp, dec!(150)),
            (Position::Co, dec!(0)),
            (Position::Btn, dec!(1)),
        ])];
        let real_hands_end = HashMap::from([
            (
                Position::Mp,
                ReadyHand::TwoPair {
                    top: Rank::Seven,
                    bottom: Rank::Two,
                    kicker: Rank::Ace,
                },
            ),
            (
                Position::Sb,
                ReadyHand::Trips {
                    trips: Rank::Three,
                    top_kicker: Rank::Ace,
                    low_kicker: Rank::Five,
                },
            ),
            (
                Position::Bb,
                ReadyHand::Trips {
                    trips: Rank::Three,
                    top_kicker: Rank::Ace,
                    low_kicker: Rank::Five,
                },
            ),
        ]);
        let real_wins = eval_clear_win_loose(all_positions_and_money, &real_hands_end, None);
        let suppose_wins = HashMap::from([
            (Position::Sb, dec!(53)),
            (Position::Bb, dec!(53)),
            (Position::Utg, dec!(-5)),
            (Position::Mp, dec!(-100)),
            (Position::Co, dec!(0)),
            (Position::Btn, dec!(-1)),
        ]);
        assert_eq!(real_wins, suppose_wins);
    }
    #[test]
    #[ignore = "Without rake"]
    fn easy_three() {
        let all_positions_and_money = vec![HashMap::from([
            (Position::Sb, dec!(0.5)),
            (Position::Bb, dec!(1)),
            (Position::Utg, dec!(30)),
            (Position::Mp, dec!(140)), //+
            (Position::Co, dec!(20)),  //+
            (Position::Btn, dec!(0)),
        ])];
        let real_hands_end = HashMap::from([
            (Position::Mp, ReadyHand::StreetFlash(Rank::King)),
            (Position::Co, ReadyHand::FlashRoal),
        ]);
        let real_wins = eval_clear_win_loose(all_positions_and_money, &real_hands_end, None);
        let suppose_wins = HashMap::from([
            (Position::Sb, dec!(-0.5)),
            (Position::Bb, dec!(-1)),
            (Position::Utg, dec!(-30)),
            (Position::Mp, dec!(-10)),
            (Position::Co, dec!(41.5)),
            (Position::Btn, dec!(0)),
        ]);
        assert_eq!(real_wins, suppose_wins);
    }
    #[test]
    #[ignore = "Without rake"]
    fn easy_four() {
        let all_positions_and_money = vec![HashMap::from([
            (Position::Sb, dec!(0.5)),
            (Position::Bb, dec!(1)),
            (Position::Utg, dec!(3.5)),
            (Position::Mp, dec!(12)), //+
            (Position::Co, dec!(0)),
            (Position::Btn, dec!(0)),
        ])];
        let real_wins = eval_clear_win_loose_five_fold(all_positions_and_money, None);
        let suppose_wins = HashMap::from([
            (Position::Sb, dec!(-0.5)),
            (Position::Bb, dec!(-1)),
            (Position::Utg, dec!(-3.5)),
            (Position::Mp, dec!(5)),
            (Position::Co, dec!(0)),
            (Position::Btn, dec!(0)),
        ]);
        assert_eq!(real_wins, suppose_wins);
    }
    #[test]
    #[ignore = "Without rake"]
    fn easy_four_alternative() {
        let all_positions_and_money = vec![HashMap::from([
            (Position::Sb, dec!(0.5)),
            (Position::Bb, dec!(1)),
            (Position::Utg, dec!(3.5)),
            (Position::Mp, dec!(12)), //+
            (Position::Co, dec!(0)),
            (Position::Btn, dec!(0)),
        ])];
        let real_hands_end = HashMap::from([(Position::Mp, ReadyHand::Street(Rank::Ace))]);
        let real_wins = eval_clear_win_loose(all_positions_and_money, &real_hands_end, None);
        let suppose_wins = HashMap::from([
            (Position::Sb, dec!(-0.5)),
            (Position::Bb, dec!(-1)),
            (Position::Utg, dec!(-3.5)),
            (Position::Mp, dec!(5)),
            (Position::Co, dec!(0)),
            (Position::Btn, dec!(0)),
        ]);
        assert_eq!(real_wins, suppose_wins);
    }
}
