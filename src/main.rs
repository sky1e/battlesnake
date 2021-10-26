#![feature(proc_macro_hygiene, decl_macro)]
#![feature(int_abs_diff)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use log::info;
use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};

mod logic;

// Request types derived from https://docs.battlesnake.com/references/api#object-definitions
// For a full example of Game Board data, see https://docs.battlesnake.com/references/api/sample-move-request

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,

    // Used in non-standard game modes
    shout: Option<String>,
    squad: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    pub fn advance(&self, move_: &Move) -> Option<Self> {
        Some(match move_ {
            Move::Up => Coord {x: self.x, y: self.y.checked_add(1)?},
            Move::Down => Coord {x: self.x, y: self.y.checked_sub(1)?},
            Move::Left => Coord {x: self.x.checked_sub(1)?, y: self.y},
            Move::Right => Coord {x: self.x.checked_add(1)?, y: self.y},
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Move::Up => "up",
            Move::Down => "down",
            Move::Left => "left",
            Move::Right => "right",
        })
    }
}

impl From<&Move> for &'static str {
    fn from(move_: &Move) -> Self {
        match move_ {
            Move::Up => "up",
            Move::Down => "down",
            Move::Left => "left",
            Move::Right => "right",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[get("/")]
fn handle_index() -> JsonValue {
    logic::get_info()
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<GameState>) -> Status {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>) -> JsonValue {
    let chosen = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    return json!({ "move": chosen });
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<GameState>) -> Status {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}

fn main() {
    let address = "0.0.0.0";
    let env_port = env::var("PORT").ok();
    let env_port = env_port.as_ref().map(String::as_str).unwrap_or("8080");
    let port = env_port.parse::<u16>().unwrap();

    env_logger::init();

    let config = Config::build(Environment::Development)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();

    info!(
        "Starting Battlesnake Server at http://{}:{}...",
        address, port
    );
    rocket::custom(config)
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
        .launch();
}
