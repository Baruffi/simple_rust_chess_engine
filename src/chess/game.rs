use crate::chess::{
    board::{Board, BoardHistory},
    piece::{Piece, PieceId, PiecePos, PieceSet},
    standard::board::StandardBoard,
};

pub struct Game<T, S>
where
    T: Board,
    S: PieceSet<'static>,
{
    board: T,
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
        Game {
            board: StandardBoard::new(initial_state),
            history: BoardHistory::new(None),
            piece_set,
        }
    }

    pub fn move_piece(&mut self, id: &PieceId<P>, square: usize) {
        let pos = &PiecePos(square, &self.board);
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            let old_square = old_pos.u();
            self.history.push(id, pos);
            self.board.set_square(&PieceId::default(), old_square);
            self.board.set_square(id, square);
        }
    }

    pub fn move_piece_relative(&mut self, id: &PieceId<P>, distance: usize) {
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            let old_square = old_pos.u();
            let relative_pos = old_pos.offset(id.sign(), distance);
            let relative_square = relative_pos.u();
            self.history.push(id, &relative_pos);
            self.board.set_square(&PieceId::default(), old_square);
            self.board.set_square(id, relative_square);
        }
    }

    pub fn clear(&mut self) {
        self.board.clear();
        self.history.clear();
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
            _ => panic!("illegal state"),
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

    pub fn visualize_board(&self) {
        Self::print_board(&self.board);
    }

    pub fn visualize_moves(&self, id: &PieceId<P>) {
        let slice = self.piece_set.valid_slice(id, &self.board, &self.history);
        let mirror: StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P> =
            slice.visualize(id.i());
        Self::print_board(&mirror);
    }
}
