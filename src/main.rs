mod new_file;

fn main() {
    // pieces on the entire a-file
    // let empty_val = 0b0000000000000000000000000000000000000000000000000000000000000000;
    let val = 0b1000000010000000100000001000000010000000100000001000000010000000;

    // define a position
    let new_pos = new_file::Position {
        bb_sides: [new_file::BitBoard(val), new_file::BitBoard(val)],
        bb_pieces: [[new_file::BitBoard(val); 6]; 2],
    };

    // get the bitboard with all the white queens
    let _white_queens: new_file::BitBoard =
        new_pos.bb_pieces[new_file::Sides::WHITE][new_file::Pieces::QUEEN];

    // struct destructuring:
    // this lets white_bitboard equal the white bitboard
    let new_file::Position {
        bb_sides: [white_bitboard, ..],
        ..
    } = new_pos;

    let new_file::BitBoard(bitval) = white_bitboard;

    println!("Bit value of BitBoard: \n{:b}", bitval as u64);

    println!("Pretty BitBoard: {}", white_bitboard.pretty());

    println!("Pretty Position: \n{}", new_pos.pretty());
}
// https://www.codeproject.com/Articles/5313417/Worlds-Fastest-Bitboard-Chess-Movegenerator
