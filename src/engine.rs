use crate::kalaha::*;
use std::ops::RangeInclusive;

fn eval(kalaha: &Kalaha) -> i8 {
    kalaha.get_bucket(6) as i8 - kalaha.get_bucket(13) as i8
}

const MIN: i8 = -128;
const MAX: i8 = 127;

fn min(kalaha: Kalaha, depth: u8) -> i8 {
    if depth == 0 { return eval(&kalaha) }
    let mut min_eval = MAX;
    for mve in 7..=12 {
        if kalaha.get_bucket(mve) == 0 { continue }
        let res = kalaha.make_move(mve, PLAYER_TWO);
        match res {
            AfterMove::Winner(w) => match w {
                PLAYER_ONE => {} 
                PLAYER_TWO => return MIN,
                REMIS => if min_eval > 0 { min_eval = 0 },
                _ => panic!(),
            },
            AfterMove::Regular { kalaha, next_move } => {
                let eval = 
                    if next_move == PLAYER_ONE { max(kalaha, depth - 1) }
                    else { min(kalaha, depth - 1) };
                if eval < min_eval { min_eval = eval }
            }
        }
    }
    return min_eval;
}
fn max(kalaha: Kalaha, depth: u8) -> i8 {
    if depth == 0 { return eval(&kalaha) }
    let mut max_eval = MIN;
    for mve in 0..=5 {
        if kalaha.get_bucket(mve) == 0 { continue }
        let res = kalaha.make_move(mve, PLAYER_ONE);
        match res {
            AfterMove::Winner(w) => match w {
                PLAYER_ONE => return MAX, 
                PLAYER_TWO => {},
                REMIS => if max_eval < 0 { max_eval = 0 },
                _ => panic!(),
            },
            AfterMove::Regular { kalaha, next_move } => {
                let eval = 
                    if next_move == PLAYER_ONE { max(kalaha, depth - 1) }
                    else { min(kalaha, depth - 1) };
                if eval > max_eval { max_eval = eval }
            }
        }
    }
    return max_eval;
}

fn greater(x: i8, y: i8) -> bool {
    x > y
}

fn lower(x: i8, y: i8) -> bool {
    x < y
}

pub fn best_move(kalaha: Kalaha, player: u8, depth: u8) -> usize {
    let mut best = None;
    let mut best_eval;
    let range: RangeInclusive<usize>;
    let comparator: fn(i8, i8) -> bool;
    if player == PLAYER_ONE {
        range = 0..=5;
        comparator = greater;
        best_eval = MIN;
    } else {
        range = 7..=12;
        comparator = lower;
        best_eval = MAX;
    }
    for mve in range {
        let eval = match kalaha.make_move(mve, player) {
            AfterMove::Winner(winner) => match winner {
                REMIS => 0,
                PLAYER_ONE => MAX,
                PLAYER_TWO => MIN,
                _ => panic!()
            },
            AfterMove::Regular { kalaha, next_move } => {
                if next_move == PLAYER_ONE { max(kalaha, depth) }
                else { min(kalaha, depth) }
            }
        };
        if comparator(eval, best_eval) { 
            best = Some(mve);
            best_eval = eval;
        }
    }
    best.unwrap()
}