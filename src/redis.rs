use redis::{from_redis_value, streams::StreamRangeReply, Commands, Connection, RedisResult};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use rust_decimal_macros::dec;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    io::Write,
};

use crate::{ActionKind, FakePostflopPause, FakePreflopPause, Position};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum RedisStreet {
    Preflop,
    Flop,
    Turn,
    River,
}
impl Display for RedisStreet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RedisStreet::Preflop => "preflop",
                RedisStreet::Flop => "flop",
                RedisStreet::Turn => "turn",
                RedisStreet::River => "river",
            }
        )
    }
}
pub struct RedisUtils;

impl RedisUtils {
    pub fn connect() -> RedisResult<Connection> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let con = client.get_connection()?;
        Ok(con)
    }
    pub fn get_preflop_key(fake: &FakePreflopPause, generation: u8) -> String {
        // "0#preflop#1|2|3|4"
        let mut key = String::new();
        key.push_str(&generation.to_string());
        key.push('#');
        key.push_str("preflop");
        key.push('#');
        match fake.biggest_action {
            crate::FakeAction::OpenRaise => key.push('0'),
            crate::FakeAction::ThreeBet => key.push('1'),
            crate::FakeAction::FourBetAndMore => key.push('2'),
        }
        key.push('|');
        match fake.agressor_position {
            crate::FakePositionAction::Early => key.push('0'),
            crate::FakePositionAction::Late => key.push('1'),
            crate::FakePositionAction::BigB => key.push('2'),
            crate::FakePositionAction::SmallB => key.push('3'),
        }
        key.push('|');
        match fake.my_position {
            crate::Position::Sb => key.push('0'),
            crate::Position::Bb => key.push('1'),
            crate::Position::Utg => key.push('2'),
            crate::Position::Mp => key.push('3'),
            crate::Position::Co => key.push('4'),
            crate::Position::Btn => key.push('5'),
        }
        key.push('|');
        key.push_str(format!("{:?}", fake.my_fake_hand).trim());
        key.push('|');
        match fake.my_ratio_commit {
            crate::RatioNeedCoomitToPot::Bad => key.push('0'),
            crate::RatioNeedCoomitToPot::Good => key.push('1'),
        }
        key.push('|');
        match fake.calc_playing_stack {
            crate::FakeStackSize::Shallow => key.push('0'),
            crate::FakeStackSize::Deep => key.push('1'),
        }
        key
    }
    pub fn get_action_id(action: ActionKind, possible_act: &Vec<ActionKind>) -> u8 {
        /*
        0-fold
        1-check
        2-call
        3-raise50
        4-raise75
        5-raise100
        Для рейза определение id такое:
        - Если рейз в возможных действиях один, то это всегда 50%
        - Если рейза в возможных действиях 2 и это максимальный сайзинг, то это всегда 75% иначе 50%
        - Если рейза в возможных действиях три и это максимальный сайзинг, то это всегда 100%, если минимальный, то 50%, иначе 75%
         */
        let mut count_raises = 0;
        let mut max_raise = Decimal::MIN;
        let mut min_raise = Decimal::MIN;
        possible_act
            .iter()
            .filter(|&&x| {
                if let ActionKind::Raise(_) = x {
                    true
                } else {
                    false
                }
            })
            .map(|&x| {
                if let ActionKind::Raise(val) = x {
                    val
                } else {
                    unreachable!()
                }
            })
            .for_each(|x| {
                max_raise = Decimal::max(x, max_raise);
                min_raise = Decimal::min(x, min_raise);
                count_raises += 1u8;
            });

        match action {
            ActionKind::Fold => 0,
            ActionKind::Check => 1,
            ActionKind::Call(_) => 2,
            ActionKind::Raise(val) => match count_raises {
                1 => 3,
                2 => {
                    if val == min_raise {
                        3
                    } else {
                        4
                    }
                }
                3 => {
                    if val == min_raise {
                        3
                    } else if val == max_raise {
                        5
                    } else {
                        4
                    }
                }
                _ => unreachable!(),
            },
        }
    }
    pub fn get_full_record(
        fakes: &Vec<(Position, String, u8, Decimal)>,
    ) -> BTreeMap<(String, u8), (Decimal, Decimal)> {
        // Vec<(Position, String, u8, Decimal)> -> HM{(String, u8), (sumResult, sumHands)}
        // Sort HM{(String, u8), (sumResult, sumHands)} -> frist key, second id
        let mut hm_key_id = BTreeMap::new();
        for (_, key_fake, id_action, result) in fakes {
            let hands_result = hm_key_id
                .entry((key_fake.to_owned(), *id_action))
                .or_insert((Decimal::ZERO, Decimal::ZERO));
            hands_result.0 += result;
            hands_result.1 += dec!(1);
        }
        hm_key_id
    }
    pub fn write_to_redis(
        record: &BTreeMap<(String, u8), (Decimal, Decimal)>,
        file_name: &str,
    ) -> RedisResult<()> {
        let mut f: std::fs::File = std::fs::File::create(file_name)?;

        let mut con = RedisUtils::connect()?;
        /*
        Отключил очистку базы, та как хочу, записывать много поколений в цикле.
        Потомучто новые поколения должны брать оттуда варианты своих действий с глубиной до 3-х.

        redis::cmd("FLUSHALL").query(&mut con)?;
        */
        for ((key, id), (result, hands)) in record {
            let winrate = ((result / hands) * dec!(100)).round();
            writeln!(
                f,
                "xadd {} {}-9 hands {} winrate {}",
                key,
                id,
                hands.to_i32().unwrap(),
                winrate.to_i32().unwrap()
            )
            .unwrap();
            // con.xadd("0#preflop#1|2|3|4", "0-1", &[("hands", 1000), ("winr", 15)])?;
            let _: () = con.xadd(
                key,
                format!("{}-{}", id, 9),
                &[
                    ("hands", hands.to_i32().unwrap()),
                    ("winrate", winrate.to_i32().unwrap()),
                ],
            )?;
        }
        Ok(())
    }
    pub fn get_postflop_key(
        fake: &FakePostflopPause,
        generation: u8,
        street: &RedisStreet,
    ) -> String {
        // Important: always use Display, not Debug here

        let mut key = String::new();
        key.push_str(&generation.to_string());
        key.push('#');
        key.push_str(&street.to_string());
        key.push('#');
        key.push_str(format!("{}", fake.my_fake_hand).trim());
        key.push('|');
        key.push_str(format!("{}", fake.fake_board).trim());
        key.push('|');
        key.push_str(format!("{}", fake.situation).trim());
        key.push('|');
        key.push_str(format!("{}", fake.prev_agr).trim());
        key.push('|');
        key.push_str(format!("ch.{}", fake.ch_board_str).trim());
        key
    }
    #[allow(unused_variables)]
    pub fn best_action(
        possible_act: &Vec<ActionKind>,
        key: String,
        con: &mut Connection,
    ) -> RedisResult<Option<ActionKind>> {
        /* Logic:
        - Err вернется только если паника при подключении к редиске.
        - Ok(None) вернется
            1. если в редиске не найден такой ключ впринципе.
            2. если в редиске найден ключ, но там действие, которое нет в списках возможных
            Это не считается ошибкой. Типо просто не достаточно много симуляций. Тогда просто сделаю
            случайное действие в мейне. ЭТО ДОВОЛЬНО РЕДКО, ВСЕГО 64 НА 100_000 РУК = 0.064% (0-поколение)
                                                                  154_000 НА 100_000 РУК (4#)
                                                                  rnd 6029 НА 2_000_000 ситуаций = 0.3% (№4,№3,№2)
                                                                  39_000 на 100_000_000 РУК = 0.039% (2-поколение)
                                                                  ------------------------------------------------
                                                                  Gen2 vs gen1 100_000 hands are rnd 2.3%
                                                                  Gen3 vs gen2 100_000 hands are rnd 0.062%
                                                                  Gen4 vs gen3 100_000 hands are rnd 0.1%
         */
        let key_debug = key.clone();
        // print!("{}", key);
        let range: StreamRangeReply = con.xrange_all(key)?;

        let mut bests_id = vec![];

        // let mut raise_result = 0;
        // let mut raise_hands = 0;

        for stream_id in range.ids {
            // print!("\n{} ", stream_id.id);
            let significant_part_of_id = &stream_id.id[0..=0];
            let id = significant_part_of_id.parse::<u8>().unwrap();

            let mut winrate = None;
            let mut hands = None;
            for (key, value) in stream_id.map {
                let val = from_redis_value::<isize>(&value)?;
                // print!(" {}: {:?}", key, val);
                if key == "winrate" {
                    winrate = Some(val);
                } else {
                    hands = Some(val);
                }
            }
            // if id == 3 || id == 4 || id == 5 {
            //     raise_result += hands.unwrap() * winrate.unwrap() / 100;
            //     raise_hands += hands.unwrap();
            // } else {
            if hands.is_some() && hands.unwrap() >= 5 {
                bests_id.push((winrate.unwrap(), id));
            }
            // }
        }
        // if raise_hands > 10 {
        //     let sum_winrate = raise_result * 100 / raise_hands;
        //     bests_id.push((sum_winrate, 3));
        //     // print!("\n->3 ");
        //     // print!(" winrate: {:?}", sum_winrate);
        //     // print!(" hands: {:?}", raise_hands);
        // }
        // Sorted from best winrate to worst. If winrate is the same, then pick minimum id (minimize despertion)
        bests_id.sort_unstable_by(|&a, &b| b.0.partial_cmp(&a.0).unwrap());
        // println!(
        //     "
        //     ------ Real in redis: {:?}, pos.act: {:?}",
        //     bests_id, possible_act
        // );
        if let Some(act) = pick_best_from_possible(bests_id, possible_act) {
            Ok(Some(act))
        } else {
            // println!("{}", key_debug);
            Ok(None)
        }
    }
}
fn pick_best_from_possible(
    bests_id: Vec<(isize, u8)>,
    possible_act: &Vec<ActionKind>,
) -> Option<ActionKind> {
    let raises_in_possible_act = possible_act
        .iter()
        .filter_map(|&act| {
            if let ActionKind::Raise(_) = act {
                Some(act)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    // Если нет рейзов+, если 1 рейз+, если 2 рейза+, если 3 рейза+
    let first_raise = raises_in_possible_act.first().map(|x| *x);
    let last_raise = raises_in_possible_act.last().map(|x| *x);
    let mid_raise = if raises_in_possible_act.len() <= 2 {
        first_raise.clone()
    } else {
        raises_in_possible_act.get(1).map(|x| *x)
    };
    for (_, action) in bests_id {
        let founded = match action {
            0 => possible_act
                .iter()
                .find(|&&x| {
                    if let ActionKind::Fold = x {
                        true
                    } else {
                        false
                    }
                })
                .map(|x| *x),
            1 => possible_act
                .iter()
                .find(|&&x| {
                    if let ActionKind::Check = x {
                        true
                    } else {
                        false
                    }
                })
                .map(|x| *x),
            2 => possible_act
                .iter()
                .find(|&&x| {
                    if let ActionKind::Call(_) = x {
                        true
                    } else {
                        false
                    }
                })
                .map(|x| *x),
            3 => last_raise,
            4 => mid_raise,
            5 => first_raise,
            // Рейзы всегда идут от большего к меньшему в possible_act.
            // 3 | 4 | 5 => possible_act
            //     .iter()
            //     .find(|&&x| {
            //         if let ActionKind::Raise(_) = x {
            //             true
            //         } else {
            //             false
            //         }
            //     })
            //     .map(|x| *x),
            _ => panic!("error 1: cant find best action"),
        };
        if founded.is_some() {
            // println!("\npicked {:?}", founded);
            return founded;
        }
    }
    None
}

pub fn start_redis() -> redis::RedisResult<HashMap<String, usize>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let _: () = redis::cmd("FLUSHALL").query(&mut con)?;
    let _: () = redis::cmd("SET").arg("new_key").arg(42).query(&mut con)?;
    let _ = redis::cmd("GET").arg("new_key").query::<u8>(&mut con)?;

    /*
    0-fold
    1-check
    2-call
    3-raise50
    4-raise75
    5-raise100
     */
    let _: () = con.xadd("0#preflop#1|2|3|4", "0-1", &[("hands", 1000), ("winr", 15)])?;
    let _: () = con.xadd("0#preflop#1|2|3|4", "2-1", &[("hands", 750), ("winr", 2)])?;
    let _: () = con.xadd("0#preflop#1|2|3|4", "5-1", &[("hands", 750), ("winr", -20)])?;
    let range: StreamRangeReply = con.xrange_all("0#preflop#1|2|3|4")?;
    for id in range.ids {
        println!("{}", id.id);
        for (key, value) in id.map {
            let val = from_redis_value::<usize>(&value)?;
            println!("--->>{}: {:?}", key, val);
        }
    }

    let _: () = con.hset("0#flop#1|2|3|4", "hands", 15_000_usize)?;
    let _: () = con.hset("0#flop#1|2|3|4", "winrate", 15_usize)?;
    let r: RedisResult<HashMap<String, usize>> = con.hgetall("0#flop#1|2|3|4"); // -> redis::RedisResult<HashMap<String, usize>>
                                                                                //con.xrange_all("0#preflop#1|2|3|4")?
    r
}
