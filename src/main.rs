mod chess_pos;

fn main() {
    // pieces on the entire a-file
    // let empty_val = 0b0000000000000000000000000000000000000000000000000000000000000000;
    let val = 0b1000000010000000100000001000000010000000100000001000000010000000;

    // define a position
    let new_pos = chess_pos::Position {
        bb_sides: [chess_pos::BitBoard(val), chess_pos::BitBoard(val)],
        bb_pieces: [[chess_pos::BitBoard(val); 6]; 2],
    };

    // get the bitboard with all the white queens
    let _white_queens: chess_pos::BitBoard =
        new_pos.bb_pieces[chess_pos::Sides::WHITE][chess_pos::Pieces::QUEEN];

    // struct destructuring:
    // this lets white_bitboard equal the white bitboard
    let chess_pos::Position {
        bb_sides: [white_bitboard, ..],
        ..
    } = new_pos;

    let chess_pos::BitBoard(_bitval) = white_bitboard;

    // println!("Bit value of BitBoard: \n{:b}", bitval as u64);

    // println!("Pretty BitBoard: {}", white_bitboard.pretty());

    println!("Pretty Position: \n{}", new_pos.pretty());
}
// https://www.codeproject.com/Articles/5313417/Worlds-Fastest-Bitboard-Chess-Movegenerator
