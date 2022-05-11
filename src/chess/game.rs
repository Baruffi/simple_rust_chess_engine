use crate::chess::{
    board::{Board, BoardHistory},
    piece::{Piece, PieceId, PiecePos, PieceSet},
    standard::board::StandardBoard,
};

use super::{
    movement::{MobilityCalculator, PresenceCalculator},
    piece::Sign,
};

pub struct Game<T, S>
where
    T: Board,
    S: PieceSet<'static>,
{
    board: T,
    positive_board_presence: T,
    negative_board_presence: T,
    overall_board_presence: T,
    history: BoardHistory,
    piece_set: S,
}

impl<
        const T_ROW_SIZE: usize,
        const T_COL_SIZE: usize,
        const T_BOARD_SIZE: usize,
        P: Piece,
        S: PieceSet<'static, PieceType = P>,
    > Game<StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>, S>
{
    pub fn new(initial_state: [isize; T_BOARD_SIZE], piece_set: S) -> Self {
        let board = StandardBoard::new(initial_state);
        let positive_board_presence = StandardBoard::new([0; T_BOARD_SIZE]);
        let negative_board_presence = StandardBoard::new([0; T_BOARD_SIZE]);
        let overall_board_presence = StandardBoard::new([0; T_BOARD_SIZE]);
        let history = BoardHistory::new(None);
        Game {
            board,
            positive_board_presence,
            negative_board_presence,
            overall_board_presence,
            history,
            piece_set,
        }
    }

    pub fn move_piece(&mut self, id: &PieceId<P>, square: usize) {
        if let Some(old_pos) = self.board.get_pos(id) {
            let pos = &PiecePos(square, &self.board);
            let old_square = old_pos.u();
            self.history.push(id, pos);
            self.board.set_square(&PieceId::default(), old_square);
            self.board.set_square(id, square);
        }
    }

    pub fn move_piece_relative(&mut self, id: &PieceId<P>, distance: usize) {
        if let Some(old_pos) = self.board.get_pos(id) {
            let old_square = old_pos.u();
            let relative_pos = old_pos.offset(id.sign(), distance);
            let relative_square = relative_pos.u();
            self.history.push(id, &relative_pos);
            self.board.set_square(&PieceId::default(), old_square);
            self.board.set_square(id, relative_square);
        }
    }

    pub fn calculate_presence(&mut self) {
        for piece in self.board.get_all_pieces() {
            for version in self.board.get_all_versions(piece, Sign::Positive) {
                let positive_slice = self.piece_set.valid_slice(
                    &mut PresenceCalculator::new(),
                    &PieceId(piece, Sign::Positive, version),
                    &self.board,
                    &self.history,
                );
                positive_slice.add(1, &mut self.positive_board_presence);
                positive_slice.add(1, &mut self.overall_board_presence);
            }
            for version in self.board.get_all_versions(piece, Sign::Negative) {
                let negative_slice = self.piece_set.valid_slice(
                    &mut PresenceCalculator::new(),
                    &PieceId(piece, Sign::Negative, version),
                    &self.board,
                    &self.history,
                );
                negative_slice.subtract(1, &mut self.negative_board_presence);
                negative_slice.subtract(1, &mut self.overall_board_presence);
            }
        }
    }

    pub fn clear_presence(&mut self) {
        self.positive_board_presence.clear();
        self.negative_board_presence.clear();
        self.overall_board_presence.clear();
    }

    pub fn clear_state(&mut self) {
        self.board.clear();
        self.history.clear();
    }

    pub fn clear_all(&mut self) {
        self.clear_presence();
        self.clear_state();
    }

    fn format_row(row: &[isize; T_ROW_SIZE]) -> String {
        let closure = |v| match v {
            &0 => ' ',
            &1 => '♟',
            &2 => '♞',
            &3 => '♝',
            &4 => '♜',
            &5 => '♛',
            &6 => '♚',
            &-1 => '♙',
            &-2 => '♘',
            &-3 => '♗',
            &-4 => '♖',
            &-5 => '♕',
            &-6 => '♔',
            _ => '?',
        };
        row.iter()
            .map(closure)
            .fold(String::from(""), |acc, v| format!("{}{}", acc, v))
    }

    fn evaluate_row(row: &[isize; T_ROW_SIZE]) -> String {
        let closure = |v| match v {
            &0 => ' ',
            &1 => '♟',
            &2 => '2',
            &3 => '♝',
            &4 => '4',
            &5 => '♜',
            &6 => '6',
            &7 => '7',
            &8 => '8',
            &9 => '♛',
            &-1 => '♙',
            &-2 => '@',
            &-3 => '♗',
            &-4 => '$',
            &-5 => '♖',
            &-6 => '^',
            &-7 => '&',
            &-8 => '*',
            &-9 => '♕',
            _ => '?',
        };
        row.iter()
            .map(closure)
            .fold(String::from(""), |acc, v| format!("{}{}", acc, v))
    }

    fn print_board(board: &StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>) {
        let top_left_corner = String::from("┌");
        let top_right_corner = String::from("┐");
        let line = String::from("─").repeat(T_ROW_SIZE);
        println!("{}{}{}", top_left_corner, line, top_right_corner);
        for i in 0..T_ROW_SIZE {
            let formatted_row = Self::format_row(&board.row(i));
            println!(" {} ", formatted_row);
        }
        let bottom_left_corner = String::from("└");
        let bottom_right_corner = String::from("┘");
        println!("{}{}{}", bottom_left_corner, line, bottom_right_corner);
    }

    fn evaluate_board(board: &StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>) {
        let top_left_corner = String::from("┌");
        let top_right_corner = String::from("┐");
        let line = String::from("─").repeat(T_ROW_SIZE);
        println!("{}{}{}", top_left_corner, line, top_right_corner);
        for i in 0..T_ROW_SIZE {
            let formatted_row = Self::evaluate_row(&board.row(i));
            println!(" {} ", formatted_row);
        }
        let bottom_left_corner = String::from("└");
        let bottom_right_corner = String::from("┘");
        println!("{}{}{}", bottom_left_corner, line, bottom_right_corner);
    }

    pub fn visualize_board(&self) {
        Self::print_board(&self.board);
    }

    pub fn visualize_presence(&self) {
        Self::evaluate_board(&self.positive_board_presence);
        Self::evaluate_board(&self.negative_board_presence);
        Self::evaluate_board(&self.overall_board_presence);
    }

    pub fn visualize_moves(&self, id: &PieceId<P>) {
        let mut mirror: StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P> =
            StandardBoard::new([0; T_BOARD_SIZE]);
        let slice =
            self.piece_set
                .valid_slice(&mut MobilityCalculator, id, &self.board, &self.history);
        slice.add(id.i(), &mut mirror);
        Self::print_board(&mirror);
    }
}
