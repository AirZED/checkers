use std::fmt::Result;

use crate::{Coordinate, GamePiece, PieceColor};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Self {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: usize,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            board: [[None; 8]; 8],
            current_turn: PieceColor::BLACK,
            move_count: 0,
        };

        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::WHITE));
            });

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::BLACK));
            });
    }

    pub fn get_piece(&self, coord: Coordinate) -> Result<&Option<GamePiece>, ()> {
        let Coordinate(x, y) = coord;

        if x <= 7 && y <= 7 && x >= 0 && y >= 0 {
            Ok(self.board[x][y])
        } else {
            Err(())
        }
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        // let legal_moves = self.legal_moves();

        Err(())
    }
}
