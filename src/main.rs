use ::redis::RedisResult;
use clap::Parser;
use lazy_static::lazy_static;
use neiro_om::{
    action,
    eval_hand::*,
    eval_result,
    inline::fakeboard,
    postflop_game::{
        eval_fake_hand::{fake_comb_side_fd, fake_comb_side_ready, fake_comb_side_sd},
        fake_postflop::{
            AgroStreet, FakeBoardNew, FakePostflopHand, FakeSuitPostFlop, PotentialFE, Utils,
        },
        FakeBoard, FakePostflopPause, FakeStreet, PostflopGame,
    },
    preflop_game,
    redis::{RedisStreet, RedisUtils},
    strategy::GraphPoint,
    ActionKind, Branch, Card, FakePostReadyHand, FakePostflopNew, FakePreflopPause, Game, Hand,
    Node, Position, PreflopGame, Spr, MAP_INLINE_RANKS_RIVER, MAP_INLINE_REALCOMB,
    MAP_INLINE_SUITS_RIVER,
};
use rand::Rng;
use redis::Connection;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_json;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::format,
    io::{Read, Write},
    iter,
    sync::Mutex,
    thread,
    time::Instant,
    usize,
};

static DEBUG_REAL_MODE: bool = false;
static DEBUG_GRAPHS: bool = false;

static mut GLOBAL_GENERATION: u8 = 0;
static mut PREV_GRAPH: Option<HashMap<FakePostflopNew, Vec<GraphPoint>>> = None;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start generation number. From 0 to ... Default = 0.
    #[arg(short, long, default_value_t = 0)]
    generation_arg: u8,

    /// Number of times to create new generation. Default = 1.
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

struct ConfigPostflop {
    game: PostflopGame,
    ch_board_str: bool,
    prev_agr_pose: Option<Position>,
    fake_board: FakeBoardNew,
}

fn main() {
    println!(
        "Map-rank-river inline loaded size: {}",
        MAP_INLINE_RANKS_RIVER.len()
    );
    println!(
        "Map-suit-river inline loaded size: {}",
        MAP_INLINE_SUITS_RIVER.len()
    );
    println!(
        "Map-real-comb inline loaded size: {}",
        MAP_INLINE_REALCOMB.len()
    );
    // fakeboard::inline_real_combination();
    // thread::available_parallelism() = 12
    // gen_multithread_serde_games(10);
    // check_games();
    // std::process::exit(0);

    let args = Args::parse();

    println!("Start generation: {}!", args.generation_arg);
    println!("Number of times to create new generation: {}!", args.count);

    // Считаю мапу со всеми играми.
    let mut file = std::fs::File::open("river_fake_and_game.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let games_str: HashMap<String, Vec<(FakePostflopNew, Position, ReadyHand)>> =
        serde_json::from_str(&contents).unwrap();
    println!("Count of river games inlined: {}", games_str.len());
    let mut games: Vec<(PostflopGame, Vec<(FakePostflopNew, Position, ReadyHand)>)> =
        Vec::with_capacity(games_str.len());
    for (k, v) in games_str {
        let river_game: PostflopGame = serde_json::from_str(&k).unwrap();
        games.push((river_game, v));
    }

    unsafe {
        GLOBAL_GENERATION = args.generation_arg;
    }
    for _ in 1..=args.count {
        // Считываю граф решений предыдушего поколения.
        unsafe {
            PREV_GRAPH = if GLOBAL_GENERATION == 0 {
                None
            } else {
                Some(read_graph(GLOBAL_GENERATION - 1))
            }
        };

        gen_multithread_preflop_postflop_games(10, games.clone());
        unsafe {
            GLOBAL_GENERATION += 1;
        }
    }
}
fn allin_count(game: &PreflopGame) -> u8 {
    game.positions_and_money
        .iter()
        .fold(0u8, |acc, (&pose, &v)| {
            let player = game.player_by_position_as_ref(pose);
            if player.stack_size == v {
                acc + 1
            } else {
                acc
            }
        })
}
fn _print_details_preflop(map: &HashMap<FakePostflopNew, Vec<GraphPoint>>) {
    let mut hand = HashSet::new();
    let mut board = HashSet::new();
    let mut spr = HashSet::new();
    let mut blockers = HashSet::new();
    let mut hboard = HashSet::new();
    let mut hagro = HashSet::new();

    for (fake, _) in map {
        hand.insert(fake.my_fake_hand);
        board.insert(fake.fake_board);
        spr.insert(fake.spr);
        blockers.insert(fake.blockers);
        hboard.insert(fake.ch_board_str);
        hagro.insert(fake.prev_agr);
    }

    println!(
        "f.hands: {}, f. boards: {}, f.spr: {}, f.blockers: {}, f.chboard: {}, f.prevagro: {}",
        hand.len(),
        board.len(),
        spr.len(),
        blockers.len(),
        hboard.len(),
        hagro.len()
    );

    println!("{:?}", hand);
    println!("{:?}", board);
    println!("{:?}", spr);
    println!("{:?}", blockers);
    println!("{:?}", hboard);
    println!("{:?}", hagro);
}
fn gen_multithread_preflop_postflop_games(
    workers_count: u8,
    games: Vec<(PostflopGame, Vec<(FakePostflopNew, Position, ReadyHand)>)>,
) {
    let mut result: HashMap<FakePostflopNew, Vec<GraphPoint>> = HashMap::new();

    // Разобью общий список играемых рук на "workers_count" списков
    let mut lists = split(games, workers_count);

    let mut handles = Vec::new();
    for _ in 1..=workers_count {
        let cur_map = lists.split_off(lists.len() - 1)[0].clone();
        let handle = thread::spawn(|| gen_games(cur_map));
        handles.push(handle);
    }
    for handle in handles {
        let map_spawn = handle.join().unwrap();
        for (fake, graph_points) in map_spawn {
            let v = result
                .entry(fake)
                .or_insert(GraphPoint::get_all_graph_points());
            for point in graph_points.iter() {
                let p = v.iter_mut().find(|p| p.node == point.node).unwrap();
                p.hands += point.hands;
                p.win += point.win;
            }
        }
    }
    println!(
        "Generation: {}. Number of keys: {}",
        unsafe { GLOBAL_GENERATION },
        result.len()
    );
    // if DEBUG_GRAPHS {
    //     for (fake, graph) in &result {
    //         println!("---------{:?}--------", fake);
    //         GraphPoint::print_graph(graph);
    //     }
    // }
    _print_details_preflop(&result);
    serde_result(result);
}

fn serde_result(result: HashMap<FakePostflopNew, Vec<GraphPoint>>) {
    let generation = unsafe { GLOBAL_GENERATION };
    let mut new_map = HashMap::new();
    for (k, v) in result {
        let k_str = serde_json::to_string(&k).unwrap();
        if let Some(_) = new_map.insert(k_str, v) {
            println!("Doubled");
        }
    }
    let content_json_str = serde_json::to_string(&new_map).unwrap();
    let file_name = format!("b_river_{}.txt", generation);
    write_to_file(content_json_str, &file_name);
}

fn split(
    mut games: Vec<(PostflopGame, Vec<(FakePostflopNew, Position, ReadyHand)>)>,
    workers_count: u8,
) -> Vec<Vec<(PostflopGame, Vec<(FakePostflopNew, Position, ReadyHand)>)>> {
    let mut result = Vec::with_capacity(11);

    let step_size = games.len() / workers_count as usize;
    for _ in 1..workers_count {
        let vec2 = games.split_off(games.len() - step_size);
        result.push(vec2);
    }
    result.push(games);
    result
}
fn gen_multithread_serde_games(workers_count: u8) {
    // (Ключ, действие)(накапливаем сумму результатов, накапливаем счетчик когда встречалось=количество розыгрышей)
    let mut result = HashMap::new();
    let mut handles = Vec::new();
    for _ in 1..=workers_count {
        let handle = thread::spawn(|| gen_serde_games_river());
        handles.push(handle);
    }
    for handle in handles {
        let map_spawn = handle.join().unwrap();
        for (k, v) in map_spawn {
            if let Some(_) = result.insert(k, v) {
                println!("Duble river");
            };
        }
    }
    let file_name = format!("river_fake_and_game.txt");
    let content_json_str = serde_json::to_string(&result).unwrap();
    write_to_file(content_json_str, &file_name);
}
fn gen_games(
    games: Vec<(PostflopGame, Vec<(FakePostflopNew, Position, ReadyHand)>)>,
) -> HashMap<FakePostflopNew, Vec<GraphPoint>> {
    let cur_gen = unsafe { GLOBAL_GENERATION };
    println!("Thread river games inlined: {}", games.len());

    let prev_gen_graphs = unsafe { &PREV_GRAPH };

    let time = Instant::now();
    let mut fakes_graphs = HashMap::new();
    for (river_game, vec_situation) in games {
        let first_situation = vec_situation[0].clone();
        let second_situation = vec_situation[1].clone();

        let mut fakes_positions = HashMap::new();
        let mut real_hands_end = HashMap::new();

        fakes_positions.insert(first_situation.1, first_situation.0.clone());
        fakes_positions.insert(second_situation.1, second_situation.0.clone());

        real_hands_end.insert(first_situation.1, first_situation.2);
        real_hands_end.insert(second_situation.1, second_situation.2);

        // Подготавливаем итоговый граф раздачи.
        fakes_graphs
            .entry(first_situation.0.clone())
            .or_insert(GraphPoint::get_all_graph_points());
        fakes_graphs
            .entry(second_situation.0.clone())
            .or_insert(GraphPoint::get_all_graph_points());

        if cur_gen == 0 {
            // Играем все ветки по этой раздаче. Для поколения 0.
            let brances = Branch::all_branches();
            for branch in brances.into_iter() {
                let mut real_hands_end_current = real_hands_end.clone();
                let mut river_game_current = river_game.clone();
                // Непосредственная игра по ветке.
                let nodes_by_poses = play_river(
                    Some(branch),
                    &mut river_game_current,
                    &mut real_hands_end_current,
                    &fakes_positions,
                    &prev_gen_graphs,
                );
                // Расчет результата розигрыша по ветке.
                // println!("real_hands_end {:?}", real_hands_end);
                let winners = eval_result::eval_clear_win_loose(
                    vec![
                        // preflop_game.positions_and_money,
                        // flop_game.positions_and_money,
                        // turn_game.positions_and_money,
                        river_game_current.positions_and_money.clone(),
                    ],
                    &real_hands_end_current,
                    Some(river_game_current.main_pot.prev_street_end_size),
                );
                if DEBUG_REAL_MODE {
                    println!("{:?}", winners);
                }
                update_win_in_graf(
                    &nodes_by_poses,
                    &fakes_positions,
                    &winners,
                    &mut fakes_graphs,
                );
            }
        } else {
            let mut real_hands_end_current = real_hands_end.clone();
            let mut river_game_current = river_game.clone();
            // Непосредственная игра по лучшим нодам.
            let nodes_by_poses = play_river(
                None,
                &mut river_game_current,
                &mut real_hands_end_current,
                &fakes_positions,
                prev_gen_graphs,
            );
            // Расчет результата розигрыша по ветке.
            // println!("real_hands_end {:?}", real_hands_end);
            let winners = eval_result::eval_clear_win_loose(
                vec![
                    // preflop_game.positions_and_money,
                    // flop_game.positions_and_money,
                    // turn_game.positions_and_money,
                    river_game_current.positions_and_money.clone(),
                ],
                &real_hands_end_current,
                Some(river_game_current.main_pot.prev_street_end_size),
            );
            update_win_in_graf(
                &nodes_by_poses,
                &fakes_positions,
                &winners,
                &mut fakes_graphs,
            );
            if DEBUG_REAL_MODE {
                println!("{:?}", winners);
                break;
            }
        }
    }
    if cur_gen != 0 {
        join_graphs(&mut fakes_graphs, prev_gen_graphs);
    }

    println!("Seconds gone: {}", time.elapsed().as_secs());
    fakes_graphs
}

fn join_graphs(
    fakes_graphs: &mut HashMap<FakePostflopNew, Vec<GraphPoint>>,
    prev_gen_graphs: &Option<HashMap<FakePostflopNew, Vec<GraphPoint>>>,
) {
    let prev_graphs = prev_gen_graphs.clone().unwrap();
    for (cur_fake, cur_points) in fakes_graphs {
        for cur_point in cur_points {
            if cur_point.hands == 0 {
                let prev_points = prev_graphs.get(cur_fake).unwrap();
                let prev_point = prev_points
                    .iter()
                    .find(|x| x.node == cur_point.node)
                    .unwrap();
                cur_point.hands = prev_point.hands;
                cur_point.win = prev_point.win;
            }
        }
    }
}

fn read_graph(cur_gen: u8) -> HashMap<FakePostflopNew, Vec<GraphPoint>> {
    let filename = format!("b_river_{}.txt", cur_gen);
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let graphs_str: HashMap<String, Vec<GraphPoint>> = serde_json::from_str(&contents).unwrap();

    let mut graphs = HashMap::with_capacity(graphs_str.len() + 10);
    for (k, v) in graphs_str {
        let fake: FakePostflopNew = serde_json::from_str(&k).unwrap();
        graphs.insert(fake, v);
    }
    graphs
}

fn update_win_in_graf(
    my_nodes: &HashMap<Position, Vec<Node>>,
    fakes_positions: &HashMap<Position, FakePostflopNew>,
    winners: &HashMap<Position, Decimal>,
    fakes_graphs: &mut HashMap<FakePostflopNew, Vec<GraphPoint>>,
) {
    for (pose, nodes) in my_nodes {
        // Get win by position.
        let result = *winners.get(pose).unwrap();
        // Get fake by position.
        let fake = fakes_positions.get(pose).unwrap();
        // Update full graph by fake.
        let graph_for_fake = fakes_graphs.get_mut(fake).unwrap();
        for node in nodes {
            let point = graph_for_fake.iter_mut().find(|p| &p.node == node).unwrap();
            point.hands += 1;
            point.win += result;
        }

        // for node in nodes {
        //     let point = graph.iter_mut().find(|p| &p.node == node).unwrap();
        //     point.hands += 1;
        //     point.win += result;
        // }
    }
}
#[allow(non_snake_case)]
fn preflop(
    all_fake_pre: &mut Vec<(Position, String, u8, Decimal)>,
    debug_real_mode: bool,
    debug_fake_mode: bool,
    con: &mut redis::Connection,
    real_network_player: &Vec<Position>,
) -> PreflopGame {
    let GENERATION = unsafe { GLOBAL_GENERATION };

    if debug_real_mode {
        println!("----------PREFLOP---------");
    }
    let poses = vec![
        Position::Utg,
        Position::Mp,
        Position::Co,
        Position::Btn,
        Position::Sb,
        Position::Bb,
    ];
    let mut preflop_game = PreflopGame::new();
    for &position in poses.iter().cycle() {
        if !preflop_game.folded_positions.contains(&position)
            && preflop_game.end_of_hand_five_foldes()
        {
            if debug_real_mode {
                println!("All fold, {:?} win!", position);
            }
            break;
        }
        /* rnd_raise_size нужен чтобы понять каким рейзом рандомно пробуем играть.
        Я просто вынес этот рандомайзер из процедуры possible_action_kind()
        Генерирует число от 1 до 3
        1 => size_75_raise,
        2 => size_50_raise,
        _ => size_pot_raise,
         */
        let possible_act = action::possible_action_kind(&preflop_game, position);

        /* Заканчиваем розагрыш на этой улице, если или или:
        1. Все игроки либо в фолде, либо в алине (possible_act кстати пустой)
        2. Все игроки кроме меня в фолде или олине (possible_act кстати пустой)
         */
        if preflop_game.end_of_street(&possible_act, position) {
            break;
        }
        if possible_act.is_empty() {
            /* Так как это не конец улицы, значит пустой набор возможных действий означает, что эта
            позиция либо в алине либо в фолде.
            В таком случае игроку не нужно совершать действие => не нужно делать точку принятия решения
            и записывать в базу.
             */
            continue;
        }
        let fake_game_pause = FakePreflopPause::from(&preflop_game, position);
        // let fake_game_pause = FakePreflopPause::mock();
        let choosen_act = if GENERATION == 0 || real_network_player.contains(&position) {
            ActionKind::rnd_action_from(&possible_act)
        } else {
            get_act_from_last_gens_pre(&fake_game_pause, &possible_act, con, 3)
        };
        if debug_real_mode {
            let player = preflop_game.player_by_position_as_ref(position);
            println!(
                "{:?} ({:?}) [pot.b. {}] [bet {}] -> {:?}",
                player,
                possible_act,
                preflop_game.main_pot.value,
                preflop_game.min_bet,
                choosen_act.unwrap(),
            );
        }
        if debug_fake_mode {
            println!("{:?}", fake_game_pause);
        }
        if debug_real_mode {
            println!(
                "                                                                               {}",
                RedisUtils::get_preflop_key(&fake_game_pause, GENERATION)
            );
        }
        if GENERATION == 0 || real_network_player.contains(&position) {
            all_fake_pre.push((
                position,
                RedisUtils::get_preflop_key(&fake_game_pause, GENERATION),
                RedisUtils::get_action_id(choosen_act.unwrap(), &possible_act),
                Decimal::ZERO,
            ));
        }

        preflop_game.do_action_on_position(choosen_act, position);
        // println!(
        //     "{:?} ",
        //     action::already_commit_by_pos(&preflop_game, position)
        // );
    }
    if debug_real_mode {
        println!("In all in {}", allin_count(&preflop_game));
    }
    return preflop_game;
}
#[allow(non_snake_case)]
fn flop(
    preflop_game: &PreflopGame,
    fake_pauses_flop: &mut Vec<(Position, String, u8, Decimal)>,
    debug_real_mode: bool,
    debug_fake_mode: bool,
    con: &mut redis::Connection,
    real_network_player: &Vec<Position>,
    prev_agr_pose: Option<Position>,
) -> PostflopGame {
    let GENERATION = unsafe { GLOBAL_GENERATION };

    if debug_real_mode {
        println!("----------FLOP---------");
    }
    let poses = vec![
        Position::Sb,
        Position::Bb,
        Position::Utg,
        Position::Mp,
        Position::Co,
        Position::Btn,
    ];
    let mut flop_game = PostflopGame::from(preflop_game);
    // Fakes+
    let fake_board = Utils::new_fake_flop_board(&flop_game);
    let ch_board_str = false;
    let mut fake_hands = HashMap::new();
    poses
        .iter()
        .filter(|&pos| !flop_game.folded_positions.contains(&pos))
        .for_each(|&pos| {
            let player = flop_game.player_by_position_as_ref(pos);
            let combination = real_comb(&player.hand, &flop_game.cards);
            let fake_hand = FakePostflopHand {
                ready: fake_comb_side_ready(&player.hand, combination, &flop_game.cards),
                flash_draw: fake_comb_side_fd(&player.hand, combination, &flop_game.cards),
                street_draw: fake_comb_side_sd(&player.hand, combination, &flop_game.cards),
            };
            fake_hands.insert(pos, fake_hand);
        });
    // Fakes-
    if debug_real_mode {
        println!("{:?}", flop_game);
    }
    let mut cyrcle_count = 0_u8;
    for &position in poses.iter().cycle() {
        if position == Position::Sb {
            cyrcle_count += 1;
        }
        if !flop_game.folded_positions.contains(&position) && flop_game.end_of_hand_five_foldes() {
            if debug_real_mode {
                println!("All fold, {:?} win!", position);
            }
            break;
        }
        // Если все кто мог сделать экшн чекнули на постфлопе, то заканчиваем улицу и переходим на следующую.
        if cyrcle_count > 1 && flop_game.no_money_in_game() {
            if debug_real_mode {
                println!("All checks who can");
            }
            break;
        }
        let player = flop_game.player_by_position_as_ref(position);
        let possible_act = action::possible_action_kind(&flop_game, position);
        if flop_game.end_of_street(&possible_act, position) {
            break;
        }
        if possible_act.is_empty() {
            /* Так как это не конец улицы, значит пустой набор возможных действий означает, что эта
            позиция либо в алине либо в фолде.
            В таком случае игроку не нужно совершать действие => не нужно делать точку принятия решения
            и записывать в базу.
            */
            continue;
        }
        // Fakes+
        let fake_situation = Utils::postflop_situation(&flop_game, player);
        let fake_game_pause = FakePostflopPause::from_parts(
            *fake_hands
                .get(&position)
                .expect("Fake hands not evaluate for pose"),
            fake_board,
            fake_situation,
            ch_board_str,
            AgroStreet::calculate(&prev_agr_pose, position),
        );
        // let fake_game_pause = FakePostflopPause::mock();
        // Fakes-
        let choosen_act = if GENERATION == 0 || real_network_player.contains(&position) {
            ActionKind::rnd_action_from(&possible_act)
        } else {
            get_act_from_last_gens(&fake_game_pause, &RedisStreet::Flop, &possible_act, con, 3)
        };
        if debug_real_mode {
            let combination = real_comb(&player.hand, &flop_game.cards);
            println!(
                "{:?} {:?} ({:?}) [pot.b. {}] [m.bet {}] -> {:?}",
                player,
                combination,
                possible_act,
                flop_game.main_pot.value,
                flop_game.min_bet,
                choosen_act.unwrap(),
            );
        }
        if debug_fake_mode {
            println!("{:?}", fake_game_pause);
        }
        if debug_real_mode {
            println!(
                "                                                                               {}",
                RedisUtils::get_postflop_key(&fake_game_pause, GENERATION, &RedisStreet::Flop)
            );
        }
        if GENERATION == 0 || real_network_player.contains(&position) {
            fake_pauses_flop.push((
                position,
                RedisUtils::get_postflop_key(&fake_game_pause, GENERATION, &RedisStreet::Flop),
                RedisUtils::get_action_id(choosen_act.unwrap(), &possible_act),
                Decimal::ZERO,
            ));
        }
        // Fakes-

        flop_game.do_action_on_position(choosen_act, position);

        //println!("{:?} ", action::already_commit_by_pos(&flop_game, position));
    }
    return flop_game;
}
#[allow(non_snake_case)]
fn turn(
    flop_game: &PostflopGame,
    fake_pauses_turn: &mut Vec<(Position, String, u8, Decimal)>,
    debug_real_mode: bool,
    debug_fake_mode: bool,
    con: &mut redis::Connection,
    real_network_player: &Vec<Position>,
    prev_agr_pose: &mut Option<Position>,
) -> PostflopGame {
    let GENERATION = unsafe { GLOBAL_GENERATION };
    let mut current_agr = None;

    if debug_real_mode {
        println!("----------TURN---------");
    }
    let poses = vec![
        Position::Sb,
        Position::Bb,
        Position::Utg,
        Position::Mp,
        Position::Co,
        Position::Btn,
    ];
    let mut turn_game = PostflopGame::from(flop_game);
    // Fakes+
    let fake_board = Utils::new_fake_flop_board(&turn_game);
    let prev_fake_board = Utils::new_fake_flop_board(&flop_game);
    if debug_real_mode {
        println!("Prev board {:?}", turn_game.cards);
    }
    let ch_board_str = fake_board != prev_fake_board;
    let mut fake_hands = HashMap::new();
    poses
        .iter()
        .filter(|&pos| !turn_game.folded_positions.contains(&pos))
        .for_each(|&pos| {
            let player = turn_game.player_by_position_as_ref(pos);
            let combination = real_comb(&player.hand, &turn_game.cards);
            let fake_hand = FakePostflopHand {
                ready: fake_comb_side_ready(&player.hand, combination, &turn_game.cards),
                flash_draw: fake_comb_side_fd(&player.hand, combination, &turn_game.cards),
                street_draw: fake_comb_side_sd(&player.hand, combination, &turn_game.cards),
            };
            fake_hands.insert(pos, fake_hand);
        });
    // Fakes-
    if debug_real_mode {
        println!("{:?}", turn_game);
    }
    let mut cyrcle_count = 0_u8;
    for &position in poses.iter().cycle() {
        if position == Position::Sb {
            cyrcle_count += 1;
        }
        if !turn_game.folded_positions.contains(&position) && turn_game.end_of_hand_five_foldes() {
            if debug_real_mode {
                println!("All fold, {:?} win!", position);
            }
            break;
        }
        // Если все кто мог сделать экшн чекнули на постфлопе, то заканчиваем улицу и переходим на следующую.
        if cyrcle_count > 1 && turn_game.no_money_in_game() {
            if debug_real_mode {
                println!("All checks who can");
            }
            break;
        }
        let player = turn_game.player_by_position_as_ref(position);
        let possible_act = action::possible_action_kind(&turn_game, position);

        if turn_game.end_of_street(&possible_act, position) {
            break;
        }
        if possible_act.is_empty() {
            /* Так как это не конец игры, значит пустой набор возможных действий означает, что эта
            позиция либо в алине либо в фолде.
            В таком случае игроку не нужно совершать действие => не нужно делать точку принятия решения
            и записывать в базу.
             */
            continue;
        }
        // Faks+
        let fake_situation = Utils::postflop_situation(&turn_game, player);
        let fake_game_pause = FakePostflopPause::from_parts(
            *fake_hands
                .get(&position)
                .expect("Fake hands not evaluate for pose"),
            fake_board,
            fake_situation,
            ch_board_str,
            AgroStreet::calculate(&prev_agr_pose, position),
        );
        // let fake_game_pause = FakePostflopPause::mock();
        // Faks-
        let choosen_act = if GENERATION == 0 || real_network_player.contains(&position) {
            ActionKind::rnd_action_from(&possible_act)
        } else {
            let rnd_deep_search = match GENERATION {
                1 => 1u8,
                _ => 2,
            };
            if debug_real_mode {
                println!(
                    "           ------ Try to find in prev gen: {}",
                    RedisUtils::get_postflop_key(
                        &fake_game_pause,
                        GENERATION - 1,
                        &RedisStreet::Turn
                    )
                );
            }
            get_act_from_last_gens(
                &fake_game_pause,
                &RedisStreet::Turn,
                &possible_act,
                con,
                rnd_deep_search,
            )
        };
        if debug_real_mode {
            let combination = real_comb(&player.hand, &turn_game.cards);
            println!(
                "{:?} {:?} ({:?}) [pot.b. {}] [m.bet {}] -> {:?}",
                player,
                combination,
                possible_act,
                turn_game.main_pot.value,
                turn_game.min_bet,
                choosen_act.unwrap(),
            );
        }
        if debug_fake_mode {
            println!(
                "           ------ We choose action: {}",
                RedisUtils::get_action_id(choosen_act.unwrap(), &possible_act)
            );
            println!("{:?}", fake_game_pause)
        };
        if GENERATION == 0 || real_network_player.contains(&position) {
            fake_pauses_turn.push((
                position,
                RedisUtils::get_postflop_key(&fake_game_pause, GENERATION, &RedisStreet::Turn),
                RedisUtils::get_action_id(choosen_act.unwrap(), &possible_act),
                Decimal::ZERO,
            ));
        }
        // Fakes-

        turn_game.do_action_on_position(choosen_act, position);

        if let ActionKind::Raise(_) = choosen_act.unwrap() {
            current_agr = Some(position);
        }

        //println!("{:?} ", action::already_commit_by_pos(&flop_game, position));
    }
    *prev_agr_pose = current_agr;
    return turn_game;
}
fn play_river(
    branch: Option<Branch>,
    river_game: &mut PostflopGame,
    real_hands_end: &mut HashMap<Position, ReadyHand>,
    fakes_positions: &HashMap<Position, FakePostflopNew>,
    prev_gen_graphs: &Option<HashMap<FakePostflopNew, Vec<GraphPoint>>>,
) -> HashMap<Position, Vec<Node>> {
    if DEBUG_REAL_MODE {
        println!("----------RIVER---------");
        println!("----------BR: {:?}", branch);
    }
    let poses = vec![
        Position::Sb,
        Position::Bb,
        Position::Utg,
        Position::Mp,
        Position::Co,
        Position::Btn,
    ];
    if DEBUG_REAL_MODE {
        println!("{:?}", river_game);
    }
    let mut nodes_by_poses: HashMap<Position, Vec<Node>> = HashMap::new();
    let mut action_count = 0_usize;
    let mut prev_node = None;
    let mut cyrcle_count = 0_u8;
    for &position in poses.iter().cycle() {
        if position == Position::Sb {
            cyrcle_count += 1;
        }
        let all_fold_or_allin = river_game
            .positions_and_money()
            .iter()
            .all(|(&pos, &money)| {
                river_game.player_by_position_as_ref(pos).stack_size == money
                    || river_game.folded_positions().contains(&pos)
            });
        if all_fold_or_allin {
            break;
        }
        if river_game.folded_positions().contains(&position)
            || river_game.position_in_allin(position)
        {
            continue;
        }

        let possible_act = action::possible_action_kind(river_game, position);
        if !river_game.folded_positions().contains(&position) && possible_act.is_empty() {
            /* Если по какой-то причине пустой набор вариантов возможных действий, то это паника в селе, спятил дед
             */
            break;
        }
        // Если все кто мог сделать экшн чекнули на постфлопе, то заканчиваем улицу и переходим на следующую.
        if cyrcle_count > 1 && river_game.no_money_in_game() {
            if DEBUG_REAL_MODE {
                println!("All checks who can");
            }
            break;
        }
        if possible_act.is_empty() {
            /* Так как это не конец игры, значит пустой набор возможных действий означает, что эта
            позиция либо в алине либо в фолде.
            В таком случае игроку не нужно совершать действие => не нужно делать точку принятия решения
            и записывать в базу.
             */
            continue;
        }

        let node = if let Some(branch) = &branch {
            let Some(&node) = branch.path.get(action_count) else {
                break;
            };
            node
        } else {
            let cur_fake = fakes_positions.get(&position).unwrap();
            let prev_graphs = prev_gen_graphs.clone().unwrap();
            best_node(cur_fake, prev_node, prev_graphs)
            // Node::B100
        };
        let act = Node::action_from_node(node, river_game.main_pot.value, &possible_act);

        if DEBUG_REAL_MODE {
            let player = river_game.player_by_position_as_ref(position);
            let combination = real_comb(&player.hand, &river_game.cards);
            println!(
                "{:?} {:?} ({:?}) [pot {}] [m.bet {}] -> {:?}",
                player,
                combination,
                possible_act,
                river_game.main_pot.value,
                river_game.min_bet,
                act,
            );
        }

        river_game.do_action_on_position(Some(act), position);

        action_count += 1;
        prev_node = Some(node);
        nodes_by_poses
            .entry(position)
            .and_modify(|v| v.push(node))
            .or_insert(vec![node]);
        //println!("{:?} ", action::already_commit_by_pos(&flop_game, position));
    }
    // За розыгрыш ривера могут сфолдить, поэтому из real_hands_end они исключаются
    // потомучто там должны храниться только комбинации между которых будет делиться банк
    poses
        .iter()
        .filter(|&pos| river_game.folded_positions().contains(&pos))
        .for_each(|&pos| {
            real_hands_end.remove(&pos);
        });
    nodes_by_poses
}

fn best_node(
    cur_fake: &FakePostflopNew,
    prev_node: Option<Node>,
    prev_graphs: HashMap<FakePostflopNew, Vec<GraphPoint>>,
) -> Node {
    let possible_nodes = if let Some(p_node) = prev_node {
        p_node.childrens()
    } else {
        Node::start_nodes()
    };
    let graph = prev_graphs.get(cur_fake).unwrap();
    if DEBUG_GRAPHS {
        println!("---------{:?}--------", cur_fake);
        let debg = graph
            .iter()
            .cloned()
            .filter(|x| possible_nodes.contains(&x.node))
            .collect::<Vec<_>>();
        println!("{:#?}", debg);
    }
    graph
        .iter()
        .filter(|x| possible_nodes.contains(&x.node))
        .max_by(|&&x, &&y| {
            let wr_x = if x.hands != 0 {
                x.win / Decimal::from(x.hands)
            } else {
                Decimal::ZERO
            };
            let wr_y = if y.hands != 0 {
                y.win / Decimal::from(y.hands)
            } else {
                Decimal::ZERO
            };
            wr_x.cmp(&wr_y)
        })
        .unwrap()
        .node
}
#[allow(non_snake_case)]
fn river(
    turn_game: &PostflopGame,
    fake_pauses_river: &mut Vec<(Position, String, u8, Decimal)>,
    debug_real_mode: bool,
    debug_fake_mode: bool,
    real_hands_end: &mut HashMap<Position, ReadyHand>,
    con: &mut redis::Connection,
    real_network_player: &Vec<Position>,
    prev_agr_pose: Option<Position>,
) -> PostflopGame {
    let GENERATION = unsafe { GLOBAL_GENERATION };
    if debug_real_mode {
        println!("----------RIVER---------");
    }
    let poses = vec![
        Position::Sb,
        Position::Bb,
        Position::Utg,
        Position::Mp,
        Position::Co,
        Position::Btn,
    ];
    let mut river_game = PostflopGame::from(turn_game);
    // Fakes+
    let fake_board = Utils::new_fake_flop_board(&river_game);
    let prev_fake_board = Utils::new_fake_flop_board(&turn_game);
    if debug_real_mode {
        println!("Prev board {:?}", turn_game.cards);
    }
    let ch_board_str = fake_board != prev_fake_board;
    // let fake_board = Utils::fake_flop_board_inline(
    //     &river_game,
    //     &MAP_INLINE_RANKS_RIVER,
    //     &MAP_INLINE_SUITS_RIVER,
    // );
    let mut fake_hands = HashMap::new();
    poses
        .iter()
        .filter(|&pos| !river_game.folded_positions().contains(&pos))
        .for_each(|&pos| {
            let player = river_game.player_by_position_as_ref(pos);
            let combination = real_comb(&player.hand, &river_game.cards);
            let fake_hand = FakePostflopHand {
                ready: fake_comb_side_ready(&player.hand, combination, &river_game.cards),
                flash_draw: fake_comb_side_fd(&player.hand, combination, &river_game.cards),
                street_draw: fake_comb_side_sd(&player.hand, combination, &river_game.cards),
            };
            fake_hands.insert(pos, fake_hand);
            real_hands_end.insert(pos, combination);
        });
    // Fakes-
    if debug_real_mode {
        println!("{:?}", river_game);
    }
    let mut cyrcle_count = 0_u8;
    for &position in poses.iter().cycle() {
        if position == Position::Sb {
            cyrcle_count += 1;
        }
        if !river_game.folded_positions.contains(&position) && river_game.end_of_hand_five_foldes()
        {
            if debug_real_mode {
                println!("All fold, {:?} win!", position);
            }
            break;
        }
        // Если все кто мог сделать экшн чекнули на постфлопе, то заканчиваем улицу и переходим на следующую.
        if cyrcle_count > 1 && river_game.no_money_in_game() {
            if debug_real_mode {
                println!("All checks who can");
            }
            break;
        }
        let player = river_game.player_by_position_as_ref(position);
        let possible_act = action::possible_action_kind(&river_game, position);

        if river_game.end_of_street(&possible_act, position) {
            break;
        }
        if possible_act.is_empty() {
            /* Так как это не конец игры, значит пустой набор возможных действий означает, что эта
            позиция либо в алине либо в фолде.
            В таком случае игроку не нужно совершать действие => не нужно делать точку принятия решения
            и записывать в базу.
             */
            continue;
        }
        // Faks+
        let fake_situation = Utils::postflop_situation(&river_game, player);
        let fake_game_pause = FakePostflopPause::from_parts(
            *fake_hands
                .get(&position)
                .expect("Fake hands not evaluate for pose"),
            fake_board,
            fake_situation,
            ch_board_str,
            AgroStreet::calculate(&prev_agr_pose, position),
        );
        // let fake_game_pause = FakePostflopPause::mock();
        // Faks-
        let choosen_act: Option<ActionKind> =
            if GENERATION == 0 || real_network_player.contains(&position) {
                ActionKind::rnd_action_from(&possible_act)
            } else {
                let rnd_deep_search = match GENERATION {
                    1 => 1u8,
                    _ => 1,
                };
                if debug_real_mode {
                    println!(
                        "           ------ Try to find in prev gen: {}",
                        RedisUtils::get_postflop_key(
                            &fake_game_pause,
                            GENERATION - 1,
                            &RedisStreet::River
                        )
                    );
                }
                get_act_from_last_gens(
                    &fake_game_pause,
                    &RedisStreet::River,
                    &possible_act,
                    con,
                    rnd_deep_search,
                )
            };
        if debug_real_mode {
            let combination = real_comb(&player.hand, &river_game.cards);
            println!(
                "{:?} {:?} ({:?}) [pot.b. {}] [m.bet {}] -> {:?}",
                player,
                combination,
                possible_act,
                river_game.main_pot.value,
                river_game.min_bet,
                choosen_act.unwrap(),
            );
        }
        if debug_fake_mode {
            println!(
                "           ------ We choose action: {}",
                RedisUtils::get_action_id(choosen_act.unwrap(), &possible_act)
            );
            println!("{:?}", fake_game_pause)
        };
        if GENERATION == 0 || real_network_player.contains(&position) {
            fake_pauses_river.push((
                position,
                RedisUtils::get_postflop_key(&fake_game_pause, GENERATION, &RedisStreet::River),
                RedisUtils::get_action_id(choosen_act.unwrap(), &possible_act),
                Decimal::ZERO,
            ));
        }
        // Fakes-

        river_game.do_action_on_position(choosen_act, position);

        //println!("{:?} ", action::already_commit_by_pos(&flop_game, position));
    }
    // За розыгрыш ривера могут сфолдить, поэтому из real_hands_end они исключаются
    // потомучто там должны храниться только комбинации между которых будет делиться банк
    poses
        .iter()
        .filter(|&pos| river_game.folded_positions().contains(&pos))
        .for_each(|&pos| {
            real_hands_end.remove(&pos);
        });
    return river_game;
}
fn rnd_raise_size() -> u8 {
    rand::Rng::gen_range(&mut rand::thread_rng(), 1u8..=3u8)
}
fn rnd_raise_size_to_string(rnd_raise_size: u8, choosen_act: Option<ActionKind>) -> String {
    /*
    - Для любого действия, кроме рейза не имеет смысла и выводится пустая строка
    - Для рейза код полностью ОБЯЗАН повторять код из процедуры action::possible_action_kind()
     */
    if let Some(ActionKind::Raise(_)) = choosen_act {
        match rnd_raise_size {
            1 => "75%".to_string(),
            2 => "50%".to_string(),
            _ => "100%".to_string(),
        }
    } else {
        String::new()
    }
}
fn add_wins_to_redis_fakes(
    fakes: &mut Vec<(Position, String, u8, Decimal)>,
    winners: &HashMap<Position, Decimal>,
    all_fakes: &mut BTreeMap<(String, u8), (Decimal, Decimal)>,
) {
    // На старте последний элемент кортежа всегда 0. Вот здесь его заполняем.
    for (pose, _, _, result) in fakes.iter_mut() {
        let win = winners
            .get(pose)
            .expect("error: Didn't find win/loose in pose");
        *result = *win;
    }
    // Добавляем в список записей все фейковые структуры.
    for (_, key_fake, id_action, result) in fakes {
        let hands_result = all_fakes
            .entry((key_fake.to_owned(), *id_action))
            .or_insert((Decimal::ZERO, Decimal::ZERO));
        hands_result.0 += *result;
        hands_result.1 += dec!(1);
    }
}
#[allow(non_snake_case)]
fn get_act_from_last_gens(
    fake_game_pause: &FakePostflopPause,
    street: &RedisStreet,
    possible_act: &Vec<ActionKind>,
    con: &mut Connection,
    number_las_gens: u8,
) -> Option<ActionKind> {
    assert_ne!(street, &RedisStreet::Preflop);
    let GENERATION = unsafe { GLOBAL_GENERATION };
    // println!("----");
    /*
    Пробный алгоритм для того, чтобы не было четное поколение супер тайт, нечетное супер агро.
    Пусть у меня нпс-игрок будет играть случайно по прошлому или позапрошлому поколению (number_las_gens = 2), кроме GENERATION = 1
     */
    let number = rand::Rng::gen_range(&mut rand::thread_rng(), 1..=number_las_gens);
    let post_key = RedisUtils::get_postflop_key(fake_game_pause, GENERATION - number, street);
    // Ошибка только если ошибка подключения в редиске. Если нет ключа/действия, то Ok(None)
    let act = RedisUtils::best_action(possible_act, post_key, con).unwrap();
    if act.is_some() {
        // println!(
        //     "{}",
        //     RedisUtils::get_postflop_key(fake_game_pause, GENERATION - 1, street)
        // );
        // println!("notzero#{}", GENERATION);
        return act;
    }
    let post_key = RedisUtils::get_postflop_key(fake_game_pause, 0, street);
    // Ошибка только если ошибка подключения в редиске. Если нет ключа/действия, то Ok(None)
    let act = RedisUtils::best_action(possible_act, post_key, con).unwrap();
    if act.is_some() {
        // println!(
        //     "{}",
        //     RedisUtils::get_postflop_key(fake_game_pause, 0, street)
        // );
        // println!("zero#{}", GENERATION);
        return act;
    }

    // for number in 1..=number_las_gens {
    //     let post_key = RedisUtils::get_postflop_key(fake_game_pause, GENERATION - number, street);
    //     // Ошибка только если ошибка подключения в редиске. Если нет ключа/действия, то Ok(None)
    //     let act = RedisUtils::best_action(possible_act, post_key, con).unwrap();
    //     if act.is_some() {
    //         // println!(
    //         //     "{}",
    //         //     RedisUtils::get_postflop_key(fake_game_pause, GENERATION - 1, street)
    //         // );
    //         return act;
    //     }
    // }

    println!(
        "#rnd{}+ {}",
        GENERATION,
        RedisUtils::get_postflop_key(fake_game_pause, GENERATION - number, street)
    );
    ActionKind::rnd_action_from(possible_act)
}
#[allow(non_snake_case)]
fn get_act_from_last_gens_pre(
    fake_game_pause: &FakePreflopPause,
    possible_act: &Vec<ActionKind>,
    con: &mut Connection,
    number_las_gens: u8,
) -> Option<ActionKind> {
    // println!("----");
    let GENERATION = unsafe { GLOBAL_GENERATION };
    for number in 1..=number_las_gens {
        let pre_key = RedisUtils::get_preflop_key(fake_game_pause, GENERATION - number);
        // Ошибка только если ошибка подключения в редиске. Если нет ключа/действия, то Ok(None)
        let act = RedisUtils::best_action(&possible_act, pre_key, con).unwrap();
        if act.is_some() {
            // println!("#{}+", GENERATION - number);
            return act;
        }
    }
    // println!("#rnd+");
    ActionKind::rnd_action_from(&possible_act)
}

#[allow(dead_code)]
fn print_end_redis_fakes(records: &BTreeMap<(String, u8), (Decimal, Decimal)>) {
    let mut print_key = None;
    for ((key, id), (result, hands)) in records {
        if print_key == None || key != print_key.unwrap() {
            println!(
                "{:?}\n   action: {:?} res: {:?} hands: {:?} wr: {}",
                key,
                id,
                result,
                hands,
                (result / hands).round()
            )
        } else {
            println!(
                "   action: {:?} res: {:?} hands: {:?} wr: {}",
                id,
                result,
                hands,
                (result / hands).round()
            )
        }
        print_key = Some(key);
    }
}
#[allow(dead_code)]
fn syntetic_preflop(lock_cards: &Vec<Card>) -> PreflopGame {
    if lock_cards.is_empty() {
        PreflopGame::new()
    } else {
        PreflopGame::new_with_lock_cards(lock_cards)
    }
}
#[allow(dead_code)]
fn syntetic_postflop(init_game: &impl Game) -> PostflopGame {
    // Everybody cheched. No need to modify any property of the game state machine(pot,map,folded,...)
    // let bottles = if is_friday { 3 } else { 1 };
    PostflopGame::from(init_game)
}
fn syntetic_river(lock_cards: &Vec<Card>, spr: Decimal) -> ConfigPostflop {
    let preflop_game = syntetic_preflop(lock_cards);
    let flop_game = syntetic_postflop(&preflop_game);
    let turn_game = syntetic_postflop(&flop_game);
    let mut river_game = syntetic_postflop(&turn_game);

    let fake_board = Utils::new_fake_flop_board(&river_game);
    let prev_fake_board = Utils::new_fake_flop_board(&turn_game);
    // let ch_board_str = fake_board != prev_fake_board;
    let ch_board_str = cacl_change_board(fake_board, prev_fake_board);

    let prev_agr_pose = modify_game_ml(&mut river_game, spr);
    ConfigPostflop {
        game: river_game,
        ch_board_str,
        prev_agr_pose,
        fake_board,
    }
}

fn cacl_change_board(fake_board: FakeBoardNew, prev_fake_board: FakeBoardNew) -> bool {
    if prev_fake_board.paired {
        return false;
    }
    // Ниже предыдущий борд всегда неспаренный.
    if fake_board.paired {
        return true;
    }
    // Ниже все борды всегда неспаренные.

    if prev_fake_board.suit_kind != FakeSuitPostFlop::Flash
        && fake_board.suit_kind != FakeSuitPostFlop::Flash
    {
        return true;
    }

    if prev_fake_board.suit_kind != FakeSuitPostFlop::Flash
        && prev_fake_board.street_kind != FakeStreet::Street
        && fake_board.street_kind == FakeStreet::Street
    {
        return true;
    }

    false
}
fn temporary_modify_river(init_game: &mut PostflopGame, position_real_player: &Position) {
    // SORTED !!!
    init_game.cards = vec![
        Card::from_string_ui("Ac".to_string()),
        Card::from_string_ui("Kc".to_string()),
        Card::from_string_ui("Ts".to_string()),
        Card::from_string_ui("9c".to_string()),
        Card::from_string_ui("2s".to_string()),
    ];
    let player = init_game.player_by_position_as_mut_ref(*position_real_player);
    // Not sorted !!!
    player.hand = Hand::new(
        Card::from_string_ui("Js".to_string()),
        Card::from_string_ui("Qh".to_string()),
        Card::from_string_ui("5s".to_string()),
        Card::from_string_ui("2h".to_string()),
    )
    .unwrap();
}
fn modify_game_ml(init_game: &mut PostflopGame, spr: Decimal) -> Option<Position> {
    let mut rnd = rand::thread_rng();

    /* Сгенерирую сфолдвшие позиции.
    100% - ХА
    20% - 3-вей
    5%  - 4-вей
    3%  - 5-вей
    2%  - 6-вей
    */
    let num_of_folded_players = match rnd.gen_range(1..=100_u8) {
        1..=100 => 4,
        _ => unreachable!(),
    };
    let mut play_positions = vec![
        Position::Utg,
        Position::Mp,
        Position::Co,
        Position::Btn,
        Position::Sb,
        Position::Bb,
    ];
    let mut folded_position = HashSet::new();
    for _ in 1..=num_of_folded_players {
        let index = rnd.gen_range(0..play_positions.len());
        folded_position.insert(play_positions.remove(index));
    }
    init_game.folded_positions_as_mut_ref().clear();
    init_game
        .folded_positions_as_mut_ref()
        .extend(folded_position);

    /* Сгенерируем размер пота в зависимости от количества игроков в поте.
    от 6х до 200
    Терн
    2 - (12-75)
    3 - (18-75)
    4 - (24-75)
    5 - (30-75)
    6 - (36-75)
    Флоп конец улицы, т.е. начало терна
    2 - (12-50)
    */
    let players_in_play_count = play_positions.len();
    // let pot_value = rnd.gen_range(6 * players_in_play_count..=30);
    let pot_value = 20;
    init_game.main_pot_as_mut_ref().value = Decimal::from(pot_value);
    init_game.main_pot_as_mut_ref().prev_street_end_size = Decimal::from(pot_value);

    /* Сгенерируем размер стеков на ривере в зависимости от размера пота.
    Если размер пота большой, то размер стеков поменьше и наоборот.
    Большой = >75 спр от 0 до 1.25
    Средний = 50-75 спр от 0.75 до 3
    Маленький = <50 спр от 0.75 до 3
     */
    init_game.players.iter_mut().for_each(|player| {
        player.stack_size = match pot_value {
            // 1..=50 => Decimal::from(rnd.gen_range(25..=150)),
            // 51..=75 => Decimal::from(rnd.gen_range(50..=150)),
            // 76..=100 => Decimal::from(rnd.gen_range(50..=130)),
            _ => spr,
            // _ => Decimal::from(rnd.gen_range(40..=200)),
        }
    });
    /* Сгенерирую случайного агрессора на предыдущей улице
    Так как играется всегда ХА, то пусть три события - я 33%, не я 33%, никто 33%
     */
    let p = *play_positions
        .get(rnd.gen_range(0..play_positions.len()))
        .unwrap();
    match rnd.gen_range(0..=2u8) {
        0 | 1 => Some(p),
        2 => None,
        _ => unreachable!(),
    }
}
fn rnd_one_positions_not_folded(init_game: &impl Game) -> Position {
    loop {
        let pose = Position::rnd_position();
        if !init_game.folded_positions().contains(&pose) {
            return pose;
        }
    }
}
fn gen_serde_games_river() -> HashMap<String, Vec<(FakePostflopNew, Position, ReadyHand)>> {
    let mut rnd = rand::thread_rng();

    let mut fakes_count = HashMap::new();
    let mut serde_river = HashMap::new();
    let mut fakes = HashSet::new();

    let count_of_games_min = 100;
    let mut cc = 0_usize;
    loop {
        if !fakes_count.is_empty() && *fakes_count.values().min().unwrap() > count_of_games_min {
            break;
        }
        // if !fakes_count.is_empty() && *fakes_count.values().min().unwrap() > cc {
        //     println!("{cc}");
        //     cc = *fakes_count.values().min().unwrap();
        // }
        // Create a new game with full random, except the spr for now.
        let lock_cards = vec![];
        let spr = match rnd.gen_range(0..=2u8) {
            0 => dec!(200),
            1 => dec!(53),
            2 => dec!(20),
            _ => unreachable!(),
        };
        let config = syntetic_river(&lock_cards, spr);

        let mut river_game: PostflopGame = config.game;
        let prev_agr_pose = config.prev_agr_pose;
        let ch_board_str = config.ch_board_str;

        let specific_board = true;

        let real_player_hand = Hand::rnd_hand(&river_game.cards);

        // Tuple for serde
        let mut tuples = vec![];

        // Calculate the fakes for the new game.
        let mut min_count_fake = usize::MAX;
        Position::all_poses()
            .iter()
            .filter(|&pos| !river_game.folded_positions().contains(&pos))
            .for_each(|&pos| {
                let player = river_game.player_by_position_as_ref(pos);
                let combination = real_comb(&player.hand, &river_game.cards);

                let fake_hand = FakePostflopHand {
                    ready: fake_comb_side_ready(&player.hand, combination, &river_game.cards),
                    flash_draw: fake_comb_side_fd(&player.hand, combination, &river_game.cards),
                    street_draw: fake_comb_side_sd(&player.hand, combination, &river_game.cards),
                };

                let blockers =
                    Utils::we_have_blockers(&player.hand.cards, &config.fake_board, &river_game);

                let fake = FakePostflopNew {
                    // river: 4*15*2*2*3*3=2160
                    fake_board: config.fake_board,
                    my_fake_hand: fake_hand,
                    blockers,
                    ch_board_str,
                    prev_agr: AgroStreet::calculate(&prev_agr_pose, pos),
                    spr: Spr::from(spr),
                };
                // serde
                tuples.push((fake.clone(), pos, combination));
                fakes.insert(fake.clone());

                fakes_count
                    .entry(fake.clone())
                    .and_modify(|val| *val += 1)
                    .or_insert(1_usize);
                min_count_fake = min_count_fake.min(*fakes_count.get(&fake).unwrap());
            });
        if min_count_fake <= count_of_games_min {
            let river_json_str = serde_json::to_string(&river_game).unwrap();
            if let Some(_) = serde_river.insert(river_json_str, tuples) {
                println!("Duble river: {:?}", river_game);
            };
        }
    }

    let file_name = format!("river_fakes.txt");
    let content_json_str = serde_json::to_string(&fakes).unwrap();
    write_to_file(content_json_str, &file_name);

    serde_river
}

fn write_to_file(content: String, file_name: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(file_name)?;
    f.write_all(content.as_bytes())?;
    Ok(())
}
fn check_games() -> std::io::Result<()> {
    let mut file = std::fs::File::open("river_fake_and_game.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let games_str: HashMap<String, Vec<(FakePostflopNew, Position, ReadyHand)>> =
        serde_json::from_str(&contents).unwrap();
    println!("Count of river games: {}", games_str.len());

    let mut file = std::fs::File::open("river_fakes.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let fakes: HashSet<FakePostflopNew> = serde_json::from_str(&contents).unwrap();
    println!("Count of fakes: {}", fakes.len());

    for fake in &fakes {
        let games_for_fake = games_str
            .clone()
            .into_iter()
            .filter(|(_, v)| v[0].0 == *fake || v[1].0 == *fake)
            .collect::<HashMap<String, Vec<(FakePostflopNew, Position, ReadyHand)>>>();
        println!("Fake: {:?} Games: {}", fake, games_for_fake.len());

        // let mut map_count = HashMap::new();
        // for (_, v) in &games_for_fake {
        //     if v[0].0 == *fake {
        //         map_count
        //             .entry(v[1].0.clone())
        //             .and_modify(|val| *val += 1usize)
        //             .or_insert(1usize);
        //     } else {
        //         map_count
        //             .entry(v[0].0.clone())
        //             .and_modify(|val| *val += 1usize)
        //             .or_insert(1usize);
        //     }
        // }
        // println!("Enemies: {:?}", map_count);

        // if fake.blockers == false
        //     && fake.fake_board == FakeBoardNew::StreetNoFlashNoPair
        //     && fake.my_fake_hand.ready == FakePostReadyHand::NutStreet
        // {
        //     for g in &games_for_fake {
        //         let game: PostflopGame = serde_json::from_str(g.0).unwrap();
        //         println!("b: {:?}", game.cards);
        //         let player = game.player_by_position_as_ref(g.1[0].1);
        //         println!("p1: {:?}", player.hand);
        //         let player = game.player_by_position_as_ref(g.1[1].1);
        //         println!("p2: {:?}", player.hand);
        //     }
        // }
        // if fake.blockers == true
        //     && fake.fake_board == FakeBoardNew::StreetNoFlashNoPair
        //     && fake.my_fake_hand.ready == FakePostReadyHand::NutStreet
        // {
        //     for g in &games_for_fake {
        //         let game: PostflopGame = serde_json::from_str(g.0).unwrap();
        //         println!("b: {:?}", game.cards);
        //         let player = game.player_by_position_as_ref(g.1[0].1);
        //         println!("p1: {:?}", player.hand);
        //         let player = game.player_by_position_as_ref(g.1[1].1);
        //         println!("p2: {:?}", player.hand);
        //     }
        // }
    }

    Ok(())
}
