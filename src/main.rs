mod kalaha;
mod engine;

use std::io;
use crate::kalaha::*;
use crate::engine::best_move;

fn main() {
    let mut player = PLAYER_ONE;
    let mut kalaha = Kalaha::new();
    loop {
        println!("Game: {:?}", kalaha.to_string());
        println!("Player {} is on turn", player + 1);
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let line = &buf.as_str().replace("\r\n", "");
        let mve: usize;
        if line == "engine" {
            mve = best_move(kalaha, player, 5);
            println!("Engine chose move {}", mve);
        } else if line == "quit" {  break}
        else  {
            if let Ok(i) = line.parse::<usize>() {
                mve = i;
            } else {
                println!("Illegal move {}", line);
                continue;
            }
        }
        let res = kalaha.make_move(mve, player);
        match res {
            AfterMove::Regular { kalaha: k, next_move } => {
                kalaha = k;
                player = next_move;
            }
            AfterMove::Winner(winner) => {
                println!("Player {} has won", winner);
                break
            }
        }
    }
}