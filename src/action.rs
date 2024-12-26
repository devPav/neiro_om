use crate::{ActionKind, Game, Position};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub fn possible_action_kind(game: &impl Game, position: Position) -> Vec<ActionKind> {
    /* 100% код preflop_action_calc.rs
    - Вернет список возможных действий. Для действий Call и Raise ассоциированным значением будет общее количество денег,
    которое игрок по итогам всех раундов на этой улице вложил в розыгрыш.
    - Пустой список возвращается для игрока, если он уже находится в состоянии Fold на предыдущих кругах.
    - Пустой список возвращается для игрока, если он уже находится в состоянии Allin на предыдущих кругах.
    - Пустой список возвращается, когда невозможно ни одно действие. ЭТО ЗНАЧИТ КОНЕЦ РАЗДАЧИ.

    !!! Не могу сыграть рейз или оверколл-алын, если все вокруг меня в алине. Только колл или фолд.
        Это важно для рассчета пота для рейка.
     */

    if game.folded_positions().contains(&position) {
        return vec![];
    }

    let player = game.player_by_position_as_ref(position);

    let my_commit = already_commit_by_pos(game, position);
    let i_am_in_all_in = my_commit == player.stack_size;

    if i_am_in_all_in {
        return vec![];
    }

    let max_commit = max_current_commit_from_all(game);
    let have_call_option = my_commit < max_commit;
    let can_min_raise = can_raise_by_stack_and_action(game, position, my_commit, max_commit);
    let can_check = can_check(game, position, my_commit, max_commit);
    let can_full_call = can_full_call(game, position, max_commit);
    let can_over_full_call_alin = can_over_full_call_alin(game, position, max_commit);

    let old_pot = game.main_pot().value;
    let size_pot_raise = max_commit + (dec!(1) * (old_pot - my_commit + max_commit)).round_dp(0);
    let size_75_raise = max_commit + (dec!(0.75) * (old_pot - my_commit + max_commit)).round_dp(0);
    let size_50_raise = max_commit + (dec!(0.5) * (old_pot - my_commit + max_commit)).round_dp(0);
    // let rnd_size = match rand::thread_rng().gen_range(1u8..=3u8) {
    //     1 => size_75_raise,
    //     2 => size_50_raise,
    //     _ => size_pot_raise,
    // };
    let can_pot_size_raise = can_min_raise && (player.stack_size >= size_pot_raise);
    let can_75_size_raise = can_min_raise && (player.stack_size >= size_75_raise);
    let can_50_size_raise = can_min_raise && (player.stack_size >= size_50_raise);

    let mut possible_acts = vec![];

    if can_min_raise {
        if can_check {
            possible_acts.push(ActionKind::Check); // Если могу чекнуть, то ни фолда ни колла нет.
        } else {
            possible_acts.push(ActionKind::Fold); // Если могу рейзнуть, но не могу чекнуть, значит было повышение до меня.
            possible_acts.push(ActionKind::Call(max_commit)); // Если могу рейзнуть, то очевидно что полный колл возможен.
        }
        // На данный момент очень важно не добавлять рейз-олины нигде, кроме последней ветки.
        // Потому что при определении id рейза в редиске на эту логику идет базирование в get_action_id()
        if can_pot_size_raise {
            possible_acts.push(ActionKind::Raise(size_pot_raise));
            possible_acts.push(ActionKind::Raise(size_75_raise));
            possible_acts.push(ActionKind::Raise(size_50_raise));
        } else if can_75_size_raise {
            possible_acts.push(ActionKind::Raise(player.stack_size)); // ???
            possible_acts.push(ActionKind::Raise(size_75_raise));
            possible_acts.push(ActionKind::Raise(size_50_raise));
        } else if can_50_size_raise {
            possible_acts.push(ActionKind::Raise(player.stack_size)); // ???
            possible_acts.push(ActionKind::Raise(size_50_raise));
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
pub fn already_commit_by_pos(game: &impl Game, position: Position) -> Decimal {
    /* 100% код preflop_action_calc.rs
    Ищется сколько денег уже внес в банк игрок, играющий на конкретной позиции */
    game.positions_and_money()
        .iter()
        .find(|(&x, _)| x == position)
        .map(|(_, y)| *y)
        .unwrap_or_else(|| unreachable!())
}
pub fn max_current_commit_from_all(game: &impl Game) -> Decimal {
    /* 100% код preflop_action_calc.rs
    Это максимальное вложение в банк, из тех, которые сделали все шесть игроков на текущий момент
    Сфолдили или нет не важно, так как предполагаю у того кто сфолдил максимального не будет.
    */
    game.positions_and_money()
        .values()
        .max()
        .map(|x| *x)
        .unwrap_or_else(|| unreachable!())
}
pub fn can_raise_by_stack_and_action(
    game: &impl Game,
    position: Position,
    already_commit: Decimal,
    max_commit: Decimal,
) -> bool {
    /* Расширяет preflop_action_calc.rs.
    Рейз не возможен, если:
    1. Текущий взнос всех шести игроков в банк, которые не сфолдили, равен твоему или по крайней
    мере разница меньше min_bet, то есть был повышенный колл AllinCall или просто Call, после которого
    нельзя рейзить.
    + Кроме первого действующего на первом кругу, или первого действующего после чеков перед ним постфлоп!
    НО нет если на первом кругу чел, а остальные в алине или фолде.
    + Кроме ВВ и SB на первом кругу ставок префлоп!
    2. Если после кола предыдущего рейза, ты можешь повысить на min_bet и выше.
     */
    let can_raise_besides_action = if game.is_preflop_game() {
        (position == Position::Bb && already_commit == dec!(1))
            || (position == Position::Sb && already_commit == dec!(0.5))
    } else {
        max_commit == Decimal::ZERO
            && already_commit == Decimal::ZERO
            && !all_alin_or_fold_except_me(game, position)
    };

    let can_raise_by_action = !game
        .positions_and_money()
        .iter()
        .filter(|(&pos, _)| !game.folded_positions().contains(&pos))
        .all(|(_, &x)| x < (already_commit + game.min_bet()));

    let player = game.player_by_position_as_ref(position);
    let can_raise_by_stack = (player.stack_size - max_commit) >= game.min_bet();

    (can_raise_besides_action || can_raise_by_action)
        && can_raise_by_stack
        && !all_alin_or_fold_except_me(game, position)
}
pub fn can_check(
    game: &impl Game,
    position: Position,
    already_commit: Decimal,
    max_commit: Decimal,
) -> bool {
    /* Расширяет preflop_action_calc.rs.
    Чек на префлопе возможен только, если до тебя были только чеки.
     */
    if game.is_preflop_game() {
        (position == Position::Bb && already_commit == dec!(1)) && (already_commit == max_commit)
    } else {
        max_commit == Decimal::ZERO && already_commit == Decimal::ZERO
    }
}
pub fn can_full_call(game: &impl Game, position: Position, max_commit: Decimal) -> bool {
    /* 100% код preflop_action_calc.rs
    Полный колл возможен, если:
    1. Твой изначальный стек за вычетом тех денег, которые ты уже внес в банк, больше или равен тому, что
    ты должен доставить до максимальной ставки.
    */
    let player = game.player_by_position_as_ref(position);
    player.stack_size >= max_commit
}
pub fn can_over_full_call_alin(game: &impl Game, position: Position, max_commit: Decimal) -> bool {
    /* 100% код preflop_action_calc.rs
    Олын, который будет не рейзом а коллом возможен, если:
    1. Твой изначальный стек за вычетом тех денег, которые ты уже внес в банк, больше тому, что
    ты должен доставить до максимальной ставки.
    init_stack - already_commit >= max_commit - already_commit
    2. Причем нельзя сделать такой колл на весь свой стек, если в итоге получится рейз. То есть
    stack_size - max_commit < min_bet
    3. Также нельзя сделать олынколл елси все соперники уже в фолде или олыне, ибо нет смысла, можно только колл
    4. !can_check(), проверяется в главной процедуре
     */
    let player = game.player_by_position_as_ref(position);
    player.stack_size > max_commit
        && ((player.stack_size - max_commit) < game.min_bet())
        && !all_alin_or_fold_except_me(game, position)
}
pub fn all_alin_or_fold_except_me(game: &impl Game, position: Position) -> bool {
    game.positions_and_money()
        .iter()
        .filter(|(&pos, _)| pos != position)
        .all(|(&pos, &money)| {
            game.player_by_position_as_ref(pos).stack_size == money
                || game.folded_positions().contains(&pos)
        })
}
