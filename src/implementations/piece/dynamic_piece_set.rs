use crate::{
    board::{Board, board_history::BoardHistory, board_slice::BoardSlice},
    movement::can_move::CanMove,
    piece::{Piece, piece_id::PieceId, piece_set::PieceSet},
};
use std::collections::HashMap;

pub struct DynamicPieceSet<'a, T>(pub HashMap<T, Vec<CanMove<'a, T>>>)
where
    T: std::hash::Hash + Eq;

impl<'a, T> DynamicPieceSet<'a, T>
where
    T: Piece + std::hash::Hash + Eq,
{
    pub fn insert(&mut self, piece: T, piece_move: CanMove<'a, T>) {
        if let Some(moveset) = self.0.get_mut(&piece) {
            moveset.push(piece_move);
        } else {
            self.0.insert(piece, vec![piece_move]);
        }
    }

    fn valid_moves(
        &self,
        piece_id: &PieceId<T>,
        board: &dyn Board<PieceType = T>,
        history: &BoardHistory,
    ) -> Option<Vec<usize>> {
        let mut valid = Vec::new();
        let pos = board.get_pos(piece_id)?;
        let moveset = self.moveset(&piece_id.piece())?;
        for can_move in moveset {
            let mut move_op = match can_move {
                CanMove::Free(m, c) => m.calculate(piece_id, &pos, c, board),
                CanMove::Conditional(c) => match c(piece_id, board, history) {
                    Some((m, c)) => m.calculate(piece_id, &pos, &c, board),
                    None => Vec::new(),
                },
            };
            valid.append(&mut move_op);
        }
        Some(valid)
    }
}

impl<'a, T> PieceSet<'a> for DynamicPieceSet<'a, T>
where
    T: Piece + std::hash::Hash + Eq,
{
    type PieceType = T;

    fn moveset(&self, piece: &T) -> Option<&[CanMove<'a, Self::PieceType>]> {
        if let Some(moveset) = self.0.get(piece) {
            return Some(&moveset[..]);
        }
        return None;
    }

    fn valid_slice(
        &self,
        piece_id: &PieceId<T>,
        board: &dyn Board<PieceType = T>,
        history: &BoardHistory,
    ) -> BoardSlice {
        BoardSlice::new(self.valid_moves(piece_id, board, history))
    }
}
