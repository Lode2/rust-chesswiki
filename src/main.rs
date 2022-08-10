mod chess_pos;

fn main() {
    let starting_pos_fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let mut starting_pos: chess_pos::Position = chess_pos::Position {
        bb_sides: [chess_pos::BitBoard(0); 2],
        bb_pieces: [[chess_pos::BitBoard(0); 6]; 2],
    };
    starting_pos.load(starting_pos_fen);

    println!("Pretty Starting Position:{}", starting_pos.pretty());

    // get the bitboard with all the white queens
    let white_queens: chess_pos::BitBoard =
        starting_pos.bb_pieces[chess_pos::Sides::WHITE][chess_pos::Pieces::QUEEN];

    println!("All the white queens:{}", white_queens.pretty());

    let example_game_fen: String =
        String::from("r4rk1/pbqn1pp1/1pn1p2p/2ppP2Q/3P4/2PBP1B1/PP1N2PP/R4RK1 w - - 0 14");

    starting_pos.load(&example_game_fen);

    println!("Loaded example position:{}", starting_pos.pretty())
}
// https://www.codeproject.com/Articles/5313417/Worlds-Fastest-Bitboard-Chess-Movegenerator
