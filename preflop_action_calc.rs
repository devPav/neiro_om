use std::collections::HashMap;

use crate::{ActionKind, Game, Player, Position, PreflopGame};
use rand::{thread_rng, Rng};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub fn possible_action_kind(game: &PreflopGame, position: Position) -> Vec<ActionKind> {
    /*
    - Вернет список возможных действий. Для действий Call и Raise ассоциированным значением будет общее количество денег,
    которое игрок по итогам всех раундов вложил в розыгрыш.
    - Пустой список возвращается для игрока, если он уже находится в состоянии Fold на предыдущих кругах.
    - Пустой список возвращается для игрока, если он уже находится в состоянии Allin на предыдущих кругах.
    - Пустой список возвращается, когда невозможно ниодно действие. ЭТО ЗНАЧИТ КОНЕЦ РАЗДАЧИ.
     */

    if game.folded_positions.contains(&position) {
        return vec![];
    }

    let player = game.player_by_position_as_ref(position);

    let my_commit = already_commit_by_pos(game, position);
    let max_commit = max_current_commit_from_all(game);
    let i_am_in_all_in = my_commit == player.stack_size;

    if i_am_in_all_in {
        return vec![];
    }

    let have_call_option = my_commit < max_commit;
    let can_min_raise = can_raise_by_stack_and_action(game, position, my_commit, max_commit);
    let can_check = can_check(position, my_commit, max_commit);
    let can_full_call = can_full_call(game, position, my_commit, max_commit);
    let can_over_full_call_alin = can_over_full_call_alin(game, position, my_commit, max_commit);

    let old_pot = game.main_pot.value;
    let size_pot_raise = max_commit + (dec!(1) * (old_pot - my_commit + max_commit)).round_dp(0);
    let size_75_raise = max_commit + (dec!(0.75) * (old_pot - my_commit + max_commit)).round_dp(0);
    let size_50_raise = max_commit + (dec!(0.5) * (old_pot - my_commit + max_commit)).round_dp(0);
    let rnd_size = match rand::thread_rng().gen_range(1u8..=3u8) {
        1 => size_pot_raise,
        2 => size_75_raise,
        _ => size_50_raise,
    };

    let can_rnd_size_raise = can_min_raise && (player.stack_size >= rnd_size);

    let mut possible_acts = vec![];

    if can_min_raise {
        if can_check {
            possible_acts.push(ActionKind::Check); // Если могу чекнуть, то ни фолда ни колла нет.
        } else {
            possible_acts.push(ActionKind::Fold); // Если могу рейзнуть, но не могу чекнуть, значит было повышение до меня.
            possible_acts.push(ActionKind::Call(max_commit)); // Если могу рейзнуть, то очевидно что полный колл возможен.
        }

        if can_rnd_size_raise {
            possible_acts.push(ActionKind::Raise(rnd_size));
        } else {
            possible_acts.push(ActionKind::Raise(player.stack_size)) // Если могу рейзнуть, но не хватает на сайзы, ставлю все что хватает
        }
    }

    if !can_check && !can_min_raise && have_call_option {
        possible_acts.push(ActionKind::Fold);
        if can_over_full_call_alin {
            possible_acts.push(ActionKind::Call(player.stack_size));
            possible_acts.push(ActionKind::Call(max_commit))
        } else if can_full_call {
            possible_acts.push(ActionKind::Call(max_commit))
        } else {
            possible_acts.push(ActionKind::Call(player.stack_size))
        }
    }
    possible_acts
}
pub fn already_commit_by_pos(game: &PreflopGame, position: Position) -> Decimal {
    /* Ищется сколько денег уже внес в банк игрок, играющий на конкретной позиции */
    game.positions_and_money
        .iter()
        .find(|(&x, &y)| x == position)
        .map(|(_, y)| *y)
        .unwrap_or_else(|| unreachable!())
}
pub fn max_current_commit_from_all(game: &PreflopGame) -> Decimal {
    /* Это максимальное вложение в банк, из тех, которые сделали все шесть игроков на текущий момент
    Сфолдили или нет не важно, так как предполагаю у того кто сфолдил максимального не будет.
    */
    game.positions_and_money
        .values()
        .max()
        .map(|x| *x)
        .unwrap_or_else(|| unreachable!())
}
pub fn can_raise_by_stack_and_action(
    game: &PreflopGame,
    position: Position,
    already_commit: Decimal,
    max_commit: Decimal,
) -> bool {
    /* Рейз не возможен, если:
    1. Текущий взнос всех шести игроков в банк, которые не сфолдили, равен твоему или по крайней
    мере разница меньше min_bet, то есть был повышенный колл AllinCall или просто Call, после которого
    нельзя рейзить. Кроме ВВ на первом кругу ставок!
    2. Если после кола предыдущего рейза, ты не можешь повысить на min_bet и выше.
     */
    let bb_sb_on_first_lap = (position == Position::Bb && already_commit == dec!(1))
        || (position == Position::Sb && already_commit == dec!(0.5));

    let can_raise_by_action = !game
        .positions_and_money
        .iter()
        .filter(|(&pos, _)| !game.folded_positions.contains(&pos))
        .all(|(_, &x)| x < (already_commit + game.min_bet));

    let player = game.player_by_position_as_ref(position);
    let can_raise_by_stack = (player.stack_size - max_commit) >= game.min_bet;

    (bb_sb_on_first_lap || can_raise_by_action) && can_raise_by_stack
}
pub fn can_check(position: Position, already_commit: Decimal, max_commit: Decimal) -> bool {
    /* Чек на префлопе возможен только, если одновременно
    1. Ты на ВВ и это первый круг ставок
    2. Максимальный взнос в банк всех шести игроков(не важно сфолдили или нет) равен твоему, то есть
    все сфолдили или все вколили.
     */
    (position == Position::Bb && already_commit == dec!(1)) && (already_commit == max_commit)
}
pub fn can_full_call(
    game: &PreflopGame,
    position: Position,
    already_commit: Decimal,
    max_commit: Decimal,
) -> bool {
    /* Полный колл возможен, если:
    1. Твой изначальный стек за вычетом тех денег, которые ты уже внес в банк, больше или равен тому, что
    ты должен доставить до максимальной ставки.
    init_stack - already_commit >= max_commit - already_commit
     */
    let player = game.player_by_position_as_ref(position);
    player.stack_size >= max_commit
}
pub fn can_over_full_call_alin(
    game: &PreflopGame,
    position: Position,
    already_commit: Decimal,
    max_commit: Decimal,
) -> bool {
    /* Олын, который будет не рейзом а коллом возможен, если:
    1. Твой изначальный стек за вычетом тех денег, которые ты уже внес в банк, больше тому, что
    ты должен доставить до максимальной ставки.
    init_stack - already_commit >= max_commit - already_commit
    2. Причем нельзя сделать такой колл на весь свой стек, если в итоге получится рейз. То есть
    stack_size - max_commit < min_bet
     */
    let player = game.player_by_position_as_ref(position);
    player.stack_size > max_commit && ((player.stack_size - max_commit) < game.min_bet)
}
