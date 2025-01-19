mod game;
use game::*;

mod board;
use board::*;

mod wasm;
use wasm::*;

#[macro_use]
extern crate lazy_static;

use mut_static::MutStatic;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<GameEngine> = MutStatic::from(GameEngine::new());
}
