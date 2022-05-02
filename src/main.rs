use std::collections::HashMap;

enum Move {
    Beam((i8, i8), u8),
    Jump(i8, i8),
}

impl Move {
    fn calculate(&self, piece_id: &PieceId, piece_pos: &PiecePos, board: &Board) -> Vec<u8> {
        let mut calculated = vec![];
        let (px, py) = piece_pos.xy();
        match self {
            Move::Beam(step, max) => {
                let (x, y) = step;
                let mut mx = px as i8 + x;
                let mut my = py as i8 + y;
                let mut iters: u8 = 0;

                while &iters < max && PiecePos::is_inbounds(mx, my) {
                    let from_xy = PiecePos::from_xy(mx as u8, my as u8);
                    let should_break_late = match board.get_id(&from_xy) {
                        Some(p) => {
                            if p.matches(piece_id) {
                                break;
                            }
                            true
                        }
                        None => false,
                    };
                    calculated.push(from_xy.0);

                    if should_break_late {
                        break;
                    }

                    mx += x;
                    my += y;
                    iters += 1;
                }
            }
            Move::Jump(x, y) => {
                let mx = px as i8 + x;
                let my = py as i8 + y;
                if PiecePos::is_inbounds(mx, my) {
                    let from_xy = PiecePos::from_xy(mx as u8, my as u8);
                    match board.get_id(&from_xy) {
                        Some(p) => {
                            if p.opposes(piece_id) {
                                calculated.push(from_xy.0);
                            }
                        }
                        None => {
                            calculated.push(from_xy.0);
                        }
                    }
                }
            }
        };
        calculated
    }
}

enum CanMove<'a> {
    Free(Move),
    Conditional(&'a dyn Fn(&PieceId, &Board, &History) -> Option<Move>),
}

const PAWN_MOVESET: [CanMove; 6] = [
    CanMove::Conditional(&|id, board, _| {
        board.get_pos(id).and_then(|op| {
            let idx = op.top(id.sign());
            if board.get_id(&idx).is_none() {
                return Some(Move::Beam((0, 1), 1));
            }
            return None;
        })
    }),
    CanMove::Conditional(&|id, _, history| {
        if history.get_slice(id).is_none() {
            return Some(Move::Beam((0, 1), 2));
        }
        return None;
    }),
    CanMove::Conditional(&|id, board, _| {
        board.get_pos(id).and_then(|op| {
            let idx = op.topleft(id.sign());
            board.get_id(&idx).and_then(|other| {
                if other.opposes(id) {
                    return Some(Move::Beam((-1, 1), 1));
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
                    return Some(Move::Beam((1, 1), 1));
                }
                return None;
            })
        })
    }),
    CanMove::Conditional(&|id, board, history| {
        board.get_pos(id).and_then(|op| {
            let op_left = op.left(id.sign());
            board.get_id(&op_left).and_then(|other| {
                if other.opposes(id) && other.piece() == AllPieces::Pawn {
                    return history.get_slice(&other).and_then(|prev| {
                        prev.0.last().and_then(|last| {
                            if last == &op_left.bottom(other.sign()).bottom(other.sign()).0 {
                                return Some(Move::Beam((-1, 1), 1));
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
                if other.opposes(id) && other.piece() == AllPieces::Pawn {
                    return history.get_slice(&other).and_then(|prev| {
                        prev.0.last().and_then(|last| {
                            if last == &op_right.bottom(other.sign()).bottom(other.sign()).0 {
                                return Some(Move::Beam((1, 1), 1));
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
const KNIGHT_MOVESET: [CanMove; 8] = [
    CanMove::Free(Move::Jump(1, 2)),
    CanMove::Free(Move::Jump(-1, 2)),
    CanMove::Free(Move::Jump(1, -2)),
    CanMove::Free(Move::Jump(-1, -2)),
    CanMove::Free(Move::Jump(2, 1)),
    CanMove::Free(Move::Jump(-2, 1)),
    CanMove::Free(Move::Jump(2, -1)),
    CanMove::Free(Move::Jump(-2, -1)),
];
const BISHOP_MOVESET: [CanMove; 4] = [
    CanMove::Free(Move::Beam((1, 1), 7)),
    CanMove::Free(Move::Beam((-1, 1), 7)),
    CanMove::Free(Move::Beam((1, -1), 7)),
    CanMove::Free(Move::Beam((-1, -1), 7)),
];
const ROOK_MOVESET: [CanMove; 6] = [
    CanMove::Free(Move::Beam((1, 0), 7)),
    CanMove::Free(Move::Beam((-1, 0), 7)),
    CanMove::Free(Move::Beam((0, 1), 7)),
    CanMove::Free(Move::Beam((0, -1), 7)),
    CanMove::Conditional(&|id, board, history| {
        board.get_pos(id).and_then(|op| {
            let idx1 = op.left(id.sign());
            let idx2 = idx1.left(id.sign());
            let idx3 = idx2.left(id.sign());
            let idx4 = idx3.left(id.sign());
            if board.get_id(&idx1).is_none() && board.get_id(&idx2).is_none() {
                return board
                    .get_id(&idx3)
                    .and_then(|other| {
                        if other.matches(id)
                            && other.piece() == AllPieces::King
                            && history.get_slice(id).is_none()
                            && history.get_slice(&other).is_none()
                        {
                            return Some(Move::Jump(-2, 0));
                        }
                        return None;
                    })
                    .or_else(|| {
                        board.get_id(&idx4).and_then(|other| {
                            if other.matches(id)
                                && other.piece() == AllPieces::King
                                && history.get_slice(id).is_none()
                                && history.get_slice(&other).is_none()
                            {
                                return Some(Move::Jump(-2, 0));
                            }
                            return None;
                        })
                    });
            }
            return None;
        })
    }),
    CanMove::Conditional(&|id, board, history| {
        board.get_pos(id).and_then(|op| {
            let idx1 = op.right(id.sign());
            let idx2 = idx1.right(id.sign());
            let idx3 = idx2.right(id.sign());
            let idx4 = idx3.right(id.sign());
            if board.get_id(&idx1).is_none() && board.get_id(&idx2).is_none() {
                return board
                    .get_id(&idx3)
                    .and_then(|other| {
                        if other.matches(id)
                            && other.piece() == AllPieces::King
                            && history.get_slice(id).is_none()
                            && history.get_slice(&other).is_none()
                        {
                            return Some(Move::Jump(-2, 0));
                        }
                        return None;
                    })
                    .or_else(|| {
                        board.get_id(&idx4).and_then(|other| {
                            if other.matches(id)
                                && other.piece() == AllPieces::King
                                && history.get_slice(id).is_none()
                                && history.get_slice(&other).is_none()
                            {
                                return Some(Move::Jump(2, 0));
                            }
                            return None;
                        })
                    });
            }
            return None;
        })
    }),
];
const QUEEN_MOVESET: [CanMove; 8] = [
    CanMove::Free(Move::Beam((1, 0), 7)),
    CanMove::Free(Move::Beam((-1, 0), 7)),
    CanMove::Free(Move::Beam((0, 1), 7)),
    CanMove::Free(Move::Beam((0, -1), 7)),
    CanMove::Free(Move::Beam((1, 1), 7)),
    CanMove::Free(Move::Beam((-1, 1), 7)),
    CanMove::Free(Move::Beam((1, -1), 7)),
    CanMove::Free(Move::Beam((-1, -1), 7)),
];
const KING_MOVESET: [CanMove; 10] = [
    CanMove::Free(Move::Beam((1, 0), 1)),
    CanMove::Free(Move::Beam((-1, 0), 1)),
    CanMove::Free(Move::Beam((0, 1), 1)),
    CanMove::Free(Move::Beam((0, -1), 1)),
    CanMove::Free(Move::Beam((1, 1), 1)),
    CanMove::Free(Move::Beam((-1, 1), 1)),
    CanMove::Free(Move::Beam((1, -1), 1)),
    CanMove::Free(Move::Beam((-1, -1), 1)),
    CanMove::Conditional(&|id, board, history| {
        board.get_pos(id).and_then(|op| {
            let idx1 = op.left(id.sign());
            let idx2 = idx1.left(id.sign());
            let idx3 = idx2.left(id.sign());
            let idx4 = idx3.left(id.sign());
            if board.get_id(&idx1).is_none() && board.get_id(&idx2).is_none() {
                return board
                    .get_id(&idx3)
                    .and_then(|other| {
                        if other.matches(id)
                            && other.piece() == AllPieces::Rook
                            && history.get_slice(id).is_none()
                            && history.get_slice(&other).is_none()
                        {
                            return Some(Move::Jump(-2, 0));
                        }
                        return None;
                    })
                    .or_else(|| {
                        board.get_id(&idx4).and_then(|other| {
                            if other.matches(id)
                                && other.piece() == AllPieces::Rook
                                && history.get_slice(id).is_none()
                                && history.get_slice(&other).is_none()
                            {
                                return Some(Move::Jump(-2, 0));
                            }
                            return None;
                        })
                    });
            }
            return None;
        })
    }),
    CanMove::Conditional(&|id, board, history| {
        board.get_pos(id).and_then(|op| {
            let idx1 = op.right(id.sign());
            let idx2 = idx1.right(id.sign());
            let idx3 = idx2.right(id.sign());
            let idx4 = idx3.right(id.sign());
            if board.get_id(&idx1).is_none() && board.get_id(&idx2).is_none() {
                return board
                    .get_id(&idx3)
                    .and_then(|other| {
                        if other.matches(id)
                            && other.piece() == AllPieces::Rook
                            && history.get_slice(id).is_none()
                            && history.get_slice(&other).is_none()
                        {
                            return Some(Move::Jump(-2, 0));
                        }
                        return None;
                    })
                    .or_else(|| {
                        board.get_id(&idx4).and_then(|other| {
                            if other.matches(id)
                                && other.piece() == AllPieces::Rook
                                && history.get_slice(id).is_none()
                                && history.get_slice(&other).is_none()
                            {
                                return Some(Move::Jump(2, 0));
                            }
                            return None;
                        })
                    });
            }
            return None;
        })
    }),
];

#[derive(PartialEq, PartialOrd)]
enum AllPieces {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl AllPieces {
    fn to_id(&self, sign: i8) -> i8 {
        match self {
            AllPieces::None => 0,
            AllPieces::Pawn => 1 * sign,
            AllPieces::Knight => 2 * sign,
            AllPieces::Bishop => 3 * sign,
            AllPieces::Rook => 4 * sign,
            AllPieces::Queen => 5 * sign,
            AllPieces::King => 6 * sign,
        }
    }

    fn moveset(&self) -> &[CanMove] {
        let moveset = match self {
            AllPieces::None => {
                let moveset: &[CanMove] = &[];
                moveset
            }
            AllPieces::Pawn => &PAWN_MOVESET[..],
            AllPieces::Knight => &KNIGHT_MOVESET[..],
            AllPieces::Bishop => &BISHOP_MOVESET[..],
            AllPieces::Rook => &ROOK_MOVESET[..],
            AllPieces::Queen => &QUEEN_MOVESET[..],
            AllPieces::King => &KING_MOVESET[..],
        };
        return moveset;
    }
}

#[derive(Debug)]
struct PieceId(i8, usize);

impl PieceId {
    fn sign(&self) -> i8 {
        self.0.signum()
    }

    fn matches(&self, other: &PieceId) -> bool {
        self.sign() == other.sign()
    }

    fn opposes(&self, other: &PieceId) -> bool {
        self.sign() == -other.sign()
    }

    fn piece(&self) -> AllPieces {
        match self.0.abs() {
            0 => AllPieces::None,
            1 => AllPieces::Pawn,
            2 => AllPieces::Knight,
            3 => AllPieces::Bishop,
            4 => AllPieces::Rook,
            5 => AllPieces::Queen,
            6 => AllPieces::King,
            _ => panic!("invalid piece id: {}", self.0),
        }
    }

    fn valid_moves(&self, board: &Board, history: &History) -> Option<Vec<u8>> {
        let mut valid = vec![];
        let pos = board.get_pos(self)?;
        for can_move in self.piece().moveset() {
            let mut move_op = match can_move {
                CanMove::Free(m) => m.calculate(self, &pos, board),
                CanMove::Conditional(c) => match c(self, board, history) {
                    Some(m) => m.calculate(self, &pos, board),
                    None => vec![],
                },
            };
            valid.append(&mut move_op);
        }
        Some(valid)
    }

    fn valid_slice(&self, board: &Board, history: &History) -> BoardSlice {
        BoardSlice::new(self.valid_moves(board, history))
    }
}

#[derive(Debug)]
struct PiecePos(u8);

impl PiecePos {
    fn is_inbounds(x: i8, y: i8) -> bool {
        return x >= 0 && x <= 7 && y >= 0 && y <= 7;
    }

    fn from_xy(x: u8, y: u8) -> Self {
        PiecePos(x + y * 8)
    }

    fn xy(&self) -> (u8, u8) {
        (self.0 % 8, self.0 / 8)
    }

    fn from_usize(u: usize) -> Self {
        PiecePos(u as u8)
    }

    fn usize(&self) -> usize {
        self.0 as usize
    }

    fn top(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 + (sign * 8)) as u8)
    }

    fn bottom(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 - (sign * 8)) as u8)
    }

    fn left(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 - sign) as u8)
    }

    fn right(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 + sign) as u8)
    }

    fn topleft(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 + (sign * 8) - 1) as u8)
    }

    fn topright(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 + (sign * 8) + 1) as u8)
    }

    fn bottomleft(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 - (sign * 8) - 1) as u8)
    }

    fn bottomright(&self, sign: i8) -> Self {
        PiecePos((self.0 as i8 - (sign * 8) + 1) as u8)
    }

    fn add(&self, sign: i8, other: &PiecePos) -> Self {
        PiecePos((self.0 as i8 + sign * other.0 as i8) as u8)
    }
}

#[derive(Debug, Clone)]
struct Board([i8; 64], HashMap<i8, Vec<usize>>);

impl Board {
    fn new(initial_state: [i8; 64]) -> Self {
        let mut repeats: HashMap<i8, Vec<usize>> = HashMap::new();
        let mut last_seen_pos: HashMap<i8, usize> = HashMap::new();
        for (pos, id) in initial_state.iter().enumerate() {
            if id == &0 {
                continue;
            }
            match last_seen_pos.insert(*id, pos) {
                Some(v) => {
                    if repeats.contains_key(id) {
                        let existing_vec = repeats.get_mut(id).unwrap();
                        existing_vec.push(pos);
                    } else {
                        repeats.insert(*id, vec![v, pos]);
                    }
                }
                None => (),
            }
        }
        Board(initial_state, repeats)
    }

    fn row(&self, row: usize) -> [i8; 8] {
        assert!(row < 8, "the board only has 8 rows");
        self.0[row * 8..row * 8 + 8]
            .try_into()
            .expect("unexpected slice length")
    }

    fn col(&self, col: usize) -> [i8; 8] {
        assert!(col < 8, "the board only has 8 columns");
        [
            self.0[col],
            self.0[col + 8],
            self.0[col + 16],
            self.0[col + 24],
            self.0[col + 32],
            self.0[col + 40],
            self.0[col + 48],
            self.0[col + 56],
        ]
    }

    fn piece_slice(&self, ids: &[PieceId]) -> BoardSlice {
        BoardSlice(
            (0..63)
                .collect::<Vec<u8>>()
                .into_iter()
                .filter(|v| ids.iter().any(|id| self.0[*v as usize] == id.0))
                .collect::<Vec<_>>(),
        )
    }

    fn get_id(&self, pos: &PiecePos) -> Option<PieceId> {
        if pos.0 > 64 {
            return None;
        }
        let id = self.0[pos.usize()];
        if id == 0 {
            return None;
        }
        self.1
            .get(&id)
            .and_then(|repeats| {
                for (version, existing_pos) in repeats.iter().enumerate() {
                    if &pos.usize() == existing_pos {
                        return Some(PieceId(id, version));
                    }
                }
                return None;
            })
            .or(Some(PieceId(id, 0)))
    }

    fn get_pos(&self, id: &PieceId) -> Option<PiecePos> {
        self.1
            .get(&id.0)
            .and_then(|versions| {
                versions.get(id.1).and_then(|specific| {
                    return Some(PiecePos(*specific as u8));
                })
            })
            .or_else(|| {
                for (idx, v) in self.0.iter().enumerate() {
                    if v == &id.0 {
                        return Some(PiecePos(idx as u8));
                    }
                }
                return None;
            })
    }

    fn set_square(&mut self, id: &PieceId, pos: &PiecePos) {
        if self.1.contains_key(&id.0) {
            self.1.get_mut(&id.0).unwrap()[id.1] = pos.0 as usize;
        }
        self.0[pos.usize()] = id.0;
    }

    fn clear(&mut self) {
        self.0 = [0; 64];
        self.1 = HashMap::new();
    }
}

#[derive(Debug)]
struct History(HashMap<(i8, usize), BoardSlice>);

impl History {
    fn new(default: Option<HashMap<(i8, usize), BoardSlice>>) -> Self {
        match default {
            Some(n) => History(n),
            None => History(HashMap::new()),
        }
    }

    fn get_slice(&self, id: &PieceId) -> Option<&BoardSlice> {
        self.0.get(&(id.0, id.1))
    }

    fn push(&mut self, id: &PieceId, pos: &PiecePos) {
        let old_slice = self.0.get(&(id.0, id.1));
        match old_slice {
            Some(slice) => {
                let mut new_slice = BoardSlice::new(Some(slice.0.to_vec()));
                new_slice.push(pos.0);
                self.0.insert((id.0, id.1), new_slice);
            }
            None => {
                let new_slice = BoardSlice::new(Some(vec![pos.0]));
                self.0.insert((id.0, id.1), new_slice);
            }
        }
    }

    fn clear(&mut self) {
        self.0 = HashMap::new();
    }
}

#[derive(Debug)]
struct BoardSlice(Vec<u8>);

impl BoardSlice {
    fn new(default: Option<Vec<u8>>) -> Self {
        match default {
            Some(n) => BoardSlice(n),
            None => BoardSlice(Vec::new()),
        }
    }

    fn push(&mut self, pos: u8) {
        self.0.push(pos);
    }

    fn visualize(&self, fill: i8) -> Board {
        let mut visual = Board::new([0; 64]);
        for v in &self.0 {
            visual.0[*v as usize] = fill;
        }
        return visual;
    }
}

struct Game {
    board: Board,
    history: History,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::new([
                AllPieces::Rook.to_id(1),
                AllPieces::Knight.to_id(1),
                AllPieces::Bishop.to_id(1),
                AllPieces::Queen.to_id(1),
                AllPieces::King.to_id(1),
                AllPieces::Bishop.to_id(1),
                AllPieces::Knight.to_id(1),
                AllPieces::Rook.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::Pawn.to_id(1),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::None.to_id(0),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Pawn.to_id(-1),
                AllPieces::Rook.to_id(-1),
                AllPieces::Knight.to_id(-1),
                AllPieces::Bishop.to_id(-1),
                AllPieces::Queen.to_id(-1),
                AllPieces::King.to_id(-1),
                AllPieces::Bishop.to_id(-1),
                AllPieces::Knight.to_id(-1),
                AllPieces::Rook.to_id(-1),
            ]),
            history: History::new(None),
        }
    }

    fn move_piece(&mut self, id: &PieceId, pos: &PiecePos) {
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            self.history.push(id, pos);
            self.board.set_square(&PieceId(0, 0), &old_pos);
            self.board.set_square(id, pos);
        }
    }

    fn move_piece_relative(&mut self, id: &PieceId, pos: &PiecePos) {
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            let relative_pos = old_pos.add(id.sign(), pos);
            self.history.push(id, &relative_pos);
            self.board.set_square(&PieceId(0, 0), &old_pos);
            self.board.set_square(id, &relative_pos);
        }
    }

    fn clear(&mut self) {
        self.board.clear();
        self.history.clear();
    }

    fn print(&self) {
        println!("{:?}", self.board.row(0));
        println!("{:?}", self.board.row(1));
        println!("{:?}", self.board.row(2));
        println!("{:?}", self.board.row(3));
        println!("{:?}", self.board.row(4));
        println!("{:?}", self.board.row(5));
        println!("{:?}", self.board.row(6));
        println!("{:?}", self.board.row(7));
    }

    fn visualize_moves(&self, id: &PieceId) {
        let mirror = id.valid_slice(&self.board, &self.history).visualize(id.0);
        println!("{:?}", mirror.row(0));
        println!("{:?}", mirror.row(1));
        println!("{:?}", mirror.row(2));
        println!("{:?}", mirror.row(3));
        println!("{:?}", mirror.row(4));
        println!("{:?}", mirror.row(5));
        println!("{:?}", mirror.row(6));
        println!("{:?}", mirror.row(7));
    }
}

fn main() {
    let mut game = Game::new();
    let my_pawn = &PieceId(AllPieces::Pawn.to_id(1), 3);
    let my_rook = &PieceId(AllPieces::Rook.to_id(1), 0);
    let my_knight = &PieceId(AllPieces::Knight.to_id(1), 0);
    let my_bishop = &PieceId(AllPieces::Bishop.to_id(1), 0);
    let my_queen = &PieceId(AllPieces::Queen.to_id(1), 0);
    let my_king = &PieceId(AllPieces::King.to_id(1), 0);

    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, &PiecePos(16));
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, &PiecePos(8));
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, &PiecePos(8));
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, &PiecePos(7));
    game.print();
    println!("");
    game.visualize_moves(my_knight);
    println!("");
    game.move_piece_relative(my_knight, &PiecePos(15));
    game.print();
    println!("");
    game.visualize_moves(my_bishop);
    println!("");
    game.move_piece_relative(my_bishop, &PiecePos(27));
    game.print();
    println!("");
    game.visualize_moves(my_queen);
    game.move_piece_relative(my_queen, &PiecePos(8));
    println!("");
    game.print();
    println!("");
    game.visualize_moves(my_king);
    println!("");
    game.move_piece(my_king, &PiecePos(2));
    game.move_piece(my_rook, &PiecePos(3));
    game.print();
    println!("");
    game.visualize_moves(my_rook);
    game.move_piece_relative(my_rook, &PiecePos(1));
    println!("");
    game.print();
    println!("");
    game.clear();
    game.print();
}
