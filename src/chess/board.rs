use std::collections::{HashMap, HashSet};

use crate::chess::piece::{Piece, PieceId, PiecePos};

use super::piece::Sign;

pub trait Board {
    type PieceType;
    fn get_row_size(&self) -> usize;
    fn get_col_size(&self) -> usize;
    fn get_board_size(&self) -> usize;
    fn get_all_pieces(&self) -> HashSet<Self::PieceType>;
    fn get_all_versions(&self, piece: Self::PieceType, sign: Sign) -> Vec<usize>;
    fn get_id(&self, pos: &PiecePos<Self::PieceType>) -> Option<PieceId<Self::PieceType>>;
    fn get_pos(&self, id: &PieceId<Self::PieceType>) -> Option<PiecePos<Self::PieceType>>;
    fn get_slice(&self) -> BoardSlice;
    fn set_square(&mut self, id: &PieceId<Self::PieceType>, square: usize);
    fn clear(&mut self);
}

pub struct BoardHistory {
    pub past: HashMap<(isize, usize), BoardSlice>,
}

impl BoardHistory {
    pub fn new(initial: Option<HashMap<(isize, usize), BoardSlice>>) -> Self {
        match initial {
            Some(past) => BoardHistory { past },
            None => BoardHistory {
                past: HashMap::new(),
            },
        }
    }

    pub fn get_slice<P: Piece>(&self, id: &PieceId<P>) -> Option<&BoardSlice> {
        self.past.get(&(id.i(), id.version()))
    }

    pub fn push<P: Piece>(&mut self, id: &PieceId<P>, pos: &PiecePos<P>) {
        let slice = self
            .past
            .entry((id.i(), id.version()))
            .or_insert(BoardSlice(Vec::new()));
        slice.push(pos.u());
    }

    pub fn clear(&mut self) {
        self.past = HashMap::new();
    }
}

pub struct BoardSlice(pub Vec<usize>);

impl BoardSlice {
    pub fn new(default: Option<Vec<usize>>) -> Self {
        match default {
            Some(n) => BoardSlice(n),
            None => BoardSlice(Vec::new()),
        }
    }

    pub fn last(&self) -> Option<&usize> {
        self.0.last()
    }

    pub fn push(&mut self, pos: usize) {
        self.0.push(pos);
    }

    pub fn add<P: Piece>(&self, amount: isize, board: &mut dyn Board<PieceType = P>) {
        for v in &self.0 {
            if let Some(existing_piece) = board.get_id(&PiecePos(*v, board)) {
                let real_fill = existing_piece.i() + amount;
                board.set_square(&PieceId(real_fill.into(), real_fill.into(), 0), *v);
            }
        }
    }

    pub fn subtract<P: Piece>(&self, amount: isize, board: &mut dyn Board<PieceType = P>) {
        for v in &self.0 {
            if let Some(existing_piece) = board.get_id(&PiecePos(*v, board)) {
                let real_fill = existing_piece.i() - amount;
                board.set_square(&PieceId(real_fill.into(), real_fill.into(), 0), *v);
            }
        }
    }
}
