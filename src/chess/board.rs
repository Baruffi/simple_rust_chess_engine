use std::collections::HashMap;

use crate::chess::{
    piece::{Piece, PieceId, PiecePos},
    standard::board::StandardBoard,
};

pub trait Board {
    type PieceType;
    fn get_row_size(&self) -> usize;
    fn get_col_size(&self) -> usize;
    fn get_board_size(&self) -> usize;
    fn get_id(&self, pos: &PiecePos<Self::PieceType>) -> Option<PieceId<Self::PieceType>>;
    fn get_id_not_none(&self, pos: &PiecePos<Self::PieceType>) -> Option<PieceId<Self::PieceType>>;
    fn get_pos(&self, id: &PieceId<Self::PieceType>) -> Option<PiecePos<Self::PieceType>>;
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

pub struct BoardSlice(pub Vec<usize>);

impl BoardSlice {
    pub fn new(default: Option<Vec<usize>>) -> Self {
        match default {
            Some(n) => BoardSlice(n),
            None => BoardSlice(Vec::new()),
        }
    }

    pub fn inner(&self) -> &Vec<usize> {
        &self.0
    }

    pub fn push(&mut self, pos: usize) {
        self.0.push(pos);
    }

    pub fn visualize<
        const T_ROW_SIZE: usize,
        const T_COL_SIZE: usize,
        const T_BOARD_SIZE: usize,
        P: Piece,
    >(
        &self,
        fill: isize,
    ) -> StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P> {
        let mut visual = StandardBoard::new([0; T_BOARD_SIZE]);
        for v in &self.0 {
            visual.set_square(&PieceId(fill.into(), fill.into(), 0), *v);
        }
        return visual;
    }
}
