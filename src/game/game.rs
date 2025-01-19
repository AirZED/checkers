use std::{fmt::Result, vec};

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
        let legal_moves = self.legal_moves();
        if !legal_moves.contains(mv) {
            return Err(());
        }

        let Coordinate(fromX, fromY) = mv.from;
        let Coordinate(toX, toY) = mv.to;

        let jumped_coordinate = self.get_jumped_coordinate(fromX, fromY, toX, toY);

        if let Some(Coordinate(x, y)) = jumped_coordinate {
            self.board[x][y] = None
        }

        let piece = self.board[fromX][fromY].unwrap();
        self.board[toX][toY] = Some(piece);
        self.board[fromX][fromY] = None;

        let crowned = match self.should_crown(piece, mv.to) {
            true => {
                self.crown_piece(mv.to);
                true
            }
            false => false,
        };

        // change the turn to player B
        self.advance_turn();

        Ok(MoveResult {
            mv: mv.clone(),
            crowned: crowned,
        })
    }

    pub fn advance_turn(&mut self) {
        if self.current_turn == PieceColor::BLACK {
            self.current_turn = PieceColor::WHITE
        } else {
            self.current_turn = PieceColor::BLACK
        }
        self.move_count += 1;
    }

    pub fn get_current_turn(&self) -> PieceColor {
        return self.current_turn;
    }

    fn crown_piece(&mut self, coordinate: Coordinate) {
        let Coordinate(x, y) = coordinate;
        let piece = self.board[x][y].unwrap();
        self.board[x][y] = Some(GamePiece::crown(piece))
    }

    fn should_crown(&mut self, piece: GamePiece, to: Coordinate) -> bool {
        let Coordinate(_, y) = to;

        (y == 0 && piece.color == PieceColor::BLACK) || (y == 7 && piece.color == PieceColor::WHITE)
    }

    fn get_jumped_coordinate(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Option<Coordinate> {
        if to_x == from_x + 2 && to_y == from_y + 2 {
            Some(Coordinate(from_x + 1, from_y + 1))
        } else if to_x == from_x - 2 && to_y == from_y - 2 {
            Some(Coordinate(from_x - 1, from_y - 1))
        } else if to_x == from_x - 2 && to_y == from_y + 2 {
            Some(Coordinate(from_x - 1, from_y + 1))
        } else if to_x == from_x + 2 && to_y == from_y - 2 {
            Some(Coordinate(from_x + 1, from_y - 1))
        } else {
            None
        }
    }

    fn legal_moves(&mut self, loc: Coordinate) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.board[x][y] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(x, y);
                        let mut valid_moves = self.get_valid_moves(loc);
                    }
                }
            }
        }

        moves
    }

    fn get_valid_moves(&mut self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(piece) = self.board[x][y] {
            let mut jumps = loc
                .get_jump_targets()
                .filter(|t| self.valid_jump(&piece, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                })
                .collect::<Vec<Move>>();

            let mut moves = loc
                .get_move_targets()
                .filter(|t| self.valid_jump(&piece, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                })
                .collect::<Vec<Move>>();

            jumps.append(&mut moves);
            jumps
        } else {
            vec![]
        }
    }

    fn valid_jump(&mut self, piece: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        // on board
        if !to.is_on_board() || !from.is_on_board() {
            return false;
        }

        // space has some value
        let Coordinate(tx, ty) = *to;
        let Coordinate(fx, fy) = *from;
        if let Some(_) = self.board[tx][ty] {
            return false;
        }

        // jumpable piece
        let midcoord = self.get_jumped_coordinate(fx, fy, tx, ty);
        if midcoord.is_none() {
            return false;
        }
        let midpiece = self.board[midcoord.unwrap().0][midcoord.unwrap().1];
        if midpiece.is_some() {
            if (midpiece.unwrap().color == piece.color) {
                return false;
            }
        } else {
            return false;
        }

        let mut valid = false;

        if ty > fy && piece.color == PieceColor::WHITE {
            // white moves down
            valid = true;
        }

        if ty < fy && piece.color == PieceColor::BLACK {
            // black moves up
            valid = true;
        }

        if ty > fy && piece.color == PieceColor::BLACK && piece.crowned {
            // black crowned moves down
            valid = true;
        }

        if ty < fy && piece.color == PieceColor::WHITE && piece.crowned {
            // white crowned moves up
            valid = true;
        }

        valid
    }
}
