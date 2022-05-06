use super::{Piece, piece_id::PieceId};
use crate::{
    board::{Board, board_history::BoardHistory, board_slice::BoardSlice},
    movement::can_move::CanMove,
};

pub trait PieceSet<'a> {
    type PieceType: Piece;
    fn moveset(&self, piece: &Self::PieceType) -> Option<&[CanMove<'a, Self::PieceType>]>;
    fn valid_slice(
        &self,
        piece_id: &PieceId<Self::PieceType>,
        board: &dyn Board<PieceType = Self::PieceType>,
        history: &BoardHistory,
    ) -> BoardSlice;
}
