use std::num::NonZeroUsize;
use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;

use log::info;

use itertools::Itertools;
use rand::RngCore;

use crate::{Battlesnake, Board, Game, Move};

pub fn get_info() -> JsonValue {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "",
        "color": "#FF99DD",
        "head": "default",
        "tail": "default",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn fit(_game: &Game, board: &Board, you: &Battlesnake, move_: &Move) -> usize {
    let my_head = &you.head;
    match move_ {
        Move::Up if my_head.y == board.height - 1 => return 0,
        Move::Down if my_head.y == 0 => return 0,
        Move::Left if my_head.x == 0 => return 0,
        Move::Right if my_head.x == board.width - 1 => return 0,
        _ => {}
    }
    for snake in &board.snakes {
        for body in &snake.body {
            let offset = (
                body.x as i32 - my_head.x as i32,
                body.y as i32 - my_head.y as i32,
            );
            match (offset, move_) {
                ((-1, 0), Move::Left) => return 0,
                ((1, 0), Move::Right) => return 0,
                ((0, -1), Move::Down) => return 0,
                ((0, 1), Move::Up) => return 0,
                _ => {}
            }
        }
    }
    let new_head = my_head.advance(move_).unwrap();
    for food in &board.food {
        if new_head.x.abs_diff(food.x) + new_head.y.abs_diff(food.y) <= you.health {
            return usize::MAX;
        }
    }
    you.health as usize
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, you: &Battlesnake) -> &'static str {
    //dbg!(game, board, you);
    let mut moves = [
        Move::Up,
        Move::Down,
        Move::Left,
        Move::Right,
    ];

    let chosen = moves.into_iter().map(|m| (m, fit(game, board, you, &m), rand::thread_rng().next_u32())).max_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2))).unwrap().0;

    info!("{} MOVE {}", game.id, chosen);

    (&chosen).into()
}
