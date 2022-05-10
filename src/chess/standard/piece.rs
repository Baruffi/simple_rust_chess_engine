use std::ops::Neg;

use crate::chess::{
    board::{Board, BoardHistory, BoardSlice},
    movement::{CanCapture, CanMove, CaptureCalculator, Move},
    piece::{Piece, PieceId, PieceSet},
};

#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
pub enum StandardPiece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Unknown(usize),
}

impl From<StandardPiece> for isize {
    fn from(p: StandardPiece) -> Self {
        match p {
            StandardPiece::None => 0,
            StandardPiece::Pawn => 1,
            StandardPiece::Knight => 2,
            StandardPiece::Bishop => 3,
            StandardPiece::Rook => 4,
            StandardPiece::Queen => 5,
            StandardPiece::King => 6,
            StandardPiece::Unknown(u) => u as isize,
        }
    }
}

impl From<isize> for StandardPiece {
    fn from(i: isize) -> Self {
        match i.abs() {
            0 => Self::None,
            1 => Self::Pawn,
            2 => Self::Knight,
            3 => Self::Bishop,
            4 => Self::Rook,
            5 => Self::Queen,
            6 => Self::King,
            u => Self::Unknown(u as usize),
        }
    }
}

impl Piece for StandardPiece {
    fn none() -> Self {
        Self::None
    }
}

pub struct StandardPieceSet;

impl StandardPieceSet {
    const PAWN_MOVESET: [CanMove<'static, StandardPiece>; 5] = [
        CanMove::Conditional(&|id, _, history| {
            if history.get_slice(id).is_none() {
                return Some((Move::new(0, 1, 2), CanCapture::None));
            }
            return Some((Move::new(0, 1, 1), CanCapture::None));
        }),
        CanMove::Conditional(&|id, board, _| {
            board.get_pos(id).and_then(|op| {
                let idx = op.topleft(id.sign());
                board.get_id(&idx).and_then(|other| {
                    if other.opposes(id) {
                        return Some((Move::new(-1, 1, 1), CanCapture::Opposing(1)));
                    }
                    return None;
                })
            })
        }),
        CanMove::Conditional(&|id, board, _| {
            board.get_pos(id).and_then(|op| {
                let idx = op.topright(id.sign());
                board.get_id(&idx).and_then(|other| {
                    if other.opposes(id) {
                        return Some((Move::new(1, 1, 1), CanCapture::Opposing(1)));
                    }
                    return None;
                })
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let op_left = op.left(id.sign());
                board.get_id(&op_left).and_then(|other| {
                    if other.opposes(id) && other.piece() == StandardPiece::Pawn {
                        return history.get_slice(&other).and_then(|prev| {
                            prev.last().and_then(|last| {
                                if last == &op_left.bottom(other.sign()).bottom(other.sign()).u() {
                                    return Some((Move::new(-1, 1, 1), CanCapture::None));
                                } else {
                                    return None;
                                }
                            })
                        });
                    }
                    return None;
                })
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let op_right = op.right(id.sign());
                board.get_id(&op_right).and_then(|other| {
                    if other.opposes(id) && other.piece() == StandardPiece::Pawn {
                        return history.get_slice(&other).and_then(|prev| {
                            prev.last().and_then(|last| {
                                if last == &op_right.bottom(other.sign()).bottom(other.sign()).u() {
                                    return Some((Move::new(1, 1, 1), CanCapture::None));
                                } else {
                                    return None;
                                }
                            })
                        });
                    }
                    return None;
                })
            })
        }),
    ];
    const KNIGHT_MOVESET: [CanMove<'static, StandardPiece>; 8] = [
        CanMove::Free(Move::new(1, 2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(2, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-2, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(2, -1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-2, -1, 1), CanCapture::Opposing(1)),
    ];
    const BISHOP_MOVESET: [CanMove<'static, StandardPiece>; 4] = [
        CanMove::Free(Move::new(1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -1, usize::MAX), CanCapture::Opposing(1)),
    ];
    const ROOK_MOVESET: [CanMove<'static, StandardPiece>; 6] = [
        CanMove::Free(Move::new(1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let mut idx = op.left(id.sign());
                while let Some(other_id) = board.get_id(&idx) {
                    if other_id.is_none() {
                        idx = idx.left(id.sign());
                        continue;
                    }
                    return Self::castle_check(
                        id,
                        history,
                        (idx.distance(op) as isize).neg(),
                        other_id,
                        StandardPiece::King,
                    );
                }
                return None;
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let mut idx = op.right(id.sign());
                while let Some(other_id) = board.get_id(&idx) {
                    if other_id.is_none() {
                        idx = idx.right(id.sign());
                        continue;
                    }
                    return Self::castle_check(
                        id,
                        history,
                        idx.distance(op) as isize,
                        other_id,
                        StandardPiece::King,
                    );
                }
                return None;
            })
        }),
    ];
    const QUEEN_MOVESET: [CanMove<'static, StandardPiece>; 8] = [
        CanMove::Free(Move::new(1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -1, usize::MAX), CanCapture::Opposing(1)),
    ];
    const KING_MOVESET: [CanMove<'static, StandardPiece>; 10] = [
        CanMove::Free(Move::new(1, 0, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 0, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, -1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -1, 1), CanCapture::Opposing(1)),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let mut idx = op.left(id.sign());
                while let Some(other_id) = board.get_id(&idx) {
                    if other_id.is_none() {
                        idx = idx.left(id.sign());
                        continue;
                    }
                    return Self::castle_check(id, history, -2, other_id, StandardPiece::Rook);
                }
                return None;
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let mut idx = op.right(id.sign());
                while let Some(other_id) = board.get_id(&idx) {
                    if other_id.is_none() {
                        idx = idx.right(id.sign());
                        continue;
                    }
                    return Self::castle_check(id, history, 2, other_id, StandardPiece::Rook);
                }
                return None;
            })
        }),
    ];

    fn castle_check<'a, P>(
        id: &PieceId<StandardPiece>,
        history: &BoardHistory,
        distance: isize,
        other: PieceId<StandardPiece>,
        piece: StandardPiece,
    ) -> Option<(Move, CanCapture<'a, P>)> {
        if other.matches(id)
            && other.piece() == piece
            && history.get_slice(id).is_none()
            && history.get_slice(&other).is_none()
        {
            return Some((Move::new(distance, 0, 1), CanCapture::None));
        }
        return None;
    }

    fn valid_moves(
        &self,
        capture_calculator: &dyn CaptureCalculator<StandardPiece>,
        piece_id: &PieceId<StandardPiece>,
        board: &dyn Board<PieceType = StandardPiece>,
        history: &BoardHistory,
    ) -> Option<Vec<usize>> {
        let mut valid = Vec::new();
        let pos = board.get_pos(piece_id)?;
        let moveset = self.moveset(&piece_id.piece());
        for can_move in moveset {
            let mut move_op = match can_move {
                CanMove::Free(m, c) => m.calculate(capture_calculator, piece_id, &pos, c, board),
                CanMove::Conditional(c) => match c(piece_id, board, history) {
                    Some((m, c)) => m.calculate(capture_calculator, piece_id, &pos, &c, board),
                    None => Vec::new(),
                },
            };
            valid.append(&mut move_op);
        }
        Some(valid)
    }
}

impl PieceSet<'static> for StandardPieceSet {
    type PieceType = StandardPiece;

    fn moveset(&self, piece: &StandardPiece) -> &[CanMove<'static, Self::PieceType>] {
        let moveset = match piece {
            StandardPiece::None => &[],
            StandardPiece::Pawn => &Self::PAWN_MOVESET[..],
            StandardPiece::Knight => &Self::KNIGHT_MOVESET[..],
            StandardPiece::Bishop => &Self::BISHOP_MOVESET[..],
            StandardPiece::Rook => &Self::ROOK_MOVESET[..],
            StandardPiece::Queen => &Self::QUEEN_MOVESET[..],
            StandardPiece::King => &Self::KING_MOVESET[..],
            StandardPiece::Unknown(_) => &[],
        };
        return moveset;
    }

    fn valid_slice(
        &self,
        capture_calculator: &dyn CaptureCalculator<StandardPiece>,
        piece_id: &PieceId<StandardPiece>,
        board: &dyn Board<PieceType = StandardPiece>,
        history: &BoardHistory,
    ) -> BoardSlice {
        BoardSlice::new(self.valid_moves(capture_calculator, piece_id, board, history))
    }
}
