use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use log::info;

use crate::{Battlesnake, Board, Game};

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

pub fn get_move(game: &Game, _turn: &u32, board: &Board, you: &Battlesnake) -> &'static str {
    //dbg!(game, board, you);
    let mut possible_moves: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // Step 0: Don't let your Battlesnake move back in on its own neck
    let my_head = &you.head;
    for snake in &board.snakes {
        for body in &snake.body {
            match (body.x as i32 - my_head.x as i32, body.y as i32 - my_head.y as i32) {
                (-1, 0) => possible_moves.remove("left"),
                (1, 0) => possible_moves.remove("right"),
                (0, -1) => possible_moves.remove("down"),
                (0, 1) => possible_moves.remove("up"),
                _ => None
            };
        }
    }


    // TODO: Step 1 - Don't hit walls.
    // Use board information to prevent your Battlesnake from moving beyond the boundaries of the board.
    // board_width = move_req.board.width
    // board_height = move_req.board.height

    if my_head.x == 0 {
        possible_moves.remove("left");
    }
    if my_head.x == board.width - 1 {
        possible_moves.remove("right");
    }
    if my_head.y == 0 {
        possible_moves.remove("down");
    }
    if my_head.y == board.height - 1 {
        possible_moves.remove("up");
    }

    // TODO: Step 2 - Don't hit yourself.
    // Use body information to prevent your Battlesnake from colliding with itself.
    // body = move_req.body

    // TODO: Step 3 - Don't collide with others.
    // Use snake vector to prevent your Battlesnake from colliding with others.
    // snakes = move_req.board.snakes

    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    // food = move_req.board.food

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.
    let moves = possible_moves
        .into_iter()
        .filter(|&(_, v)| v == true)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();

    info!("{} MOVE {}", game.id, chosen);

    return chosen;
}
