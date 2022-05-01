use std::collections::HashMap;

const ALL_MOVES: [i8; 60] = [
    8, 16, 24, 32, 40, 48, 56, -8, -16, -24, -32, -40, -48, -56, 1, 2, 3, 4, 5, 6, 7, -1, -2, -3,
    -4, -5, -6, -7, 9, 18, 27, 36, 45, 54, 63, -9, -18, -27, -36, -45, -54, -63, 14, 21, 28, 35,
    42, 49, -14, -21, -28, -35, -42, -49, 10, 15, 17, -10, -15, -17,
];

// pawn, rook, knight, bishop, queen, king
const VALID_MOVES: [[u8; 60]; 7] = [
    [0; 60],
    [
        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 3, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 1, 1, 0, 0, 0, 0, 2, 1, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
];

enum AllPieces {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl std::ops::Index<&AllPieces> for [[u8; 60]; 7] {
    type Output = [u8; 60];

    fn index(&self, idx: &AllPieces) -> &Self::Output {
        match idx {
            AllPieces::None => &self[0],
            AllPieces::Pawn => &self[1],
            AllPieces::Rook => &self[2],
            AllPieces::Knight => &self[3],
            AllPieces::Bishop => &self[4],
            AllPieces::Queen => &self[5],
            AllPieces::King => &self[6],
        }
    }
}

impl AllPieces {
    fn valid_moves(&self) -> Vec<i8> {
        let mut valid_moves = vec![];
        for (idx, valid_move_mask) in VALID_MOVES[self].iter().enumerate() {
            if valid_move_mask > &0 {
                valid_moves.push(ALL_MOVES[idx]);
            }
        }
        valid_moves
    }
}

#[derive(Debug)]
struct PieceId(i8);

impl PieceId {
    fn piece(&self) -> AllPieces {
        match self.0.abs() {
            0 => AllPieces::None,
            1 => AllPieces::Pawn,
            2 => AllPieces::Rook,
            3 => AllPieces::Knight,
            4 => AllPieces::Bishop,
            5 => AllPieces::Queen,
            6 => AllPieces::King,
            _ => panic!("invalid piece id: {}", self.0),
        }
    }

    fn valid_moves(&self) -> Vec<i8> {
        self.piece()
            .valid_moves()
            .iter()
            .map(|x| x * self.0.signum())
            .collect()
    }
}

#[derive(Debug)]
struct PiecePos(u8);

#[derive(Debug)]
struct Board([i8; 64]);

impl Board {
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

    fn map(&self) -> BoardMap {
        BoardMap(
            (0..63)
                .collect::<Vec<u8>>()
                .into_iter()
                .zip(self.0.into_iter().filter(|a| a != &0))
                .collect::<HashMap<_, _>>(),
        )
    }

    fn set(&mut self, id: &PieceId, pos: &PiecePos) {
        self.0[pos.0 as usize] = id.0
    }

    fn set_from_slice(&mut self, id: &PieceId, slice: &BoardSlice) {
        for pos in &slice.0 {
            self.0[*pos as usize] = id.0
        }
    }

    fn set_from_map(&mut self, map: &BoardMap) {
        for (pos, id) in &map.0 {
            self.0[*pos as usize] = *id
        }
    }

    fn reset(&mut self) {
        self.0 = [0; 64]
    }
}

#[derive(Debug)]
struct BoardMap(HashMap<u8, i8>);

impl BoardMap {
    fn new(default: Option<Vec<(u8, i8)>>) -> Self {
        match default {
            Some(n) => BoardMap(n.into_iter().collect::<HashMap<_, _>>()),
            None => BoardMap(HashMap::new()),
        }
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
}

#[derive(Debug)]
struct PieceData {
    moves: BoardSlice,
    controlled_squares: BoardMap,
}

impl PieceData {
    fn calc_moves(piece_pos: &PiecePos, piece_id: &PieceId) -> BoardSlice {
        let mut board_vec: Vec<u8> = vec![];

        for m in piece_id.valid_moves() {
            let p = (
                (piece_pos.0 as i8 / 8) + (m / 8),
                (piece_pos.0 as i8 % 8) + (m % 8),
            );

            if m.abs() == 7 {
                if p.0 >= 0 && p.0 <= 7 && p.1 >= 0 && p.1 <= 7 {
                    board_vec.push((p.0 * 8 + p.1) as u8);
                } else if p.0 * 8 + p.1 > 0 && p.0 * 8 + p.1 < 63 {
                    board_vec.push((p.0 * 8 + p.1) as u8);
                }
            } else if m % 7 == 0 && m.abs() <= 49 {
                if ((p.0 < 0 || p.0 > 7) || (p.1 < 0 || p.1 > 7))
                    && p.0 * 8 + p.1 > 0
                    && p.0 * 8 + p.1 < 63
                {
                    board_vec.push((p.0 * 8 + p.1) as u8);
                }
            } else {
                if p.0 >= 0 && p.0 <= 7 && p.1 >= 0 && p.1 <= 7 {
                    board_vec.push((p.0 * 8 + p.1) as u8);
                }
            }
        }
        BoardSlice::new(Some(board_vec))
    }

    fn calc_squares(piece_pos: &PiecePos, piece_id: &PieceId) -> BoardMap {
        BoardMap::new(None)
    }

    fn new(piece_pos: &PiecePos, piece_id: &PieceId) -> Self {
        PieceData {
            moves: PieceData::calc_moves(piece_pos, piece_id),
            controlled_squares: PieceData::calc_squares(piece_pos, piece_id),
        }
    }
}

fn main() {
    let mut board = Board([0; 64]);
    let white_pawn_data = PieceData::new(&PiecePos(10), &PieceId(-1));
    let black_pawn_data = PieceData::new(&PiecePos(46), &PieceId(1));
    let white_queen_data = PieceData::new(&PiecePos(18), &PieceId(-5));
    let black_queen_data = PieceData::new(&PiecePos(54), &PieceId(5));

    println!("{:?}", white_pawn_data);
    println!("{:?}", black_pawn_data);

    println!("{:?}", white_queen_data);
    board.reset();
    board.set_from_slice(&PieceId(5), &white_queen_data.moves);
    println!("{:?}", board.row(0));
    println!("{:?}", board.row(1));
    println!("{:?}", board.row(2));
    println!("{:?}", board.row(3));
    println!("{:?}", board.row(4));
    println!("{:?}", board.row(5));
    println!("{:?}", board.row(6));
    println!("{:?}", board.row(7));

    println!("{:?}", black_queen_data);
    board.reset();
    board.set_from_slice(&PieceId(5), &black_queen_data.moves);
    println!("{:?}", board.row(0));
    println!("{:?}", board.row(1));
    println!("{:?}", board.row(2));
    println!("{:?}", board.row(3));
    println!("{:?}", board.row(4));
    println!("{:?}", board.row(5));
    println!("{:?}", board.row(6));
    println!("{:?}", board.row(7));
}
