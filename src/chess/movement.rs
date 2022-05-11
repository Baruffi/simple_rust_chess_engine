use crate::chess::{
    board::{Board, BoardHistory},
    piece::{Piece, PieceId, PiecePos},
};

#[derive(PartialEq)]
pub enum CalculationResult {
    Free(usize),
    Captured(usize),
    Blocked(usize),
}

pub enum InterpretationResult {
    Continue,
    BreakBefore,
    BreakAfter,
}

pub trait CaptureInterpreter {
    fn clear_internal_states(&mut self);
    fn interpret(&mut self, calculation_result: CalculationResult) -> InterpretationResult;
}

pub struct MobilityCalculator;

impl CaptureInterpreter for MobilityCalculator {
    fn clear_internal_states(&mut self) {}

    fn interpret(&mut self, calculation_result: CalculationResult) -> InterpretationResult {
        match calculation_result {
            CalculationResult::Free(_) => InterpretationResult::Continue,
            CalculationResult::Captured(_) => InterpretationResult::Continue,
            CalculationResult::Blocked(_) => InterpretationResult::BreakBefore,
        }
    }
}

pub struct PresenceCalculator(pub CalculationResult);

impl PresenceCalculator {
    pub fn new() -> Self {
        PresenceCalculator(CalculationResult::Free(1))
    }
}

impl CaptureInterpreter for PresenceCalculator {
    fn clear_internal_states(&mut self) {
        self.0 = CalculationResult::Free(1);
    }

    fn interpret(&mut self, calculation_result: CalculationResult) -> InterpretationResult {
        let result = match calculation_result {
            CalculationResult::Free(max) => {
                if max == 0 {
                    return InterpretationResult::BreakBefore;
                }
                return InterpretationResult::Continue;
            }
            CalculationResult::Captured(_) => InterpretationResult::Continue,
            CalculationResult::Blocked(max) => {
                if max == 0 {
                    return InterpretationResult::BreakBefore;
                }
                if let CalculationResult::Captured(_) = self.0 {
                    return InterpretationResult::BreakBefore;
                }
                return InterpretationResult::BreakAfter;
            }
        };
        self.0 = calculation_result;
        result
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
        capture_interpreter: &mut dyn CaptureInterpreter,
        piece_id: &PieceId<P>,
        piece_pos: &PiecePos<P>,
        can_capture: &CanCapture<P>,
        board: &dyn Board<PieceType = P>,
    ) -> Vec<usize> {
        capture_interpreter.clear_internal_states();
        let mut calculated = Vec::new();
        let (px, py): (isize, isize) = piece_pos.into();
        let MoveStep { x, y } = self.step;
        let mut mx = px as isize + x * piece_id.sign();
        let mut my = py as isize + y * piece_id.sign();
        let mut iters: usize = 0;
        let mut captured: usize = 0;
        while iters < self.max_steps && PiecePos::is_inbounds(mx, my, board) {
            let from_xy = PiecePos::from((mx, my, board));
            if let Some(other) = board.get_id(&from_xy) {
                let calculation_result = can_capture.calculate(piece_id, &other, &mut captured);
                match capture_interpreter.interpret(calculation_result) {
                    InterpretationResult::Continue => {
                        calculated.push(from_xy.u());
                        mx += x;
                        my += y;
                        iters += 1;
                        continue;
                    }
                    InterpretationResult::BreakAfter => {
                        calculated.push(from_xy.u());
                    }
                    InterpretationResult::BreakBefore => {}
                }
            }
            break;
        }
        calculated
    }
}

pub enum CanCapture<'a, P: Piece> {
    None,
    Matching(usize),
    Opposing(usize),
    All(usize),
    Specific(&'a dyn Fn(&PieceId<P>, &PieceId<P>, &mut usize) -> CalculationResult),
}

impl<'a, P: Piece> CanCapture<'a, P> {
    pub fn calculate(
        &self,
        id: &PieceId<P>,
        other: &PieceId<P>,
        captured: &mut usize,
    ) -> CalculationResult {
        match self {
            CanCapture::None => {
                if other.is_none() {
                    return CalculationResult::Free(0);
                }
                return CalculationResult::Blocked(0);
            }
            CanCapture::Matching(max) => {
                if other.is_none() {
                    return CalculationResult::Free(*max);
                }
                if id.matches(other) && *captured < *max {
                    *captured += 1;
                    return CalculationResult::Captured(*max);
                }
                return CalculationResult::Blocked(*max);
            }
            CanCapture::Opposing(max) => {
                if other.is_none() {
                    return CalculationResult::Free(*max);
                }
                if id.opposes(other) && *captured < *max {
                    *captured += 1;
                    return CalculationResult::Captured(*max);
                }
                return CalculationResult::Blocked(*max);
            }
            CanCapture::All(max) => {
                if other.is_none() {
                    return CalculationResult::Free(*max);
                }
                if *captured < *max {
                    *captured += 1;
                    return CalculationResult::Captured(*max);
                }
                return CalculationResult::Blocked(*max);
            }
            CanCapture::Specific(s) => s(id, other, captured),
        }
    }
}

pub enum CanMove<'a, P: Piece> {
    Free(Move, CanCapture<'a, P>),
    Conditional(
        &'a dyn Fn(
            &PieceId<P>,
            &dyn Board<PieceType = P>,
            &BoardHistory,
        ) -> Option<(Move, CanCapture<'a, P>)>,
    ),
}
