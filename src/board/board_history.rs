use super::board_slice::BoardSlice;
use crate::piece::{Piece, piece_id::PieceId, piece_pos::PiecePos};
use std::collections::HashMap;

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
        let old_slice = self.past.get(&(id.i(), id.version()));
        match old_slice {
            Some(slice) => {
                let mut new_slice = BoardSlice::new(Some(slice.inner().to_vec()));
                new_slice.push(pos.u());
                self.past.insert((id.i(), id.version()), new_slice);
            }
            None => {
                let new_slice = BoardSlice::new(Some(vec![pos.u()]));
                self.past.insert((id.i(), id.version()), new_slice);
            }
        }
    }

    pub fn clear(&mut self) {
        self.past = HashMap::new();
    }
}
