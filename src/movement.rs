use crate::{
    board::{Board, BoardHistory},
    piece::{Piece, PieceId, PiecePos},
};

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
            match board.get_id(&from_xy) {
                Some(p) => {
                    if !can_capture.check(piece_id, &p, &mut captured) {
                        break;
                    }
                }
                None => (),
            };
            calculated.push(from_xy.u());

            mx += x;
            my += y;
            iters += 1;
        }
        calculated
    }
}

pub enum CanCapture<'a, P> {
    None,
    Matching(usize),
    Opposing(usize),
    Specific(&'a dyn Fn(&PieceId<P>, &PieceId<P>, &mut usize) -> bool),
    All,
}

impl<'a, P: Piece> CanCapture<'a, P> {
    pub fn check(&self, id: &PieceId<P>, other: &PieceId<P>, captured: &mut usize) -> bool {
        match self {
            CanCapture::None => other.is_none(),
            CanCapture::Matching(max) => {
                other.is_none() || (*captured < *max && (*captured += 1) == () && id.matches(other))
            }
            CanCapture::Opposing(max) => {
                other.is_none() || (*captured < *max && (*captured += 1) == () && id.opposes(other))
            }
            CanCapture::Specific(s) => s(id, other, captured),
            CanCapture::All => true,
        }
    }
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
