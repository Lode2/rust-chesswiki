// file containing bitboard and position struct
use regex::Regex;

// define objects and apply methods to those objects
// #[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
#[derive(Debug, Clone, Copy)]
pub struct BitBoard(pub u64);

// add methods to the BitBoard struct
impl BitBoard {
    // creates a pretty string of the bitboard (for debugging purposes)
    pub fn pretty(&self) -> String {
        let my_int = self.u64();
        let formatted_bitboard = format!("{my_int:064b}");
        let pretty_array: [char; 64] = rearrange_array(formatted_bitboard.chars());
        let mut pretty_string: String = String::new();

        for (idx, x) in pretty_array.into_iter().enumerate() {
            if idx % 8 == 0 {
                pretty_string += "\n";
                pretty_string += &(x as char).to_string();
                pretty_string += " ";
            } else {
                pretty_string += &(x as char).to_string();
                pretty_string += " ";
            }
        }
        return pretty_string;
    }
    // given an initial bitboard u64, flip the bit at position square_pos
    pub fn bit_flip(&self, square_idx: usize) -> BitBoard {
        let BitBoard(old_u64) = self;
        return BitBoard(old_u64 ^ (1 << (square_idx)));
    }
    // returns a vector with the position of all 1-bits
    pub fn find_set_bits(&self) -> Vec<usize> {
        let mut set_bits: Vec<usize> = vec![];
        let &BitBoard(mut my_int) = self;
        let mut index: usize = 0;
        while my_int != 0 {
            if (my_int & 1) == 1 {
                set_bits.push(index);
            }
            my_int = my_int >> 1;
            index += 1;
        }
        return set_bits;
    }
    // extract the u64 inside the BitBoard struct
    pub fn u64(&self) -> &u64 {
        let BitBoard(my_integer) = self;
        return my_integer;
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
    // execute this function to get the chess starting position
    pub fn starting_pos() -> Position {
        return Position {
            bb_sides: [
                BitBoard(0b0000000000000000000000000000000000000000000000001111111111111111),
                BitBoard(0b1111111111111111000000000000000000000000000000000000000000000000),
            ],
            bb_pieces: [
                [
                    BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000000100100),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000001000010),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000010000001),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000000001000),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000000010000),
                ],
                [
                    BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000),
                    BitBoard(0b0010010000000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b0100001000000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b1000000100000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b0000100000000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b0001000000000000000000000000000000000000000000000000000000000000),
                ],
            ],
        };
    }
    // creates a pretty string of the position (for debugging purposes)
    pub fn pretty(&self) -> String {
        let mut pretty_array: [&str; 64] = ["x "; 64];

        // 1. find all occupied squares for both colors -> so 2 variables
        let white_occupies = self.bb_sides[0].find_set_bits();
        let black_occupies = self.bb_sides[1].find_set_bits();

        // 2. loop over these found indices for both colors
        for w_idx in white_occupies {
            // 3. find what the piece is that occupies a found index
            let found_piece = self.piece_id_finder(0, w_idx);
            let pretty_piece = match found_piece {
                (0, 0, _) => "P ",
                (0, 1, _) => "B ",
                (0, 2, _) => "N ",
                (0, 3, _) => "R ",
                (0, 4, _) => "Q ",
                (0, 5, _) => "K ",
                _ => "Err",
            };
            // 4. append this piece to the pretty_array
            pretty_array[to_pretty_index(63 - w_idx)] = pretty_piece;
        }
        for b_idx in black_occupies {
            let found_piece = self.piece_id_finder(1, b_idx);
            let pretty_piece = match found_piece {
                (1, 0, _) => "p ",
                (1, 1, _) => "b ",
                (1, 2, _) => "n ",
                (1, 3, _) => "r ",
                (1, 4, _) => "q ",
                (1, 5, _) => "k ",
                _ => "Err",
            };
            pretty_array[to_pretty_index(63 - b_idx)] = pretty_piece;
        }

        // 5. transform pretty_array into a string
        let mut pretty_pos: String = String::new();
        for i in 0..64 {
            if i % 8 == 0 {
                pretty_pos += "\n"
            }
            pretty_pos += pretty_array[i];
        }

        return pretty_pos;
    }
    // load a fen position
    pub fn load(&mut self, fen: &str) {
        // start with empty position
        self.bb_sides = [BitBoard(0); 2];
        self.bb_pieces = [[BitBoard(0); 6]; 2];

        let fen_split_regex: Regex = Regex::new(" ").unwrap();
        let slash_regex = Regex::new("/").unwrap();

        // remove the slashes from the piece data of the fen
        let slashless = slash_regex.replace_all(fen, "");
        // split the piece data and meta data of the fen
        let fen_data_split: Vec<&str> = fen_split_regex.splitn(&slashless, 2).collect();

        let mut square_count: usize = 0;
        for fen_character in fen_data_split[0].bytes() {
            // find the square index of the analyzed character in the fen
            let div_by_8: f32 = square_count as f32 / 8.;
            let floor: usize = (div_by_8).floor() as usize;
            let square_idx: usize = (7 - floor) * 8 + ((div_by_8 % 1.) * 8.) as usize;

            // catch which piece bitboard needs to be updated
            match fen_character {
                0b110001 => (),                // 1, 1 is already added at each iteration
                0b110010 => square_count += 1, // 2
                0b110011 => square_count += 2, // 3
                0b110100 => square_count += 3, // 4
                0b110101 => square_count += 4, // 5
                0b110110 => square_count += 5, // 6
                0b110111 => square_count += 6, // 7
                0b111000 => square_count += 7, // 8
                0b1010000 => {
                    self.bb_pieces[0][0] = self.bb_pieces[0][0].bit_flip(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_flip(square_idx);
                } // P
                0b1010010 => {
                    self.bb_pieces[0][3] = self.bb_pieces[0][3].bit_flip(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_flip(square_idx);
                } // R
                0b1001110 => {
                    self.bb_pieces[0][2] = self.bb_pieces[0][2].bit_flip(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_flip(square_idx);
                } // N
                0b1000010 => {
                    self.bb_pieces[0][1] = self.bb_pieces[0][1].bit_flip(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_flip(square_idx);
                } // B
                0b1001011 => {
                    self.bb_pieces[0][5] = self.bb_pieces[0][5].bit_flip(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_flip(square_idx);
                } // K
                0b1010001 => {
                    self.bb_pieces[0][4] = self.bb_pieces[0][4].bit_flip(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_flip(square_idx);
                } // Q
                0b1110000 => {
                    self.bb_pieces[1][0] = self.bb_pieces[1][0].bit_flip(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_flip(square_idx);
                } // p
                0b1110010 => {
                    self.bb_pieces[1][3] = self.bb_pieces[1][3].bit_flip(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_flip(square_idx);
                } // r
                0b1101110 => {
                    self.bb_pieces[1][2] = self.bb_pieces[1][2].bit_flip(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_flip(square_idx);
                } // n
                0b1100010 => {
                    self.bb_pieces[1][1] = self.bb_pieces[1][1].bit_flip(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_flip(square_idx);
                } // b
                0b1101011 => {
                    self.bb_pieces[1][5] = self.bb_pieces[1][5].bit_flip(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_flip(square_idx);
                } // k
                0b1110001 => {
                    self.bb_pieces[1][4] = self.bb_pieces[1][4].bit_flip(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_flip(square_idx);
                } // q
                _ => println!("No match found in the fen data!"),
            };
            square_count += 1;
        }
    }
    // return all the legal moves of the position
    pub fn moves(&self) -> Vec<&str> {
        // pawns
        // let Position {
        //     bb_sides: [..],
        //     bb_pieces: [[wpawn, ..], [..]],
        // } = self;

        let moves: Vec<&str> = vec!["e4", "e5"];
        return moves;
    }
    // output -> vector of tuples: (piece color (0=white), piece id (0=pawn), piece position (a1=0))
    pub fn get_pieces(&self, piece_color: usize) -> Vec<(usize, usize, usize)> {
        let occupied_idx = self.bb_sides[piece_color].find_set_bits();

        let mut pieces: Vec<(usize, usize, usize)> = vec![];

        // find which piece occupies the found square index
        for idx in occupied_idx {
            pieces.push(self.piece_id_finder(piece_color, idx));
        }
        return pieces;
    }
    // finds what piece occupies a provided square index
    pub fn piece_id_finder(&self, color: usize, idx: usize) -> (usize, usize, usize) {
        #[allow(unused_assignments)]
        let mut piece_id: usize = 5; // standard
        if (self.bb_pieces[color][0].u64() >> idx) & 1 == 1 {
            piece_id = 0;
        } else if (self.bb_pieces[color][1].u64() >> idx) & 1 == 1 {
            piece_id = 1;
        } else if (self.bb_pieces[color][2].u64() >> idx) & 1 == 1 {
            piece_id = 2;
        } else if (self.bb_pieces[color][3].u64() >> idx) & 1 == 1 {
            piece_id = 3;
        } else if (self.bb_pieces[color][4].u64() >> idx) & 1 == 1 {
            piece_id = 4;
        } // 5 check not needed, default
        return (color, piece_id, idx);
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

// generally used functions for this file
// transform the an index of a square in a bitboard to an index for display
fn to_pretty_index(ugly_index: usize) -> usize {
    let div_by_8: f32 = ugly_index as f32 / 8.;
    let floor: usize = (div_by_8).floor() as usize;
    let pretty_index: usize = (floor) * 8 + 7 - ((div_by_8 % 1.) * 8.) as usize;
    // println!("ugly_index:{}, pretty_index:{}", ugly_index, pretty_index);
    return pretty_index;
}

// transform a bitboard-ordered-array to an array that can be used for display
fn rearrange_array(old_array: std::str::Chars) -> [char; 64] {
    let mut new_array: [char; 64] = ['0'; 64];
    for (old_idx, itm) in old_array.into_iter().enumerate() {
        if itm == '1' {
            let new_idx: usize = to_pretty_index(old_idx);
            new_array[new_idx] = '1';
        }
    }
    return new_array;
}
