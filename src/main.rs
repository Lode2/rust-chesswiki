#![allow(dead_code, unused_imports)]
// declare all the modules for the crate
mod chess;
mod move_gen;

use structs_methods::State;

// use the chess_pos.rs file and access the Debug trait
use crate::chess::structs::BitBoard;
use crate::chess::structs::Position;
use crate::chess::structs_methods;
use crate::structs_methods::Debug;

fn main() {
    // let starting_pos_fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    // let mut my_pos: Position = Position {
    //     bb_sides: [BitBoard(0); 2],
    //     bb_pieces: [[BitBoard(0); 6]; 2],
    //     state: State {
    //         stm: 0,
    //         castling_rights: 0b00001111,
    //         en_passant_square: None,
    //         half_move_counter: 0,
    //         full_move_counter: 0,
    //     },
    // };
    // my_pos.load(starting_pos_fen);

    // println!("Pretty Starting Position:{}", my_pos.pretty());

    // // get the bitboard with all the white queens
    // let white_queens: BitBoard =
    //     my_pos.bb_pieces[structs_methods::Sides::WHITE][structs_methods::Pieces::QUEEN];

    // println!("All the white queens:{}", white_queens.pretty());

    // let example_game_fen: String =
    //     String::from("r4rk1/pbqn1pp1/1pn1p2p/2ppP2Q/3P4/2PBP1B1/PP1N2PP/R4RK1 w - - 0 14");

    // my_pos.load(&example_game_fen);

    // println!("Loaded example position:{}", my_pos.pretty());

    // // get the bitboard with all the white pawns
    // let white_pawns: BitBoard =
    //     my_pos.bb_pieces[structs_methods::Sides::WHITE][structs_methods::Pieces::PAWN];

    // println!("All the white pawns:{}", white_pawns.pretty());

    // let mut test_starting_pos = Position {
    //     ..Position::starting_pos()
    // };

    // println!(
    //     "testing new default option for position struct:{}",
    //     test_starting_pos.pretty()
    // );

    // println!(
    //     "The white pieces are: (color,piece_id,square_idx)\n{:?}",
    //     test_starting_pos.get_pieces(0)
    // );

    // println!(
    //     "The moves in the position are:\n{:?}",
    //     test_starting_pos.moves()
    // );

    // let my_test_int = 0b11100111;
    // // println!("my test int:{:#b}", my_test_int);
    // // println!("my test int:{:#b}", my_test_int >> 2);
    // println!("my test int:{:#b}", my_test_int);

    // let mut move_testing_position = Position {
    //     ..Position::empty_pos()
    // };

    // move_testing_position.put_piece(5, 1, 60);
    // move_testing_position.put_piece(3, 1, 56);

    // println!("{}", move_testing_position.pretty());

    // test_starting_pos.remove_piece(7);

    // println!("{}", test_starting_pos.pretty());

    let mut test_starting_pos = Position {
        ..Position::starting_pos()
    };
    test_starting_pos.put_piece(2, 0, 18);
    // test_starting_pos.put_piece(2, 1, 17);
    test_starting_pos.put_piece(2, 1, 23);
    test_starting_pos.put_piece(2, 1, 22);
    test_starting_pos.remove_piece(48);
    test_starting_pos.remove_piece(56);
    test_starting_pos.put_piece(0, 0, 48);

    println!("{}", test_starting_pos.pretty());

    println!("{:?}", test_starting_pos.moves());
}
// article about Gigantua, fastest move generator
// https://www.codeproject.com/Articles/5313417/Worlds-Fastest-Bitboard-Chess-Movegenerator

// explaining what to keep in mind when making a move generator
// https://peterellisjones.com/posts/generating-legal-chess-moves-efficiently/
