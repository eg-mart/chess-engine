use crate::PieceType::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

pub enum Players {
    White,
    Black,
}

pub struct ChessIndex {
    x: u8,
    y: u8,
}

impl ChessIndex {
    pub fn new(x: u8, y: u8) -> ChessIndex {
        if x >= 8 || y >= 8 {
            panic!("Value must be between 0 and 7, got x: {}, y: {}", x, y);
        }
        ChessIndex { x, y }
    }

    pub fn linear(&self) -> u8 {
        self.x + self.y * 8
    }

    pub fn index(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    pub fn update(&mut self, dx: i8, dy: i8) {
        if self.x as i8 + dx >= 8
            || self.x as i8 + dx < 0
            || self.y as i8 + dy >= 8
            || self.y as i8 + dy < 0
        {
            panic!(
                "Value must be between 0 and 7, after update got x: {}, y: {}",
                self.x as i8 + dx,
                self.y as i8 + dy
            );
        }
        self.x = (dx + self.x as i8) as u8;
        self.y = (dy + self.y as i8) as u8;
    }
}

pub struct Board {
    content: [PieceType; 64],
    allowed_moves: HashMap<ChessIndex, Vec<ChessIndex>>,
    current_player: Players,
}

impl Board {
    pub fn make_move(&mut self, from: ChessIndex, to: ChessIndex) -> Result<(), String> {
        unimplemented!();
    }

    fn calculate_allowed_moves(&mut self) {
        unimplemented!();
    }

    pub fn calculate_best_move(&self) -> (ChessIndex, ChessIndex) {
        unimplemented!();
    }

    pub fn get_allowed_moves(&self) -> &HashMap<ChessIndex, Vec<ChessIndex>> {
        &self.allowed_moves
    }

    pub fn get_content(&self) -> &[PieceType] {
        &self.content
    }

    pub fn start_position() -> Board {
        let content = [Empty; 64];
        Board {
            content,
            allowed_moves: HashMap::new(),
            current_player: Players::White,
        }
    }
}
