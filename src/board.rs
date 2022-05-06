use self::board_slice::BoardSlice;
use crate::piece::{piece_id::PieceId, piece_pos::PiecePos};

pub mod board_history;
pub mod board_slice;

pub trait Board {
    type PieceType;
    fn get_row_size(&self) -> usize;
    fn get_col_size(&self) -> usize;
    fn get_board_size(&self) -> usize;
    fn piece_slice(&self, ids: &[PieceId<Self::PieceType>]) -> BoardSlice;
    fn get_id(&self, pos: &PiecePos<Self::PieceType>) -> Option<PieceId<Self::PieceType>>;
    fn get_id_not_none(&self, pos: &PiecePos<Self::PieceType>) -> Option<PieceId<Self::PieceType>>;
    fn get_pos(&self, id: &PieceId<Self::PieceType>) -> Option<PiecePos<Self::PieceType>>;
    fn set_square(&mut self, id: &PieceId<Self::PieceType>, square: usize);
    fn clear(&mut self);
}
