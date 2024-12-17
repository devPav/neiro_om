use crate::{action::*, Game, Hand};
use crate::{FakeHand, FakeStackSize, Position, PreflopGame};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::fmt::Debug;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakeAction {
    OpenRaise,
    ThreeBet,
    FourBetAndMore,
}
impl FakeAction {
    pub fn from(value: Decimal) -> Self {
        match value {
            v if v <= dec!(5.75) => Self::OpenRaise,
            v if v >= dec!(19.0) => Self::FourBetAndMore,
            _ => Self::ThreeBet,
        }
    }
}
impl Debug for FakeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OpenRaise => "OR".to_string(),
                Self::ThreeBet => "3bet".to_string(),
                Self::FourBetAndMore => "4bet+".to_string(),
            }
        )
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum FakePositionAction {
    Early,
    Late,
    BigB,
    SmallB,
}
impl FakePositionAction {
    pub fn from(position: Position) -> Self {
        match position {
            Position::Utg | Position::Mp => Self::Early,
            Position::Co | Position::Btn => Self::Late,
            Position::Bb => Self::BigB,
            Position::Sb => Self::SmallB,
        }
    }
}
impl Debug for FakePositionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Early => "EP/MP".to_string(),
                Self::Late => "CO/BTN".to_string(),
                Self::BigB => "BB".to_string(),
                Self::SmallB => "SB".to_string(),
            }
        )
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum RatioNeedCoomitToPot {
    Bad,  // Не вносил значимых(отличных от лимпа) денег в банк
    Good, // Уже внес денег в банк чуть больше лимпа => запазан в трибет точно в четыребет неточно.
}
impl RatioNeedCoomitToPot {
    pub fn from(value: Decimal) -> Self {
        match value {
            v if v <= dec!(1) => Self::Bad,
            _ => Self::Good,
        }
    }
}
impl Debug for RatioNeedCoomitToPot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bad => "<=1BB".to_string(),
                Self::Good => ">1BB".to_string(),
            }
        )
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct FakePreflopPause {
    // Наиболее важные оценки.
    pub biggest_action: FakeAction,
    pub agressor_position: FakePositionAction,
    pub my_position: Position,
    pub my_fake_hand: FakeHand,
    // Менее важные и точные оценки.
    pub my_ratio_commit: RatioNeedCoomitToPot, // Оценка мертвых денег в поте. Не уверен в точности. Тупо вносил ли бабки.
    pub calc_playing_stack: FakeStackSize, // Оценка эфективного стека в мультивее. Не уверен в точности. В ХА это эффективный стек.
}
impl FakePreflopPause {
    pub fn from(game: &PreflopGame, position: Position) -> Self {
        let player = game.player_by_position_as_ref(position);
        let (biggest_action, agressor_position) = Utils::first_biggest_action(&game, position);
        Self {
            biggest_action,
            agressor_position,
            my_position: position,
            my_fake_hand: FakeHand::from(&player.hand),
            my_ratio_commit: Utils::ratio_add_commit_to_pot(&game, position),
            calc_playing_stack: Utils::playing_stack(&game, position, player.stack_size),
        }
    }
    pub fn mock() -> Self {
        Self {
            biggest_action: FakeAction::ThreeBet,
            agressor_position: FakePositionAction::Early,
            my_position: Position::Sb,
            my_fake_hand: FakeHand::from(&Hand::rnd_hand(&vec![])),
            my_ratio_commit: RatioNeedCoomitToPot::Bad,
            calc_playing_stack: FakeStackSize::Deep,
        }
    }
}
impl Debug for FakePreflopPause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "***Action [{:?}] from [{:?}], 
***I'm in [{:?}], with hand:  {:?}
***Deep [{:?}], put in: {:?}",
            self.biggest_action,
            self.agressor_position,
            self.my_position,
            self.my_fake_hand,
            self.calc_playing_stack,
            self.my_ratio_commit
        );
        write!(f, "{}", s)
    }
}
struct Utils;
impl Utils {
    fn first_biggest_action(
        game: &PreflopGame,
        position: Position,
    ) -> (FakeAction, FakePositionAction) {
        let ordered_analize_pos = Self::position_order_list();
        let (mut max_value_pos, mut max_value) = (position, dec!(0));
        let mut i = 0u8; // Для позиционировании при обходе позиций, начиная с текущей.
        for &pos in ordered_analize_pos.iter().cycle() {
            if pos == position {
                i += 1
            }
            if i < 1 {
                continue;
            } else if i > 1 {
                break;
            }
            let val = *game
                .positions_and_money
                .get(&pos)
                .unwrap_or_else(|| unreachable!());
            if val > max_value {
                max_value = val;
                max_value_pos = pos;
            }
        }
        (
            FakeAction::from(max_value),
            FakePositionAction::from(max_value_pos),
        )
    }
    #[allow(dead_code)]
    fn ratio_add_commit_to_pot_old(game: &PreflopGame, position: Position) -> RatioNeedCoomitToPot {
        let my_commit = already_commit_by_pos(game, position);
        let max_commit = max_current_commit_from_all(game);
        let add_to_commit = max_commit - my_commit; // Равен 0, если ББ и все фолд или колл. Иначе больше.
        let ratio = dec!(100) * add_to_commit / game.main_pot.value; // Пот создается 1.5, а дальше только увеличивается.
        RatioNeedCoomitToPot::from(ratio)
    }
    fn ratio_add_commit_to_pot(game: &PreflopGame, position: Position) -> RatioNeedCoomitToPot {
        let my_commit = already_commit_by_pos(game, position);
        RatioNeedCoomitToPot::from(my_commit)
    }
    fn playing_stack(
        game: &PreflopGame,
        position: Position,
        my_initial_stack: Decimal,
    ) -> FakeStackSize {
        /* Result is
        - If my stack Shallow then Shallow
        - Else if we Deep then analize all start-street-stacks for all not-fold players exclude mine.
        - Else If exists at least one with >75 then Deep, except Shallow.
         */
        if FakeStackSize::from(my_initial_stack) == FakeStackSize::Shallow {
            return FakeStackSize::Shallow;
        }
        if !game
            .players()
            .iter()
            .filter(|player| {
                player.position != position
                    && FakeStackSize::from(player.stack_size) == FakeStackSize::Deep
            })
            .all(|player| game.folded_positions().contains(&player.position))
        {
            FakeStackSize::Deep
        } else {
            FakeStackSize::Shallow
        }
    }
    fn position_order_list() -> Vec<Position> {
        vec![
            Position::Sb,
            Position::Bb,
            Position::Utg,
            Position::Mp,
            Position::Co,
            Position::Btn,
        ]
    }
}
