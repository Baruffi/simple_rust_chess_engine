use crate::{
    game::Game,
    implementations::{
        board::StandardBoard,
        piece::{StandardPiece, StandardPieceSet},
    },
    piece::{PieceId, Sign},
};

mod board;
mod game;
mod implementations;
mod movement;
mod piece;

fn main() {
    let mut game = Game::<StandardBoard<8, 8, 64, StandardPiece>, StandardPieceSet>::new(
        [
            4, 2, 3, 5, 6, 3, 2, 4, //
            1, 1, 1, 1, 1, 1, 1, 1, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            -1, -1, -1, -1, -1, -1, -1, -1, //
            -4, -2, -3, -5, -6, -3, -2, -4, //
        ],
        StandardPieceSet,
    );
    let my_pawn = &PieceId(StandardPiece::Pawn, Sign::Positive, 3);
    let my_rook = &PieceId(StandardPiece::Rook, Sign::Positive, 0);
    let my_knight = &PieceId(StandardPiece::Knight, Sign::Positive, 0);
    let my_bishop = &PieceId(StandardPiece::Bishop, Sign::Positive, 0);
    let my_queen = &PieceId(StandardPiece::Queen, Sign::Positive, 0);
    let my_king = &PieceId(StandardPiece::King, Sign::Positive, 0);

    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 16);
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 8);
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 8);
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 7);
    game.print();
    println!("");
    game.visualize_moves(my_knight);
    println!("");
    game.move_piece_relative(my_knight, 15);
    game.print();
    println!("");
    game.visualize_moves(my_bishop);
    println!("");
    game.move_piece_relative(my_bishop, 27);
    game.print();
    println!("");
    game.visualize_moves(my_queen);
    game.move_piece_relative(my_queen, 8);
    println!("");
    game.print();
    println!("");
    game.visualize_moves(my_king);
    println!("");
    game.move_piece(my_king, 2);
    game.move_piece(my_rook, 3);
    game.print();
    println!("");
    game.visualize_moves(my_rook);
    game.move_piece_relative(my_rook, 1);
    println!("");
    game.print();
    println!("");
    game.clear();
    game.print();
}
