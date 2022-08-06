mod chess_pos;

fn main() {
    // bitboards for the starting position
    struct StartingPos;
    impl StartingPos {
        const WHITE: chess_pos::BitBoard =
            chess_pos::BitBoard(0b1111111111111111000000000000000000000000000000000000000000000000);
        const BLACK: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000001111111111111111);
        const WPAWN: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000);
        const BPAWN: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000);
        const WROOK: chess_pos::BitBoard =
            chess_pos::BitBoard(0b1000000100000000000000000000000000000000000000000000000000000000);
        const BROOK: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000010000001);
        const WBISHOP: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0010010000000000000000000000000000000000000000000000000000000000);
        const BBISHOP: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000000100100);
        const WKNIGHT: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0100001000000000000000000000000000000000000000000000000000000000);
        const BKNIGHT: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000001000010);
        const WKING: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000100000000000000000000000000000000000000000000000000000000000);
        const BKING: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000000001000);
        const WQUEEN: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0001000000000000000000000000000000000000000000000000000000000000);
        const BQUEEN: chess_pos::BitBoard =
            chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000000010000);
    }
    // let empty_val = 0b0000000000000000000000000000000000000000000000000000000000000000;
    // pieces on the entire a-file
    let val: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;

    // define a new position
    let new_pos: chess_pos::Position = chess_pos::Position {
        bb_sides: [chess_pos::BitBoard(val), chess_pos::BitBoard(val)],
        bb_pieces: [[chess_pos::BitBoard(val); 6]; 2],
    };

    let mut starting_pos: chess_pos::Position = chess_pos::Position {
        bb_sides: ([StartingPos::WHITE, StartingPos::BLACK]),
        bb_pieces: ([
            [
                StartingPos::WPAWN,
                StartingPos::WBISHOP,
                StartingPos::WKNIGHT,
                StartingPos::WROOK,
                StartingPos::WQUEEN,
                StartingPos::WKING,
            ],
            [
                StartingPos::BPAWN,
                StartingPos::BBISHOP,
                StartingPos::BKNIGHT,
                StartingPos::BROOK,
                StartingPos::BQUEEN,
                StartingPos::BKING,
            ],
        ]),
    };

    // get the bitboard with all the white queens
    let _white_queens: chess_pos::BitBoard =
        new_pos.bb_pieces[chess_pos::Sides::WHITE][chess_pos::Pieces::QUEEN];

    // struct destructuring:
    // this lets white_bitboard equal the bitboard containing the white pieces
    let chess_pos::Position {
        bb_sides: [white_bitboard, ..],
        ..
    } = new_pos;

    let chess_pos::BitBoard(bitval) = white_bitboard;

    println!("Bit value of white pieces BitBoard: \n{:b}", bitval as u64);

    println!("Pretty BitBoard:{}", white_bitboard.pretty());

    println!("Pretty Position:{}", new_pos.pretty());

    println!("Pretty Starting Position:{}", starting_pos.pretty());

    // let my_game: String =
    //     String::from("r4rk1/pQ1bbppp/2p1nq1n/3pp3/2BPP3/P1N1BN2/1P3PPP/R3K2R w KQ - 0 13");
    let example_game: String =
        String::from("r4rk1/pbqn1pp1/1pn1p2p/2ppP2Q/3P4/2PBP1B1/PP1N2PP/R4RK1 w - - 0 14");

    // let program_test = String::from("12345678prnbkqPRNBKQ");

    starting_pos.load(&example_game);

    // println!("Loaded FEN:{}", starting_pos.pretty())
}
// https://www.codeproject.com/Articles/5313417/Worlds-Fastest-Bitboard-Chess-Movegenerator
