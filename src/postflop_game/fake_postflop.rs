use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::sync::LockResult;

use crate::eval_hand::real_comb;
use crate::postflop_game::{eval_fake_hand, flop};
use crate::{action, inline::fakeboard, Card, Game, Player, Position, Rank};

use crate::PostflopGame;

use super::eval_fake_hand::{fake_comb_side_fd, fake_comb_side_ready, fake_comb_side_sd};

// Hand:
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakePostReadyHand {
    // 15
    Nothing, // One case trips or care on board and have no pair.It's trash hand, so its here
    TPOP,
    BottomBottom,
    TopBottom, // Супер точно, но если борд спаренные и это не Топ-2, то эскалация руки вниз
    // Эта эскалация супер норм, т.к. все равно при экшене по человечески вероятен трипс
    // -> BottomBottom
    TopTwo, // SUPER RARE ERROR, почти идеальная точность, на бордах любой спаренности
    // кроме одного борда AAKKx (QQ)->bottom-bottom.
    // Также если на борде лежит трипс, то это никогда ни какие две пары
    TripsLess,
    TripsNutKicker, //Simplyfied продумано!: nuts trips only K77xx (A7) or AA772 (AK), else no nut trips
    //т.е. AAK22 (AQ33) и A7722 (K733) не являются натсовым трипсом. Но таких
    //исключений мало - только структуры борда A+pair, AA+K, причем в первом
    //случае это правильно потомучто соперники часто играют руки с А а
    //второй случай просто редок
    // -> -> TripsLess
    Set,
    NoNutStreet,
    NutStreet,
    LowFlash,
    SecondThreeFlash, // Flash from K here as J-flash. It's tight but tight is right
    NutFlash,
    LowFullHouse, // One case nut full house here board JJJ82-AA22, 777A2-AA22. When trips(not care!)
    // on board. Because if agro then hightlikely care. Its not ultimate strong, but still strong
    Imba, // Nuts full here, care, street flash, flash roal.
          // Nut full when board with trips not this group(high likely care when action => not imba)
          // SUPER RARE ERROR TTTT2 = KQ33 is here
          // -> LowFullHouse
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakePostflopFD {
    // 7 (5 on flop, 6 on turn, 1 on river)
    Nothing,
    TwoBD, // only on flop
    Low,
    OneSecondThree,
    OneNutFD,
    TwoFD,        // only turn
    TwoFdWithNut, // only turn
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakePostflopSD {
    // 4 flop, 4 turn, 1 river
    Nothing, // Gut shot is here
    Oesd,
    NoNutWrap, // first candidate to delete
    NutWrap,
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub struct FakePostflopHand {
    // flop 15*5*4 = 300, turn 15*6*4 = 360, river 15*1*1 = 15
    pub ready: FakePostReadyHand,
    pub flash_draw: FakePostflopFD,
    pub street_draw: FakePostflopSD,
}
impl Debug for FakePostflopHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "r: {:?} fd: {:?} sd: {:?}",
            self.ready, self.flash_draw, self.street_draw
        )
    }
}
impl Display for FakePostflopHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?},{:?},{:?})",
            self.ready, self.flash_draw, self.street_draw
        )
    }
}

// Situation
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum RatioNeedCoomitToPotPercent {
    Less35p, // enemy bet 1/2 pot or less
    More35p, // enemy bet 75% pot or more or cold call re-raise
}
impl RatioNeedCoomitToPotPercent {
    pub fn from(value: Decimal) -> Self {
        match value {
            v if v < dec!(35) => Self::Less35p, // enemy bet 1/2 pot or less
            _ => Self::More35p,                 // enemy bet 75% pot or more
        }
    }
}
impl Debug for RatioNeedCoomitToPotPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Less35p => format!("Odds good <35%"),
                Self::More35p => format!("Odds bad >=35%)"),
            }
        )
    }
}
impl Display for RatioNeedCoomitToPotPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Less35p => format!("Odds good <35%"),
                Self::More35p => format!("Odds bad >=35%)"),
            }
        )
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum FakeSpr {
    Deep, // > 1. Deep stack open raise pot
    Low,  // < 1. 100 stack, bound: pot after preflop 25 and faced bet pot
}
impl FakeSpr {
    pub fn from(spr_stack: Decimal, start_pot: Decimal) -> Self {
        match spr_stack / start_pot {
            c if c >= dec!(0) && c < dec!(1.) => Self::Low,
            c if c >= dec!(1.) => Self::Deep,
            _ => unreachable!(),
        }
    }
}
impl Debug for FakeSpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Deep => ">1 hight",
                Self::Low => "<1 low",
            }
        )
    }
}
impl Display for FakeSpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Deep => ">1_hight",
                Self::Low => "<1_low",
            }
        )
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct FakeSituation {
    /* 4*3*2*2*2 = 96
    ШАНСЫ БАНКА. КАКИЕ У МЕНЯ ШАНСЫ НА КОЛЛ.
    Характеризует и подобран хорошо для ситуаций, в которых в меня летит один бет. Тогда
    примерно по сайзингам (no, 0-1/3, 1/3-2/3, 2/3-pot).
    В случае если появляются коллы, то шансы улучшаются и это логично.
    Такая оценка оддсов уже сбоит при рирейз флопа. Там будет иногда средний диапп, но это
    вообще не страшно, т.к. ситуации с рирейзами фильтруются от обычных за счет rised_pot,
    там нейронка построит свою логику и она будет норм скорее всего, не буду ей помогать.
    */
    pub odds: RatioNeedCoomitToPotPercent,
    /* СЛОЖНЕЕ НО ВКРАТЦЕ ХАРАКТЕРИЗУЕТ ГЛУБИНУ СТЕКА ОТНОСИТЕЛЬНО ПОТА.
    Считается относительно всех стеков, а не только своего.
    Логика похожа на calc_playing_stack префлопа*/
    pub spr: FakeSpr,
    /* Мелкие но важные фишки, тащить ли мелкое фд(hu), флоатить ли(ip_cmp),
    насколько много натсов у соперника(raised_pot) */
    pub hu: bool,
    pub ip_cmp: bool,
    pub raised: bool,
}
impl Debug for FakeSituation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pot odds: {:?}, spr(adop): {:?}, raised: {}, hu: {}, ip: {}",
            self.odds, self.spr, self.raised, self.hu, self.ip_cmp
        )
    }
}
impl Display for FakeSituation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{},{},{})",
            self.odds, self.spr, self.raised, self.hu, self.ip_cmp
        )
    }
}

// Board struct
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakeSuitPostFlop {
    Flash,        // flop + turn + river
    TwoFlashDraw, //        turn
    OneFlashDraw, // flop + turn
    Rainbow,      // flop + turn
    NoFlashRiver, //               river
}
impl Debug for FakeSuitPostFlop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Flash => "flash",
                Self::TwoFlashDraw => "2fd",
                Self::OneFlashDraw => "1fd",
                Self::Rainbow => "rainbow",
                Self::NoFlashRiver => "noflash(river)",
            }
        )
    }
}
impl FakeSuitPostFlop {
    pub fn from_str(s: &str) -> Self {
        match s {
            "flash" => Self::Flash,
            "noflash(river)" => Self::NoFlashRiver,
            _ => unreachable!(),
        }
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakeStreet {
    // Количество дырок. Равно (max-min)-1
    Street,        // flop + turn + river
    Drawly,        // flop + turn
    Dry,           // flop + turn
    NoStreetRiver, //               river
}
impl Debug for FakeStreet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Street => "Street",
                Self::Drawly => "Drawly",
                Self::Dry => "Dry",
                Self::NoStreetRiver => "Nostreet(river)",
            }
        )
    }
}
impl FakeStreet {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Street" => Self::Street,
            "Drawly" => Self::Drawly,
            "Dry" => Self::Dry,
            "Nostreet(river)" => Self::NoStreetRiver,
            _ => unreachable!(),
        }
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakeBoardStruct {
    // x - any rank, B - Ten+
    X,
    BB,
    A,
    AB,
}
impl Debug for FakeBoardStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "*",
                Self::BB => "BB",
                Self::A => "A",
                Self::AB => "AB",
            }
        )
    }
}
impl FakeBoardStruct {
    pub fn from_str(s: &str) -> Self {
        match s {
            "*" => Self::X,
            "BB" => Self::BB,
            "A" => Self::A,
            "AB" => Self::AB,
            _ => unreachable!(),
        }
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub struct FakeBoard {
    // 3*3*2*4 = 72
    pub suit_kind: FakeSuitPostFlop,
    pub street_kind: FakeStreet,
    pub paired: bool,
    pub rank_struct: FakeBoardStruct,
}
impl Debug for FakeBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "suit: {:?}; street: {:?}; paired: {}; struct {:?}",
            self.suit_kind, self.street_kind, self.paired, self.rank_struct
        )
    }
}
impl Display for FakeBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?},{:?},{},{:?})",
            self.suit_kind, self.street_kind, self.paired, self.rank_struct
        )
    }
}

// Postflop pause on position
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct FakePostflopPause {
    //300 * 96 * 72 = 2 073 600
    pub my_fake_hand: FakePostflopHand,
    pub fake_board: FakeBoardNew,
    pub situation: FakeSituationNew,
    pub ch_board_str: bool,
    pub prev_agr: AgroStreet,
}
impl Debug for FakePostflopPause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "
***Hand {:?}
***Board: {:?}
***Situation: {:?}
***Changed str: {:?}
***Agr prev str: {:?}
",
            self.my_fake_hand, self.fake_board, self.situation, self.ch_board_str, self.prev_agr
        );
        write!(f, "{}", s)
    }
}
impl FakePostflopPause {
    pub fn from(game: &PostflopGame, position: Position) -> FakePostflopPause {
        // panic!();
        let player = game.player_by_position_as_ref(position);
        let real_comb = real_comb(&player.hand, &game.cards);
        let fake_board = Utils::new_fake_flop_board(game);
        let prev_fake_board = Utils::new_fake_flop_board(&game);
        let ch_board_str = fake_board != prev_fake_board;
        /* Я не могу это посчитать, кто был агрессором. Это данные исключительно динамики раздачи, а стейт игры статичен
        Поэтому здесь всегда стоит, что не было агрессора.
        !!! НИКОГДА БОЛЬШЕ НЕ ИСПОЛЬЗОВАТЬ ЭТУ ФУНЦИЮ БЕЗ РУЧНОГО ЗАПОЛНЕНИЯ ЭТОГО СВОЙСТВА !!! from_parts()*/
        let prev_agr = AgroStreet::NoOne;
        Self {
            my_fake_hand: FakePostflopHand {
                ready: fake_comb_side_ready(&player.hand, real_comb, &game.cards),
                flash_draw: fake_comb_side_fd(&player.hand, real_comb, &game.cards),
                street_draw: fake_comb_side_sd(&player.hand, real_comb, &game.cards),
            },
            fake_board,
            situation: Utils::postflop_situation(game, &player),
            ch_board_str,
            prev_agr,
        }
    }
    pub fn from_parts(
        my_fake_hand: FakePostflopHand,
        fake_board: FakeBoardNew,
        situation: FakeSituationNew,
        ch_board_str: bool,
        prev_agr: AgroStreet,
    ) -> FakePostflopPause {
        Self {
            my_fake_hand,
            fake_board,
            situation,
            ch_board_str,
            prev_agr,
        }
    }
    pub fn mock() -> Self {
        Self {
            my_fake_hand: FakePostflopHand {
                ready: FakePostReadyHand::BottomBottom,
                flash_draw: FakePostflopFD::Low,
                street_draw: FakePostflopSD::Nothing,
            },
            fake_board: FakeBoardNew::NoSpecial,
            situation: FakeSituationNew {
                action: FakeActionNew::StartOpp,
                spr: FakeSpr::Deep,
            },
            ch_board_str: false,
            prev_agr: AgroStreet::NoOne,
        }
    }
}
pub struct Utils;
impl Utils {
    // board
    pub fn fake_flop_board(game: &PostflopGame) -> FakeBoard {
        FakeBoard {
            suit_kind: Self::suit_kind_board(game),
            street_kind: Self::street_kind_board(&game.cards),
            paired: Self::paired(game),
            rank_struct: Self::fake_board_struct(game),
        }
    }
    pub fn new_fake_flop_board(game: &PostflopGame) -> FakeBoardNew {
        let suit_kind = Self::suit_kind_board(game);
        let street_kind = Self::street_kind_board(&game.cards);
        let paired = Self::paired(game);
        match (paired, suit_kind, street_kind) {
            (true, _, _) => FakeBoardNew::Pair,
            (false, FakeSuitPostFlop::Flash, _) => FakeBoardNew::FlashNoPair,
            (false, _, FakeStreet::Street) => FakeBoardNew::StreetNoFlashNoPair,
            _ => FakeBoardNew::NoSpecial,
        }
    }
    pub fn fake_flop_board_inline(
        game: &PostflopGame,
        map_ranks: &BTreeMap<String, FakeBoard>,
        map_suits: &BTreeMap<String, FakeBoard>,
    ) -> FakeBoard {
        fakeboard::fake_board_from_inline(map_ranks, map_suits, &game.cards)
    }
    fn fake_board_struct(game: &PostflopGame) -> FakeBoardStruct {
        /* УНИВЕРСАЛЬНАЯ ДЛЯ флопа, терна, ривера
        На ривере на самом деле уже не важны ранги карт, это очень важно на флопе, средневажно на терне
        Поэтому не плодим фейки ривера
        */
        if game.cards.len() == 5 {
            return FakeBoardStruct::X;
        }
        let ranks = game.cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();
        let has_ace = ranks.iter().find(|&&r| r == Rank::Ace).is_some();
        let count_ten_to_king = ranks.iter().fold(0, |acc, &x| {
            if vec![Rank::Ten, Rank::Jack, Rank::Queen, Rank::King].contains(&x) {
                acc + 1
            } else {
                acc
            }
        });
        match (has_ace, count_ten_to_king) {
            (true, 0) => FakeBoardStruct::A,
            (true, _) => FakeBoardStruct::AB,
            (false, v) if v >= 2 => FakeBoardStruct::BB,
            _ => FakeBoardStruct::X,
        }
    }
    fn paired(game: &PostflopGame) -> bool {
        /* УНИВЕРСАЛЬНАЯ ДЛЯ флопа, терна, ривера
        - Спаренный борд приравнивается к трипсовому и карешному для уменьшения кол-ва вариаций.
         */
        let ranks = game.cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();

        // Спаренный борд или трипсовый или карешный
        let mut map = HashMap::new();
        ranks.iter().for_each(|&rank| {
            let v = map.entry(rank).or_insert(0u8);
            *v += 1;
        });
        let max_val = *map.values().max().unwrap_or_else(|| unreachable!());
        max_val >= 2
    }
    fn street_kind_board(cards: &Vec<Card>) -> FakeStreet {
        /*  УНИВЕРСАЛЬНАЯ ДЛЯ флопа, терна, ривера
        При создании борда карты уже отсортированы от большего к меньшему.
        Количество дырок. Равно (max-min)-1
        Особый случай, когда А, потомучто он дополняет стриты и сверху и снизу.
        На ривере может быть только NoStreetRiver или Street
         */
        let ranks = cards.iter().map(|c| c.rank).collect::<Vec<Rank>>();

        /* Основной случай, самый частый. Берем все возможные сочетания из трех карт борда
        и считаем дырки.
        - Если хоть в одной тройке есть спарка, то нужно игнорировать эту тройку, потомучто она точно без стрита
        - Если хоть в одной тройке есть стрит, то сразу выход из процедуры
        - Если НИ В ОДНОЙ тройке нет стрита, но в какой-то был drawly, то дровяной
        иначе сухой.(исключение ривер, там Drawly + Dry = NoStreetRiver)
        Вырожденный случай - флоп, просто одно сочетание*/
        let mut drawly = false;
        for i in 0..ranks.len() {
            for j in i + 1..ranks.len() {
                for k in j + 1..ranks.len() {
                    let v = vec![ranks[i], ranks[j], ranks[k]];
                    //println!("{:?}", v);
                    // Обязательно исключить тройки со спаркой, в них нет стрита и стритдро никогда.
                    let mut set = HashSet::new();
                    if !v.iter().all(|&element| set.insert(element)) {
                        continue;
                    }

                    let first_discriminant = *v.first().unwrap() as isize;
                    let last_discriminant = *v.last().unwrap() as isize;
                    let gap_counts = first_discriminant - last_discriminant - 2;
                    match gap_counts {
                        v if v >= 0 && v <= 2 => {
                            return FakeStreet::Street;
                        }
                        v if v >= 3 && v <= 4 => {
                            drawly = true;
                        }
                        _ => {}
                    }
                    // Особый случай стрита и стритдро с тузом снизу.
                    let v1 = vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five];
                    let v2 = vec![
                        Rank::Ace,
                        Rank::Two,
                        Rank::Three,
                        Rank::Four,
                        Rank::Five,
                        Rank::Six,
                    ];
                    let low_street = v.iter().all(|rank| v1.contains(rank));
                    let low_sd_a_lot = v.iter().all(|rank| v2.contains(rank));
                    if v[0] == Rank::Ace && low_street {
                        return FakeStreet::Street;
                    } else if v[0] == Rank::Ace && low_sd_a_lot {
                        drawly = true;
                    }
                }
            }
        }
        if ranks.len() == 5 {
            FakeStreet::NoStreetRiver
        } else if drawly {
            FakeStreet::Drawly
        } else {
            FakeStreet::Dry
        }
    }
    fn suit_kind_board(game: &PostflopGame) -> FakeSuitPostFlop {
        /* УНИВЕРСАЛЬНАЯ ДЛЯ флопа, терна, ривера
        На ривере если нет флеша на борде, просто возвращаем NoFlashRiver
         */
        let mut map = HashMap::new();
        game.cards.iter().for_each(|&card| {
            let v = map.entry(card.suit).or_insert(0u8);
            *v += 1;
        });
        let max_val = *map.values().max().unwrap_or_else(|| unreachable!());
        if game.cards.len() == 5 && max_val < 3 {
            return FakeSuitPostFlop::NoFlashRiver;
        }
        let max_val_count = map
            .iter()
            .fold(0, |acc, (_, &v)| if v == max_val { acc + 1 } else { acc });
        match max_val {
            v if v == 1 => FakeSuitPostFlop::Rainbow,
            v if v == 2 && max_val_count == 1 => FakeSuitPostFlop::OneFlashDraw,
            v if v == 2 && max_val_count == 2 => FakeSuitPostFlop::TwoFlashDraw,
            v if v >= 3 => FakeSuitPostFlop::Flash,
            _ => unreachable!(),
        }
    }
    // situation
    pub fn flop_situation(game: &PostflopGame, player: &Player) -> FakeSituation {
        let raised_pot = Self::raised_pot(game, player.position);
        FakeSituation {
            odds: Self::odds(game, player.position),
            spr: Self::spr(game, player.position, player.stack_size),
            hu: Self::hu(game),
            ip_cmp: Self::ip_cmp(game, player.position),
            raised: raised_pot,
        }
    }
    // new situation
    pub fn postflop_situation(game: &PostflopGame, player: &Player) -> FakeSituationNew {
        let action = Self::new_action(game, player.position);
        let spr = Self::new_spr(game, player.position, player.stack_size);
        // let board = Self::new_fake_flop_board(game);
        FakeSituationNew { action, spr }
    }
    pub fn we_have_blockers(
        player_cards: &[Card],
        fake_board: &FakeBoardNew,
        game: &PostflopGame,
    ) -> bool {
        match fake_board {
            FakeBoardNew::FlashNoPair => {
                let flash_blocker = game.flash_blockers_to_board();
                if let Some(card) = flash_blocker {
                    player_cards.contains(&card)
                } else {
                    false
                }
            }
            FakeBoardNew::StreetNoFlashNoPair => {
                // Длина вектора всегда 2.
                let street_blockers = game.street_blockers_to_board();
                if let Some(v) = street_blockers {
                    let count1 =
                        player_cards.iter().fold(
                            0,
                            |acc, &x| {
                                if x.rank == v[0] {
                                    acc + 1
                                } else {
                                    acc
                                }
                            },
                        );
                    let count2 =
                        player_cards.iter().fold(
                            0,
                            |acc, &x| {
                                if x.rank == v[1] {
                                    acc + 1
                                } else {
                                    acc
                                }
                            },
                        );
                    if count1 >= 2 || count2 >= 2 {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    fn new_action(game: &PostflopGame, position: Position) -> FakeActionNew {
        let raised_pot = Self::raised_pot(game, position);
        let odds = Self::odds(game, position);
        let my_commit = crate::already_commit_by_pos(game, position);
        let no_money_in_game = game.no_money_in_game();
        let ip = Self::ip_cmp(game, position);
        match (no_money_in_game, odds, raised_pot, my_commit, ip) {
            (true, _, _, _, true) => FakeActionNew::StartIp,
            (true, _, _, _, false) => FakeActionNew::StartOpp,
            (false, RatioNeedCoomitToPotPercent::Less35p, false, val, true)
                if val == Decimal::ZERO =>
            {
                FakeActionNew::BetToMeGoodIp
            }
            (false, RatioNeedCoomitToPotPercent::Less35p, false, val, false)
                if val == Decimal::ZERO =>
            {
                FakeActionNew::BetToMeGoodOop
            }
            (false, RatioNeedCoomitToPotPercent::More35p, false, val, true)
                if val == Decimal::ZERO =>
            {
                FakeActionNew::BetToMeBadIp
            }
            (false, RatioNeedCoomitToPotPercent::More35p, false, val, false)
                if val == Decimal::ZERO =>
            {
                FakeActionNew::BetToMeBadOop
            }
            (false, _, true, _, _) => FakeActionNew::Raise,
            _ => unreachable!(),
        }
    }
    fn ip_cmp(game: &PostflopGame, position: Position) -> bool {
        /* Логика:
        Берем только позиции, которые не в фолде, включая текущую.
        И из оставшихся все должны быть не больше текущей.
         */
        let ordered_list_by_action = Self::_position_order_list();
        ordered_list_by_action
            .iter()
            .filter(|&pos| !game.folded_positions.contains(pos))
            .all(|&pos| pos <= position)
    }
    fn hu(game: &PostflopGame) -> bool {
        game.folded_positions.len() >= 4
    }
    /* Эта реализация очень классная и точна для рассчета базы солверных решений.
    Но когда я буду это же рассчитывать после распознавания картинки, то мне очень не нравится,
    что здесь учитывается такой параметр как min_bet. Он не будет рассчитываться OpenCV,
    поэтому пишу альтернативную функцию raised_pot_opencv, которая не такая точная, но подойдет.*/
    #[allow(unused)]
    fn old_raised_pot(game: &PostflopGame, position: Position) -> bool {
        /* Логика:
        - Если я уже вносил деньги в пот и снова в точке решения, значит это точно рейз постфлоп пот
        - Если на постфлопе никто не вносил деньги в пот, то это не рейз постфлоп пот
        - Исключаем игроков в алинах меньше max_commit или внесших 0, потомучто это всегда callalin* или фолд/чек
        Если все взносы СТРОГО больше (max_commit-game.min_bet), то это не рейз пот.
        Почему именно больше а не = max_commit - если кто-то сыграл бет-алинколл(и это макс на текущий момент)

        * Есть небольшое исключение если идет r-alin, r-alin, то по этоиу алгоритму первый r-alinотбросится
        считая что это call-alin. Ето плохо, но если подумать, то это значит, что банк такой огромный или
        человек пошедший ralin настолько короткий, что можно это проигнорировать
         */
        let my_commit = action::already_commit_by_pos(game, position);
        if my_commit != Decimal::ZERO {
            return true;
        }
        let max_commit = action::max_current_commit_from_all(game);
        if max_commit == Decimal::ZERO {
            return false;
        }
        let was_not_raise = game
            .positions_and_money
            .iter()
            .filter(|(&pos, &money)| {
                !(money == Decimal::ZERO || (game.position_in_allin(pos) && (money < max_commit)))
            })
            .all(|(_, &money)| money > (max_commit - game.min_bet));
        !was_not_raise
    }
    #[allow(unused)]
    fn raised_pot(game: &PostflopGame, position: Position) -> bool {
        /* Логика:
        - Если я уже вносил деньги в пот и снова в точке решения, значит это точно рейз постфлоп пот
        - Если на постфлопе никто не вносил деньги в пот, то это не рейз постфлоп пот
        - Фильтруем позиции кроме моей:
            - Обычно самый скромный рейз постфлоп имеет х2 сайзинг. Поэтому, если среди действий
            есть money > 2*money, то это рейз постфлоп.

        1000 рук
        23 ошибки новый алгоритм 2.3%	4 = 0.4%
        17 ошибок старый алгоритм 1.7%	14 = 1.4%
        */
        let my_commit = action::already_commit_by_pos(game, position);
        if my_commit != Decimal::ZERO {
            return true;
        }
        let max_commit = action::max_current_commit_from_all(game);
        if max_commit == Decimal::ZERO {
            return false;
        }
        let (max_money_pose, max_money) = game
            .positions_and_money()
            .iter()
            .filter(|(_, &v)| v > dec!(0))
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(&k, &v)| (k, v))
            .expect("Error: opencv calc raise pot");
        // В стандарной логике max_by возвращается последний максимальный элемент. Нужен первый, поэтому добавляю find:
        let (max_money_pose, max_money) = game
            .positions_and_money()
            .iter()
            .find(|(&_, &v)| v == max_money)
            .map(|(&k, &v)| (k, v))
            .expect("Error: opencv calc raise pot");

        let (min_money_pose, min_money) = game
            .positions_and_money()
            .iter()
            .filter(|(_, &v)| v > dec!(0))
            // Исключаем игроков между максимальной ставкой и мной, если максимум до меня (ч-р)
            // Исключаем игроков после максимально ставки, если максимум после меня
            .filter(|(&k, _)| {
                (max_money_pose < position && (k <= max_money_pose || k > position))
                    || (max_money_pose > position && k <= max_money_pose)
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(&k, &v)| (k, v))
            .expect("Error: opencv calc raise pot");

        max_money >= dec!(2) * min_money
    }
    fn spr(game: &PostflopGame, position: Position, my_initial_stack: Decimal) -> FakeSpr {
        /* Идиальная логика. Подумай и никогда не меняй!!!
         */

        let my_money_in_pot_already = game
            .positions_and_money()
            .iter()
            .find(|(&k, &_)| k == position)
            .map(|(&k, &v)| v)
            .unwrap();

        // let my_money_in_pot_already = Decimal::ZERO;

        let mut biggest_postflop_stack_except_my = Decimal::ZERO;
        // Get max from others stacks, not in fold:
        for (&pos, _) in game
            .positions_and_money
            .iter()
            .filter(|(&k, _)| !game.folded_positions.contains(&k) && k != position)
        {
            let curr_player = game.player_by_position_as_ref(pos);
            if curr_player.stack_size > biggest_postflop_stack_except_my {
                biggest_postflop_stack_except_my = curr_player.stack_size
            }
        }
        if biggest_postflop_stack_except_my > my_initial_stack {
            FakeSpr::from(
                my_initial_stack - my_money_in_pot_already,
                game.main_pot.value,
            )
        } else {
            FakeSpr::from(
                biggest_postflop_stack_except_my - my_money_in_pot_already,
                game.main_pot.value,
            )
        }
    }
    fn new_spr(game: &PostflopGame, position: Position, my_initial_stack: Decimal) -> FakeSpr {
        /* Эффектиыный стек в начале улицы с учетом фолдов с потом на начале улицы.
        Упрощенный показатель глубины.
         */
        let mut biggest_postflop_stack_except_my = Decimal::ZERO;
        // Get max from others stacks, not in fold on start of the street:
        for (&pos, _) in game
            .positions_and_money
            .iter()
            .filter(|(&k, _)| !game.folded_positions.contains(&k) && k != position)
        {
            let curr_player = game.player_by_position_as_ref(pos);
            if curr_player.stack_size > biggest_postflop_stack_except_my {
                biggest_postflop_stack_except_my = curr_player.stack_size
            }
        }
        if biggest_postflop_stack_except_my > my_initial_stack {
            FakeSpr::from(my_initial_stack, game.main_pot.prev_street_end_size)
        } else {
            FakeSpr::from(
                biggest_postflop_stack_except_my,
                game.main_pot.prev_street_end_size,
            )
        }
    }
    fn odds(game: &PostflopGame, position: Position) -> RatioNeedCoomitToPotPercent {
        let player = game.player_by_position_as_ref(position);
        let my_commit = action::already_commit_by_pos(game, position);
        let max_commit = action::max_current_commit_from_all(game);
        let add_to_commit;
        if player.stack_size >= max_commit {
            add_to_commit = max_commit - my_commit;
        } else {
            add_to_commit = player.stack_size - my_commit;
        }
        let ratio = dec!(100) * add_to_commit / game.main_pot.value;
        RatioNeedCoomitToPotPercent::from(ratio)
    }
    fn _position_order_list() -> Vec<Position> {
        vec![
            Position::Sb,
            Position::Bb,
            Position::Utg,
            Position::Mp,
            Position::Co,
            Position::Btn,
        ]
    }
    fn potential_fe(
        player: &Player,
        fake_board: &FakeBoardNew,
        game: &PostflopGame,
    ) -> PotentialFE {
        let blockers = Self::we_have_blockers(&player.hand.cards, fake_board, game);
        PotentialFE {
            blockers,
            raise_vs_bet_hu_hspr: false,
            bet_vs_check_ip_hu_hspr: false,
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub struct PotentialFE {
    pub blockers: bool,
    pub raise_vs_bet_hu_hspr: bool,
    pub bet_vs_check_ip_hu_hspr: bool,
}
impl Debug for PotentialFE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "blockers: {}, raise vs bet hu hspr: {}, bet vs check ip hu hspr {}",
            self.blockers, self.raise_vs_bet_hu_hspr, self.bet_vs_check_ip_hu_hspr
        );
        write!(f, "{}", s)
    }
}
impl Display for PotentialFE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count: u8 = if self.blockers { 1 } else { 0 }
            + if self.raise_vs_bet_hu_hspr { 1 } else { 0 }
            + if self.bet_vs_check_ip_hu_hspr { 1 } else { 0 };
        write!(f, "FE{}", count * 20)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum FakeActionNew {
    StartIp,
    StartOpp,
    BetToMeGoodIp,
    BetToMeGoodOop,
    BetToMeBadIp,
    BetToMeBadOop,
    Raise,
}
impl Debug for FakeActionNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FakeActionNew::StartIp => "I start IP".to_string(),
            FakeActionNew::StartOpp => "I start OOP".to_string(),
            FakeActionNew::BetToMeGoodIp => "Btm g.ptr IP".to_string(),
            FakeActionNew::BetToMeGoodOop => "Btm g.ptr OOP".to_string(),
            FakeActionNew::BetToMeBadIp => "Btm b.ptr IP".to_string(),
            FakeActionNew::BetToMeBadOop => "Btm b.ptr OOP".to_string(),
            FakeActionNew::Raise => "Raise".to_string(),
        };
        write!(f, "{}", s)
    }
}
impl Display for FakeActionNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FakeActionNew::StartIp => "I_start_IP".to_string(),
            FakeActionNew::StartOpp => "I_start_OOP".to_string(),
            FakeActionNew::BetToMeGoodIp => "Btm_g.ptr_IP".to_string(),
            FakeActionNew::BetToMeGoodOop => "Btm_g.ptr_OOP".to_string(),
            FakeActionNew::BetToMeBadIp => "Btm_b.ptr_IP".to_string(),
            FakeActionNew::BetToMeBadOop => "Btm_b.ptr_OOP".to_string(),
            FakeActionNew::Raise => "Raise".to_string(),
        };
        write!(f, "{}", s)
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakeBoardNew {
    Pair,
    FlashNoPair,
    StreetNoFlashNoPair,
    NoSpecial,
}
impl Debug for FakeBoardNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FakeBoardNew::Pair => "Pair".to_string(),
            FakeBoardNew::FlashNoPair => "Flash_nP".to_string(),
            FakeBoardNew::StreetNoFlashNoPair => "Street_nF_nP".to_string(),
            FakeBoardNew::NoSpecial => "Common".to_string(),
        };
        write!(f, "{}", s)
    }
}
impl Display for FakeBoardNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FakeBoardNew::Pair => "P".to_string(),
            FakeBoardNew::FlashNoPair => "F_nP".to_string(),
            FakeBoardNew::StreetNoFlashNoPair => "S_nF_nP".to_string(),
            FakeBoardNew::NoSpecial => "Com".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct FakeSituationNew {
    pub action: FakeActionNew,
    /* Абсолютчно честный расчет spr. Не менять.
    pot 20 eff 60. Bet 20 eto granitsa seep/low. Pot 40, eff 60
    */
    /* На практике показывает абсурдный результат. Лучше изменю на новый алгоритм.
    Если пот в самом начале улицы больше или равен моему стеку вначале, то это low
     */
    pub spr: FakeSpr,
}
impl Debug for FakeSituationNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "action: {:?}, spr: {:?}", self.action, self.spr)
    }
}
impl Display for FakeSituationNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "action:_{},spr:_{}", self.action, self.spr)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum AgroStreet {
    Me,
    NotMe,
    NoOne,
}
impl Debug for AgroStreet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AgroStreet::Me => "me".to_string(),
            AgroStreet::NotMe => "notme".to_string(),
            AgroStreet::NoOne => "noone".to_string(),
        };
        write!(f, "{:?}", s)
    }
}
impl Display for AgroStreet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AgroStreet::Me => "me".to_string(),
            AgroStreet::NotMe => "notme".to_string(),
            AgroStreet::NoOne => "noone".to_string(),
        };
        write!(f, "agr:{}", s)
    }
}
impl AgroStreet {
    pub fn calculate(prev_agr_pose: &Option<Position>, my_pose: Position) -> Self {
        let Some(p) = prev_agr_pose else {
            return Self::NoOne;
        };
        if *p == my_pose {
            Self::Me
        } else {
            Self::NotMe
        }
    }
}

#[cfg(test)]
mod complex_algorythm_board_has_street {
    use super::*;
    use crate::Suit;
    #[test]
    fn A53_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn A63_no_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_ne!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AK52_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AKT_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn KT9_street() {
        let v = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn KT8_no_street() {
        let v = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_ne!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AKQ2_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn KKQQ_no_street() {
        let v = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Harts),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_ne!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn KKQQT_street() {
        let v = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Ten, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AQ952_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AQ962_no_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_ne!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AQ322_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Harts),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AQQ72_no_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Harts),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_ne!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AK975_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Harts),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
    #[test]
    fn AK943_street() {
        let v = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Harts),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Four, Suit::Harts),
            Card::new(Rank::Three, Suit::Clubs),
        ];
        let bord_by_street = Utils::street_kind_board(&v);
        assert_eq!(bord_by_street, FakeStreet::Street);
    }
}
