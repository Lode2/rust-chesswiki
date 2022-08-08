// file containing bitboard and position struct
use regex::Regex;

// generally used functions
fn to_pretty_index(ugly_index: usize) -> usize {
    let div_by_8: f32 = ugly_index as f32 / 8.;
    let floor: usize = (div_by_8).floor() as usize;
    let pretty_index: usize = (7 - floor) * 8 + ((div_by_8 % 1.) * 8.) as usize;
    return pretty_index;
}

fn rearrange_array(old_array: std::str::Chars) -> [char; 64] {
    let mut new_array: [char; 64] = [' '; 64];
    for (old_idx, itm) in old_array.into_iter().enumerate() {
        let new_idx: usize = to_pretty_index(old_idx);
        new_array[new_idx] = itm;
    }
    return new_array;
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

// add methods to the BitBoard struct
impl BitBoard {
    pub fn pretty(&self) -> String {
        let BitBoard(pretty) = self;
        let formatted_bitboard = format!("{pretty:064b}");
        let pretty_array: [char; 64] = rearrange_array(formatted_bitboard.chars());
        let mut pretty_board: String = String::new();

        for (i, x) in pretty_array.into_iter().enumerate() {
            if i % 8 == 0 {
                pretty_board += "\n";
                pretty_board += &(x as char).to_string();
                pretty_board += " ";
            } else {
                pretty_board += &(x as char).to_string();
                pretty_board += " ";
            }
        }
        return pretty_board;
    }
}

#[derive(Debug)]
pub struct Position {
    /// Board for each side
    pub bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    pub bb_pieces: [[BitBoard; 6]; 2],
}

impl Position {
    pub fn pretty(&self) -> String {
        let mut pretty_array: [&str; 64] = ["x "; 64];

        // destructuring the position, white is the bitboard for white, wpawn the bitboard for white pawns etc.
        let Position {
            bb_sides: [_white, _black],
            bb_pieces:
                [[wpawn, wbishop, wknight, wrook, wqueen, wking], [bpawn, bbishop, bknight, brook, bqueen, bking]],
        } = self;

        // array of all pieces bitboards
        let all_pieces = [
            wpawn, wbishop, wknight, wrook, wqueen, wking, bpawn, bbishop, bknight, brook, bqueen,
            bking,
        ];

        let mut j: u8 = 0;

        // loop over all the pieces bitboards
        for i in all_pieces.iter() {
            // destructure bitboard to get the unsigned 64-bit integer
            let BitBoard(val) = i;

            // loop over a stringified u64, get bit position and bit value
            for k in format!("{val:064b}").to_string().char_indices() {
                if k.1 == '1' {
                    let string_representation: &str = match j {
                        0 => "P ",   // wpawn
                        6 => "p ",   // bpawn
                        3 => "R ",   // wrook
                        9 => "r ",   // brook
                        1 => "B ",   // wbishop
                        7 => "b ",   // bbishop
                        2 => "N ",   // wknight
                        8 => "n ",   // bknight
                        5 => "K ",   // wking
                        11 => "k ",  // bking
                        4 => "Q ",   // wqueen
                        10 => "q ",  // bqueen
                        _ => "Err ", // in case of no match
                    };
                    // adjusted k.0 index to inverse board
                    pretty_array[to_pretty_index(k.0)] = string_representation;
                    // pretty_array[k.0] = string_representation;
                }
            }
            j += 1;
        }

        // println!("{:#?}", pretty_array);

        // transforming the pretty array into a pretty string
        let mut pretty_pos: String = String::new();
        for i in 0..64 {
            if i % 8 == 0 {
                pretty_pos += "\n"
            }
            pretty_pos += pretty_array[i];
        }

        return pretty_pos;
    }

    pub fn load(&mut self, fen: &str) {
        // split the FEN using regex

        let fen_split_regex: Regex = Regex::new(" ").unwrap();
        let slash_regex = Regex::new("/").unwrap();

        // remove the slashes from the piece data of the fen
        let slashless = slash_regex.replace_all(fen, "");
        // split the piece data and meta data of the fen
        let fen_data_split: Vec<&str> = fen_split_regex.splitn(&slashless, 2).collect();

        fn myfunc(square_pos: u64, color: u8, piece: u8) {
            // format square_pos to binary number
            println!("Place a {piece} of color {color} on square {square_pos}");
            // self.bb_pieces[0][1] =
            //     BitBoard(0b0000000000000000000000000001000000000000000000000000000000000000);
            // self.bb_sides[0] =
            //     BitBoard(0b0000000000000000000000000001000000000000000000000000000000000000);
        }

        let mut square_num: u64 = 0;
        for i in fen_data_split[0].bytes() {
            square_num += 1;

            match i {
                0b110001 => (),                        // 1, 1 is already added at each iteration
                0b110010 => square_num += 1,           // 2
                0b110011 => square_num += 2,           // 3
                0b110100 => square_num += 3,           // 4
                0b110101 => square_num += 4,           // 5
                0b110110 => square_num += 5,           // 6
                0b110111 => square_num += 6,           // 7
                0b111000 => square_num += 7,           // 8
                0b1010000 => myfunc(square_num, 0, 0), // P
                0b1010010 => myfunc(square_num, 0, 3), // R
                0b1001110 => myfunc(square_num, 0, 2), // N
                0b1000010 => myfunc(square_num, 0, 1), // B
                0b1001011 => myfunc(square_num, 0, 5), // K
                0b1010001 => myfunc(square_num, 0, 4), // Q
                0b1110000 => myfunc(square_num, 1, 0), // p
                0b1110010 => myfunc(square_num, 1, 3), // r
                0b1101110 => myfunc(square_num, 1, 2), // n
                0b1100010 => myfunc(square_num, 1, 1), // b
                0b1101011 => myfunc(square_num, 1, 5), // k
                0b1110001 => myfunc(square_num, 1, 4), // q
                0b1011010 => {
                    self.bb_pieces[0][1] = BitBoard(
                        0b0000000000000000000000000001000000000000000000000000000000000000,
                    );
                    self.bb_sides[0] = BitBoard(
                        0b0000000000000000000000000001000000000000000000000000000000000000,
                    );
                }
                _ => println!("No match found in the fen data!"),
            };
            // println!("{} has the byte value {:b}", i.escape_ascii(), i);
        }

        // let piece_regex: Regex = Regex::new("[1-8]").unwrap();

        // for i in 0..1 {
        //     let pieces: Vec<_> = board_rank[i].chars().collect();
        //     println!("{:?}", pieces)
        // }

        // append bits to respective bitboard for the occupied squares

        println!(
            "{}",
            BitBoard(0b0000000000000000000000000001000000000000000000000000000000000000).pretty()
        );

        // replace the old bitboards
        let previous_val = 0b0000000000000000000000000001000000000000000000000000000000000000;
        let square_pos = 32;
        self.bb_pieces[0][1] = BitBoard(previous_val ^ (1 << (square_pos - 1))); // WORKS!!

        println!("{}", self.bb_pieces[0][1].pretty());

        self.bb_pieces[0][1] =
            BitBoard(0b0000000000000000000000000001000000000000000000000000000000000000);
        self.bb_sides[0] =
            BitBoard(0b0000000000000000000000000001000000000000000000000000000000000000);
    }
}

// intuitive pointers to the position's sides bitboard
pub struct Sides;
#[allow(dead_code)]
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

// intuitive pointers to the position's pieces bitboard
pub struct Pieces;
#[allow(dead_code)]
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}
