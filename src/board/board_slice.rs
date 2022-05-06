use crate::{
    implementations::board::standard_board::StandardBoard,
    piece::{Piece, piece_id::PieceId},
};

use super::Board;

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
