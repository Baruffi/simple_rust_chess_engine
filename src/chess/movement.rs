use crate::chess::{
    board::{Board, BoardHistory},
    piece::{Piece, PieceId, PiecePos},
};

pub trait CaptureCalculator<P: Piece> {
    fn calculate(
        &self,
        can_capture: &CanCapture<'_, P>,
        id: &PieceId<P>,
        other: &PieceId<P>,
        captured: &mut usize,
    ) -> bool;
}

pub struct MobilityCalculator;

impl<P: Piece> CaptureCalculator<P> for MobilityCalculator {
    fn calculate(
        &self,
        can_capture: &CanCapture<'_, P>,
        id: &PieceId<P>,
        other: &PieceId<P>,
        captured: &mut usize,
    ) -> bool {
        match can_capture {
            CanCapture::None => other.is_none(),
            CanCapture::Matching(max) => {
                other.is_none() || (id.matches(other) && *captured < *max && (*captured += 1) == ())
            }
            CanCapture::Opposing(max) => {
                other.is_none() || (id.opposes(other) && *captured < *max && (*captured += 1) == ())
            }
            CanCapture::All(max) => other.is_none() || *captured < *max && (*captured += 1) == (),
            CanCapture::MatchingAnd(s) => {
                id.matches(other) && s(id, other, *captured) && (*captured += 1) == ()
            }
            CanCapture::OpposingAnd(s) => {
                id.opposes(other) && s(id, other, *captured) && (*captured += 1) == ()
            }
            CanCapture::AllAnd(s) => s(id, other, *captured) && (*captured += 1) == (),
        }
    }
}

pub struct PresenceCalculator;

impl<P: Piece> CaptureCalculator<P> for PresenceCalculator {
    fn calculate(
        &self,
        can_capture: &CanCapture<'_, P>,
        id: &PieceId<P>,
        other: &PieceId<P>,
        captured: &mut usize,
    ) -> bool {
        match can_capture {
            CanCapture::None => false,
            CanCapture::Matching(max) | CanCapture::Opposing(max) | CanCapture::All(max) => {
                other.is_none() || (*captured < *max && (*captured += 1) == ())
            }
            CanCapture::MatchingAnd(s) | CanCapture::OpposingAnd(s) | CanCapture::AllAnd(s) => {
                s(id, other, *captured) && (*captured += 1) == ()
            }
        }
    }
}

struct MoveStep {
    x: isize,
    y: isize,
}

pub struct Move {
    step: MoveStep,
    max_steps: usize,
}

impl Move {
    pub const fn new(step_x: isize, step_y: isize, max_steps: usize) -> Self {
        Move {
            step: MoveStep {
                x: step_x,
                y: step_y,
            },
            max_steps,
        }
    }

    pub fn calculate<P: Piece>(
        &self,
        capture_calculator: &dyn CaptureCalculator<P>,
        piece_id: &PieceId<P>,
        piece_pos: &PiecePos<P>,
        can_capture: &CanCapture<P>,
        board: &dyn Board<PieceType = P>,
    ) -> Vec<usize> {
        let mut calculated = Vec::new();
        let (px, py): (isize, isize) = piece_pos.into();
        let MoveStep { x, y } = self.step;
        let mut mx = px as isize + x;
        let mut my = py as isize + y;
        let mut iters: usize = 0;
        let mut captured: usize = 0;
        while iters < self.max_steps && PiecePos::is_inbounds(mx, my, board) {
            let from_xy = PiecePos::from((mx, my, board));
            if let Some(other) = board.get_id(&from_xy) {
                if capture_calculator.calculate(can_capture, piece_id, &other, &mut captured) {
                    calculated.push(from_xy.u());
                    mx += x;
                    my += y;
                    iters += 1;
                    continue;
                }
            }
            break;
        }
        calculated
    }
}

pub enum CanCapture<'a, P> {
    None,
    Matching(usize),
    Opposing(usize),
    All(usize),
    MatchingAnd(&'a dyn Fn(&PieceId<P>, &PieceId<P>, usize) -> bool),
    OpposingAnd(&'a dyn Fn(&PieceId<P>, &PieceId<P>, usize) -> bool),
    AllAnd(&'a dyn Fn(&PieceId<P>, &PieceId<P>, usize) -> bool),
}

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
