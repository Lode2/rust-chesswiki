/*
!!!!! IMPORTANT !!!!!
THESE BITBOARDS DO NOT DISPLAY THE CORRECT STARTING POSITION
!!!!! IMPORTANT !!!!!
*/

// bitboards for the starting position
// struct StartingPos;
// impl StartingPos {
//     const WHITE: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000001111111111111111);
//     const BLACK: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b1111111111111111000000000000000000000000000000000000000000000000);
//     const WPAWN: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000);
//     const BPAWN: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000);
//     const WROOK: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000010000001);
//     const BROOK: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b1000000100000000000000000000000000000000000000000000000000000000);
//     const WBISHOP: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000000100100);
//     const BBISHOP: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0010010000000000000000000000000000000000000000000000000000000000);
//     const WKNIGHT: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000001000010);
//     const BKNIGHT: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0100001000000000000000000000000000000000000000000000000000000000);
//     const WKING: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000000010000);
//     const BKING: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000100000000000000000000000000000000000000000000000000000000000);
//     const WQUEEN: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0000000000000000000000000000000000000000000000000000000000001000);
//     const BQUEEN: chess_pos::BitBoard =
//         chess_pos::BitBoard(0b0001000000000000000000000000000000000000000000000000000000000000);
// }

// let mut starting_pos: chess_pos::Position = chess_pos::Position {
//     bb_sides: ([StartingPos::WHITE, StartingPos::BLACK]),
//     bb_pieces: ([
//         [
//             StartingPos::WPAWN,
//             StartingPos::WBISHOP,
//             StartingPos::WKNIGHT,
//             StartingPos::WROOK,
//             StartingPos::WQUEEN,
//             StartingPos::WKING,
//         ],
//         [
//             StartingPos::BPAWN,
//             StartingPos::BBISHOP,
//             StartingPos::BKNIGHT,
//             StartingPos::BROOK,
//             StartingPos::BQUEEN,
//             StartingPos::BKING,
//         ],
//     ]),
// };

/*
FEN'S FOR TESTING PURPOSES
*/

// initiate empty position
// let new_pos: chess_pos::Position = chess_pos::Position {
//     bb_sides: [chess_pos::BitBoard(0); 2],
//     bb_pieces: [[chess_pos::BitBoard(0); 6]; 2],
// };

// let empty_fen: String = String::from("8/8/8/8/8/8/8/8 w - - 0 14");

// let program_test_fen = String::from("12345678prnbkqPRNBKQ");

// let my_game_fen: String = String::from("r4rk1/pQ1bbppp/2p1nq1n/3pp3/2BPP3/P1N1BN2/1P3PPP/R3K2R w KQ - 0 13");
