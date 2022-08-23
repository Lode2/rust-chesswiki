mod chess_pos;

fn main() {
    let starting_pos_fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let mut my_pos: chess_pos::Position = chess_pos::Position {
        bb_sides: [chess_pos::BitBoard(0); 2],
        bb_pieces: [[chess_pos::BitBoard(0); 6]; 2],
    };
    my_pos.load(starting_pos_fen);

    println!("Pretty Starting Position:{}", my_pos.pretty());

    // get the bitboard with all the white queens
    let white_queens: chess_pos::BitBoard =
        my_pos.bb_pieces[chess_pos::Sides::WHITE][chess_pos::Pieces::QUEEN];

    println!("All the white queens:{}", white_queens.pretty());

    let example_game_fen: String =
        String::from("r4rk1/pbqn1pp1/1pn1p2p/2ppP2Q/3P4/2PBP1B1/PP1N2PP/R4RK1 w - - 0 14");

    my_pos.load(&example_game_fen);

    println!("Loaded example position:{}", my_pos.pretty());

    // get the bitboard with all the white pawns
    let white_pawns: chess_pos::BitBoard =
        my_pos.bb_pieces[chess_pos::Sides::WHITE][chess_pos::Pieces::PAWN];

    println!("All the white pawns:{}", white_pawns.pretty());

    let test_starting_pos = chess_pos::Position {
        ..chess_pos::Position::starting_pos()
    };

    println!(
        "testing new default option for position struct:{}",
        test_starting_pos.pretty()
    );

    println!(
        "The white pieces are: (color,piece_id,square_idx)\n{:?}",
        test_starting_pos.get_pieces(0)
    );

    println!(
        "The moves in the position are:\n{:?}",
        test_starting_pos.moves()
    );

    let my_test_int = 0b11100111;
    // println!("my test int:{:#b}", my_test_int);
    // println!("my test int:{:#b}", my_test_int >> 2);
    println!("my test int:{:#b}", my_test_int);

    let mut move_testing_position = chess_pos::Position {
        ..chess_pos::Position::empty_pos()
    };

    move_testing_position.put_piece(3, 0, 14);

    println!("{}", move_testing_position.pretty());
}
// article about Gigantua, fastest move generator
// https://www.codeproject.com/Articles/5313417/Worlds-Fastest-Bitboard-Chess-Movegenerator

// explaining what to keep in mind when making a move generator
// https://peterellisjones.com/posts/generating-legal-chess-moves-efficiently/
