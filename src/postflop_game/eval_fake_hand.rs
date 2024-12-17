use std::collections::{HashMap, HashSet};

use crate::{Card, FakePostReadyHand, FakePostflopFD, FakePostflopSD, Hand, Rank, ReadyHand, Suit};

mod tests_eval;

struct ConfigBoard {
    board_ranks: Vec<Rank>, // Sorted if the boared was sorted
    board_ranks_contains_ace: bool,
}
impl ConfigBoard {
    fn from(board: &Vec<Card>) -> Self {
        let board_ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        Self {
            board_ranks_contains_ace: board_ranks.contains(&Rank::Ace),
            board_ranks,
        }
    }
}

pub fn fake_comb_side_ready(
    hand: &Hand,
    ready_hand: ReadyHand,
    board: &Vec<Card>, // Always sort from top to low!
) -> FakePostReadyHand {
    /* Так как некоторые категории пересекаются, то для производительности и чтобы убрать лишние
    проверки и тд, в логике важен порядок групп. От сильнейших к слабейшим.
    Наиболее явный пример, см. фулл-хауз или стрит или варианты трипсо-сетов
    ВАЖНО board - всегда должен быть отсортирован от большего ранга к меньшему*/
    let ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    let mut map = HashMap::new();
    ranks.iter().for_each(|&rank| {
        let v = map.entry(rank).or_insert(0u8);
        *v += 1;
    });

    // Супер редкая ошибка TTTT2 = KQ33
    if is_imba(ready_hand, board, &map) {
        return FakePostReadyHand::Imba;
    }
    // Также ситуация с трипсом на флопе не покрывается фейковой ситуацией, поэтому все такие фулхаузы
    // вручную уменьшают свою фейовую силу и отправляются в младший фуллхауз.
    if is_low_full_house(ready_hand) {
        return FakePostReadyHand::LowFullHouse;
    }
    if let Some(val) = is_nut_flash(hand, ready_hand, board) {
        return val;
    }
    if is_nut_street(ready_hand, board) {
        return FakePostReadyHand::NutStreet;
    }
    if is_no_nut_street(ready_hand) {
        return FakePostReadyHand::NoNutStreet;
    }
    if is_set(ready_hand, &map) {
        return FakePostReadyHand::Set;
    }
    if is_nut_trips(ready_hand, &map, &hand) {
        return FakePostReadyHand::TripsNutKicker;
    }
    if is_nonut_trips(ready_hand, &map) {
        return FakePostReadyHand::TripsLess;
    }
    if is_top_two(ready_hand, board, &map) {
        return FakePostReadyHand::TopTwo;
    }
    // Между категориями Топ-боттом и боттом-боттом немного размыта граница, из-за сложности
    // алгоритма на спаренных досках, на неспаренных ок, но это вообще не важная история!
    if is_top_bottom(ready_hand, board, &map) {
        return FakePostReadyHand::TopBottom;
    }
    if is_bottom_bottom(ready_hand) {
        return FakePostReadyHand::BottomBottom;
    }
    if is_toppair_overpair(ready_hand, board) {
        return FakePostReadyHand::TPOP;
    }
    FakePostReadyHand::Nothing
}
pub fn fake_comb_side_fd(hand: &Hand, ready_hand: ReadyHand, board: &Vec<Card>) -> FakePostflopFD {
    // Если у нас флеш, то пофигу на флешдро - не нужно считать время и не нужно плодить разные фейки.
    if let ReadyHand::Flash(_, _, _, _, _) = ready_hand {
        return FakePostflopFD::Nothing;
    }
    // Если у нас рука сильнее флеша, то не нужно плодить фейки руки, считая врапы.
    match ready_hand {
        ReadyHand::FlashRoal
        | ReadyHand::StreetFlash(..)
        | ReadyHand::Care(..)
        | ReadyHand::FullHouse { .. } => return FakePostflopFD::Nothing,
        _ => {}
    }
    // Если мы на ривере, то нет смысла плодить сущности ибо неготовые части руки не важны
    if board.len() == 5 {
        return FakePostflopFD::Nothing;
    }
    let suits_on_board = board.iter().map(|c| c.suit).collect::<Vec<Suit>>();
    let mut map_board = HashMap::new();
    suits_on_board.iter().for_each(|&suit| {
        let v = map_board.entry(suit).or_insert(0u8);
        *v += 1;
    });
    let suits_in_hand = hand.cards.iter().map(|c| c.suit).collect::<Vec<Suit>>();
    let mut map_hand = HashMap::new();
    suits_in_hand.iter().for_each(|&suit| {
        let v = map_hand.entry(suit).or_insert(0u8);
        *v += 1;
    });
    /* Логика:
    - ready_hand не должна быть флешом,
    - на ривере смысла искать флешдро в руке нет
    !- либо же если это флеш, то флеш-дро должно иметь другую масть. Это возможно только на ривере,
        поэтому можно это проигнорировать
    - в руке должно быть две или более масти и такие же ровно две масти(иначе флеш) должны быть на борде
    - в руке должна быть такая карта этой масти, которая равна максимальной из возможных за
        вычитом тех, которые есть есть на борде.
    */
    let ss_suits_on_board = map_board
        .iter()
        .filter(|(_, &v)| v == 2)
        .map(|(&k, _)| k)
        .collect::<HashSet<Suit>>();
    let ss_suits_in_hand = map_hand
        .iter()
        .filter(|(_, &v)| v >= 2)
        .map(|(&k, _)| k)
        .collect::<HashSet<Suit>>();
    let suits_possible_fd = ss_suits_on_board.intersection(&ss_suits_in_hand);
    let mut count_nut_fd = 0u8;
    let mut count_secondthree_fd = 0u8;
    suits_possible_fd.clone().for_each(|&fd| {
        let all_ranks_from_low_to_top = Rank::to_vec_from_low();
        let ranks_hand = hand
            .cards
            .iter()
            .filter(|&&card| card.suit == fd)
            .map(|&card| card.rank)
            .collect::<Vec<Rank>>();
        let ranks_board = board
            .iter()
            .filter(|&&card| card.suit == fd)
            .map(|&card| card.rank)
            .collect::<Vec<Rank>>();
        let search_rank_from_low = all_ranks_from_low_to_top
            .iter()
            .filter(|&&r| !ranks_board.contains(&r))
            .map(|&r| r)
            .collect::<Vec<Rank>>();
        let nut_rank = *search_rank_from_low
            .last()
            .unwrap_or_else(|| unreachable!());
        if ranks_hand.contains(&nut_rank) {
            count_nut_fd += 1;
        }
        let secondthree_rank =
            &search_rank_from_low[search_rank_from_low.len() - 4..=search_rank_from_low.len() - 2];
        if !ranks_hand.iter().all(|&r| !secondthree_rank.contains(&r)) {
            count_secondthree_fd += 1;
        }
    });
    let fd_count = suits_possible_fd.count();
    if fd_count == 2 && count_nut_fd >= 1 {
        FakePostflopFD::TwoFdWithNut
    } else if fd_count == 2 && count_nut_fd == 0 {
        FakePostflopFD::TwoFD
    } else if fd_count == 1 && count_nut_fd == 1 {
        FakePostflopFD::OneNutFD
    } else if fd_count == 1 && count_nut_fd == 0 && count_secondthree_fd >= 1 {
        FakePostflopFD::OneSecondThree
    } else if fd_count == 1 && count_nut_fd == 0 && count_secondthree_fd == 0 {
        FakePostflopFD::Low
    } else if ss_suits_in_hand.len() == 2
        && board.len() == 3
        && ss_suits_in_hand
            .iter()
            .all(|&suit| suits_on_board.contains(&suit))
    {
        FakePostflopFD::TwoBD
    } else {
        FakePostflopFD::Nothing
    }
}
pub fn fake_comb_side_sd(hand: &Hand, ready_hand: ReadyHand, board: &Vec<Card>) -> FakePostflopSD {
    if let ReadyHand::Street(_) = ready_hand {
        return FakePostflopSD::Nothing;
    }
    // Если у нас рука сильнее стрита, то не нужно плодить фейки руки, считая врапы.
    match ready_hand {
        ReadyHand::FlashRoal
        | ReadyHand::StreetFlash(..)
        | ReadyHand::Care(..)
        | ReadyHand::FullHouse { .. }
        | ReadyHand::Flash(..) => return FakePostflopSD::Nothing,
        _ => {}
    }
    if board.len() == 5 {
        return FakePostflopSD::Nothing;
    }
    // Get board properties
    let config_board = ConfigBoard::from(board);

    // Get all variants of five cards. Three from hand and two from board. Board and hand always sorted
    let mut two_cards_from_board = vec![];
    for i in 0..board.len() {
        for j in i + 1..board.len() {
            let v = vec![board[i], board[j]];
            two_cards_from_board.push(v);
        }
    }
    let mut trips_cards_from_hand = vec![];
    for i in 0..hand.cards.len() {
        for j in i + 1..hand.cards.len() {
            for k in j + 1..hand.cards.len() {
                let v = vec![hand.cards[i], hand.cards[j], hand.cards[k]];
                trips_cards_from_hand.push(v);
            }
        }
    }
    let mut all_five_cards = vec![];
    for pair in two_cards_from_board.iter() {
        for trips in trips_cards_from_hand.iter() {
            let mut v = pair.clone();
            v.extend_from_slice(&trips);
            all_five_cards.push(v);
        }
    }
    //println!("{:?}", all_five_cards);
    // Оптимизировано, чтобы не считать OESD, если уже был Wrap
    let mut was_nonut_wrap = false;
    let mut all_ready_hands = vec![];
    for five_cards in all_five_cards.iter() {
        let wrap_combination = wrap_combination(five_cards, &config_board, was_nonut_wrap);
        match wrap_combination {
            FakePostflopSD::NutWrap => return wrap_combination,
            FakePostflopSD::NoNutWrap => was_nonut_wrap = true,
            _ => {}
        }
        all_ready_hands.push(wrap_combination);
    }
    all_ready_hands.sort_unstable_by(|a, b| b.cmp(a));
    //println!("{:?}", all_ready_hands);
    *all_ready_hands.first().unwrap_or_else(|| unreachable!())
}
fn is_imba(ready_hand: ReadyHand, board: &Vec<Card>, map: &HashMap<Rank, u8>) -> bool {
    let val = *map.values().max().unwrap_or_else(|| unreachable!());
    match ready_hand {
        ReadyHand::FlashRoal | ReadyHand::StreetFlash(_) | ReadyHand::Care(_) => true,
        ReadyHand::FullHouse { trips, pair } if val == 2 => {
            // board with trips not this group. Paired or double paired board
            let top_board_rank = board[0].rank;
            let second_board_rank_if_toppared = board[2].rank;
            let paired_ranks = map
                .iter()
                .filter(|(_, &v)| v == val)
                .map(|(&k, _)| k)
                .collect::<Vec<Rank>>();
            if paired_ranks.contains(&pair) && trips == top_board_rank {
                // A7722, A8772 = AA**
                return true;
            } else if paired_ranks.contains(&trips)
                && trips == top_board_rank
                && pair == second_board_rank_if_toppared
            {
                // QQJJ6, QQJ73 = QJ**
                return true;
            }
            return false;
        }
        ReadyHand::FullHouse { trips, pair } if val == 4 => {
            let top_board_rank = board[0].rank;
            if trips == top_board_rank && top_board_rank > pair {
                // K2222 = KK**. Ошибка в структуре TTTT2 = AK33 супер редкая => ignore
                return true;
            }
            if trips == top_board_rank && trips != Rank::Ace && pair == Rank::Ace {
                // 77772 = AA**, 2222 = AA**
                return true;
            }
            if trips == top_board_rank && trips == Rank::Ace && pair == Rank::King {
                // AAAAK = KK**, AAAA = KK**
                return true;
            }
            false
        }
        _ => false,
    }
}
fn is_low_full_house(ready_hand: ReadyHand) -> bool {
    if let ReadyHand::FullHouse { trips: _, pair: _ } = ready_hand {
        true
    } else {
        false
    }
}
fn is_nut_flash(
    hand: &Hand,
    ready_hand: ReadyHand,
    board: &Vec<Card>,
) -> Option<FakePostReadyHand> {
    // if let ReadyHand::Flash(hight_card, _, _, _, _) = ready_hand {
    //     hight_card == Rank::Ace
    // } else {
    //     false
    // }
    let ReadyHand::Flash(_, _, _, _, _) = ready_hand else {
        return None;
    };
    let suits_on_board = board.iter().map(|c| c.suit).collect::<Vec<Suit>>();
    let mut map_suits_on_board = HashMap::new();
    suits_on_board.iter().for_each(|&suit| {
        let v = map_suits_on_board.entry(suit).or_insert(0u8);
        *v += 1;
    });
    let flash_suit = map_suits_on_board
        .iter()
        .find(|(_, &v)| v >= 3)
        .map(|(&k, _)| k)
        .unwrap_or_else(|| unreachable!());
    let all_ranks_from_low_to_top = Rank::to_vec_from_low();
    let ranks_hand = hand
        .cards
        .iter()
        .filter(|&&card| card.suit == flash_suit)
        .map(|&card| card.rank)
        .collect::<Vec<Rank>>();
    let ranks_board = board
        .iter()
        .filter(|&&card| card.suit == flash_suit)
        .map(|&card| card.rank)
        .collect::<Vec<Rank>>();
    let search_rank_in_hand_from_low = all_ranks_from_low_to_top
        .iter()
        .filter(|&&r| !ranks_board.contains(&r))
        .map(|&r| r)
        .collect::<Vec<Rank>>();
    let nut_rank = *search_rank_in_hand_from_low
        .last()
        .unwrap_or_else(|| unreachable!());
    if ranks_hand.contains(&nut_rank) {
        return Some(FakePostReadyHand::NutFlash);
    }
    let secondthree_rank = &search_rank_in_hand_from_low
        [search_rank_in_hand_from_low.len() - 2..=search_rank_in_hand_from_low.len() - 2];
    if !ranks_hand.iter().all(|&r| !secondthree_rank.contains(&r)) {
        return Some(FakePostReadyHand::SecondThreeFlash);
    }
    Some(FakePostReadyHand::LowFlash)
}
fn is_nut_street(ready_hand: ReadyHand, board: &Vec<Card>) -> bool {
    /* Логика:
    - Знаю карту Ка, с которой начинается стрит.
    - Если эта карта Т (дискр. 8), то это всегда натс
    - Иначе берутся все возможные тройки борда. Отбираются только те тройки, которые могут дать стрит, то
        есть без спарки.
        Проверяются все минимальные карты этих троек и если находится хоть одна, которая больше Ка, значит
        стрит ненатсовый.
    - Особый случай только с одним стритом А2345
     */
    let low_card_street_disc = match ready_hand {
        ReadyHand::Street(r) if r >= Rank::Ace => return true,
        ReadyHand::Street(Rank::Five) => 12,
        ReadyHand::Street(r) => r as isize - 4,
        _ => return false,
    };

    let ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    let mut trips_cards_from_board = vec![];
    for i in 0..ranks.len() {
        for j in i + 1..ranks.len() {
            for k in j + 1..ranks.len() {
                let v = vec![ranks[i], ranks[j], ranks[k]];
                trips_cards_from_board.push(v);
            }
        }
    }
    for v in trips_cards_from_board.iter() {
        let mut set = HashSet::new();
        if !v.iter().all(|&element| set.insert(element)) {
            // Обязательно исключить тройки со спаркой, в них нет стрита никогда.
            continue;
        }
        if (v[0] as isize - v[2] as isize) > 4 {
            // Это не стритовая тройка по рангу(дискриминанту), слишком далекие карты.
            // Здесь стритовые тройки на лоу стрит считаются нестритовыми, т.к. это минимальный стрит
            // И если есть любой другой то он не натс, а если нет, то натс
            continue;
        }
        // Сюда дойдет если есть хоть одна стритовая тройка, НО НЕ НА СТРИТ ОТ ТУЗА.
        // А значит в этот момент всегда стрит от туза автоматически становится ненатсовым.
        if v[2] as isize > low_card_street_disc || low_card_street_disc == 12 {
            return false;
        }
    }
    // Сюда дойдут только реально натсовые стриты и стриты А до 5
    true
}
fn is_no_nut_street(ready_hand: ReadyHand) -> bool {
    if let ReadyHand::Street(_) = ready_hand {
        true
    } else {
        false
    }
}
fn is_set(ready_hand: ReadyHand, map: &HashMap<Rank, u8>) -> bool {
    /* Логика:
    - ready_hand это трипс
    - борд обязательно не содержит пару или другое повторение.
    - так как наиболее сильный вариант фейковой руки - фулл хаус уже отфильтрован ранее, то
        этих условий достаточно.
     */
    let max_val = *map.values().max().unwrap_or_else(|| unreachable!());
    match ready_hand {
        ReadyHand::Trips {
            trips: _,
            top_kicker: _,
            low_kicker: _,
        } if max_val == 1 => true,
        _ => false,
    }
}
fn is_nut_trips(ready_hand: ReadyHand, map: &HashMap<Rank, u8>, hand: &Hand) -> bool {
    /* Логика:
    - ready_hand это трипс
    - обязательно спаренная доска или дабл спаренная,
        если на доске лежит трипс то это эскалируется даже не в ненатсовый трипс а хуже
    - натс трипс всегда на максимальной спарке
    - УПРОЩЕНИЕ: Пусть если доска с АА, то рука должна содержать К, иначе А
        т.е. AAK22 (AQ33) и A7722 (K733) не являются натсовым трипсом
    */
    let ReadyHand::Trips { .. } = ready_hand else {
        return false;
    };
    let max_val = *map.values().max().unwrap_or_else(|| unreachable!());
    if max_val != 2 {
        return false;
    }
    let pairs = map
        .iter()
        .filter_map(|(&k, &v)| if v == 2 { Some(k) } else { None })
        .collect::<Vec<_>>();
    let max_pair = *pairs.iter().max().unwrap();
    let hand_ranks = hand.cards.iter().map(|c| c.rank).collect::<Vec<_>>();
    match ready_hand {
        ReadyHand::Trips { trips, .. }
            if trips == Rank::Ace && hand_ranks.contains(&Rank::King) =>
        {
            true
        }
        ReadyHand::Trips { trips, .. }
            if trips != Rank::Ace && trips == max_pair && hand_ranks.contains(&Rank::Ace) =>
        {
            true
        }
        _ => false,
    }
}
fn is_nonut_trips(ready_hand: ReadyHand, map: &HashMap<Rank, u8>) -> bool {
    /* Логика:
    - ready_hand это трипс
    - никаких условий больше не нужно, так как сет, натсовый стрипс уже отфильтрованы,
        остались только ненатсовые трипсы
    - Минуточку:) Если борд содержит больше чем спарку, то это отправляется в треш руки(АК** на ТТТ22)
    */
    let max_val = *map.values().max().unwrap_or_else(|| unreachable!());
    if let ReadyHand::Trips {
        trips: _,
        top_kicker: _,
        low_kicker: _,
    } = ready_hand
    {
        if max_val == 2 {
            return true;
        }
    }
    false
}
fn is_top_two(ready_hand: ReadyHand, board: &Vec<Card>, map: &HashMap<Rank, u8>) -> bool {
    /* Логика, почти идеальная точность, кроме одного борда (AAKKx):
    - ready_hand это две пары
    - борд неспаренный, ранги двух пар равны рангам двух максимальных карт борда
    - борд спаренный один раз,
        тогда ранг топпары всегда А, а ранг второй пары равен либо К (если борд со спаркой А),
        либо ранг второй пары это спарка флопа (если борд без спарки А, а в руке АА)
    - борд спаренный два раза, работает ровно также как односпаренный, но есть только одно
        исключение ААККх-QQ:

    AAK92 tp-A-K, AAKK9 !tp-A-Q, AAQQ9 tp-A-K, KKQ92 tp-A-K, AK992 tp-A-K, 55443 tp-A-5, 322 tp-A-3
     */
    let ReadyHand::TwoPair {
        top,
        bottom,
        kicker: _,
    } = ready_hand
    else {
        return false;
    };
    let max_val = *map.values().max().unwrap_or_else(|| unreachable!());
    let ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    /* max_rank_except_ace всегда существует, потомучто иначе не выполнится первый гард с
    проверкой реальной комбинации. Потомучто единственно возможный случай это флоп AAA(реальная
    комба трипс) или терн АААА(реальная комба каре)
     */
    let paired_ranks = map
        .iter()
        .filter(|(_, &v)| v == 2)
        .map(|(&k, _)| k)
        .collect::<Vec<Rank>>();
    if max_val == 1 && top == ranks[0] && bottom == ranks[1] {
        true
    } else if max_val == 2
        && top == Rank::Ace
        && (bottom == Rank::King || paired_ranks.contains(&bottom))
    {
        true
    } else {
        false
    }
}
fn is_top_bottom(ready_hand: ReadyHand, board: &Vec<Card>, map: &HashMap<Rank, u8>) -> bool {
    /* Логика(не совсем точная, но эта неточность не самая важная из-за высокой вероятности трипса):
    - ready_hand это две пары
    - борд неспаренный, ранг одной пары должен быть максимальным на борде
    - борд спаренный или двуспаренный, но не односиься к топ-2 парам, хрен сним, пусть всегда будет
        не в этой категории, а в категории две младшие пары, т.к. это слабая комба из-за вероятности
        трипса у соперника в случае экшена
     */
    let ReadyHand::TwoPair { top, .. } = ready_hand else {
        return false;
    };
    let max_val = *map.values().max().unwrap_or_else(|| unreachable!());
    let ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    if max_val == 1 && top == ranks[0] {
        true
    } else {
        false
    }
}
fn is_bottom_bottom(ready_hand: ReadyHand) -> bool {
    /* Логика:
    - ready_hand это две пары
    - никаких условий больше не нужно, так как все остальные варианты двух пар уже отфильтрованы
    */
    if let ReadyHand::TwoPair { .. } = ready_hand {
        true
    } else {
        false
    }
}
fn is_toppair_overpair(ready_hand: ReadyHand, board: &Vec<Card>) -> bool {
    /* Логика:
    - ready_hand это пара
    - ранг пары больше(OP) или равен(TP) максимальной карте борда
    - в случае спаренного борда все также, это не важно, т.к. это учитывается в фейковой ситуации
     */
    let ReadyHand::OnePair { pair, .. } = ready_hand else {
        return false;
    };
    let ranks = board.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    if pair >= ranks[0] {
        true
    } else {
        false
    }
}
fn wrap_combination(
    five_cards: &Vec<Card>,
    config_board: &ConfigBoard,
    was_nonut_wrap: bool,
) -> FakePostflopSD {
    /* Логика:
    -1. was_nonut_wrap - это флаг, который означает что для какой-то 5-ки карт ранее уже найлен
        возможный врап, а значит искать для последующих 5-к oesd нет смысла
    0. Предыдущий шаг гарантирует, что нет стрита реального.
    0. Все ранги должны быть уникальные, иначе no wrap but could be oesd
    1. Дискриминанты карты максимума и карты минимума отличаются на 4, это врап
    - Если в пятерке есть А, то это натс врап всегда, неважно в руке или на борде
    - Если младшая карта принадлежит борду, то это натс врап.
    2. Исключение нижний врап, проверяется, что все карты в дипе (А, 5, 4, 3, 2), это врап
    - Если А принадлежит борду, то это натс врап
    3 Особый случай OESD, если рап не найден:
    - Все ранги четверки должны быть уникальные, иначе Nothing.
    - Получить все четверки карт из пятерки карт и если их дискриминанты отличаются на 3, то это oesd
    - Исключение если в четверке есть А, то это не oesd, не важно лоу или хай
     */
    let mut ranks = five_cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();
    let mut set = HashSet::new();
    let wrap_possible = ranks.iter().all(|&r| set.insert(r));
    ranks.sort_unstable_by(|a, b| b.cmp(a));
    if wrap_possible {
        let all_five_cards_low = ranks
            .iter()
            .all(|&r| vec![Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two].contains(&r));
        let delta_rank = ranks[0] as isize - ranks[4] as isize;
        // Wraps conditions.
        if all_five_cards_low && config_board.board_ranks_contains_ace {
            return FakePostflopSD::NutWrap;
        } else if all_five_cards_low && !config_board.board_ranks_contains_ace {
            return FakePostflopSD::NoNutWrap;
        } else if !all_five_cards_low
            && delta_rank == 4
            && (ranks.contains(&Rank::Ace) || config_board.board_ranks.contains(&ranks[4]))
        {
            return FakePostflopSD::NutWrap;
        } else if !all_five_cards_low && delta_rank == 4 {
            return FakePostflopSD::NoNutWrap;
        }
    }
    // Oesd conditions.
    if was_nonut_wrap {
        return FakePostflopSD::Nothing;
    }
    // "oesd_possible" Board must have exactly two cards of oesd and can't be paired.

    // 1st stage. 76542 -> 7654*, 76544 -> 7654*
    let oesd_ranks = &ranks[0..ranks.len() - 1];
    let delta_rank = oesd_ranks[0] as isize - oesd_ranks[3] as isize;
    let mut set = HashSet::new();
    let oesd_possible = oesd_ranks.iter().all(|&r| set.insert(r))
        && (oesd_ranks.iter().fold(0, |acc, r| {
            if config_board.board_ranks.contains(r) {
                acc + 1
            } else {
                acc
            }
        }) == 2);
    if oesd_possible && !oesd_ranks.contains(&Rank::Ace) && delta_rank == 3 {
        //println!("1st {:?}->{:?}",ranks, oesd_ranks);
        return FakePostflopSD::Oesd;
    }
    // 2nd stage. AJT98 -> *JT98
    let oesd_ranks = &ranks[1..ranks.len()];
    let delta_rank = oesd_ranks[0] as isize - oesd_ranks[3] as isize;
    let mut set = HashSet::new();
    let oesd_possible = oesd_ranks.iter().all(|&r| set.insert(r))
        && (oesd_ranks.iter().fold(0, |acc, r| {
            if config_board.board_ranks.contains(r) {
                acc + 1
            } else {
                acc
            }
        }) == 2);
    if oesd_possible && !oesd_ranks.contains(&Rank::Ace) && delta_rank == 3 {
        //println!("2nd {:?}->{:?}",ranks, oesd_ranks);
        return FakePostflopSD::Oesd;
    }
    FakePostflopSD::Nothing
}
