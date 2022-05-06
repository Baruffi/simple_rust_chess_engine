use super::{can_capture::CanCapture, Move};
use crate::{
    board::{Board, board_history::BoardHistory},
    piece::piece_id::PieceId,
};

pub enum CanMove<'a, P> {
    Free(Move, CanCapture<'a, P>),
    Conditional(
        &'a dyn Fn(
            &PieceId<P>,
            &dyn Board<PieceType = P>,
            &BoardHistory,
        ) -> Option<(Move, CanCapture<'a, P>)>,
    ),
}
