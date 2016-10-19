// This file is part of the lwchess library.
// Copyright (C) 2016 Lakin Wecker <lakin@wecker.ca>
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate phf_codegen;

// I can't wait until we have build plugins, so we can do this at
// compile time without a nightly and without codegen. Ugh.

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, r#"enum Color {{ White, Black }}"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"const COLOR_NAMES: &'static [&'static str] = &["black", "white"];"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"enum PieceTypes {{ Pawn = 1, Knight = 2, Bishop = 3, Rook = 4, Queen = 5, King = 6 }}"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"const PIECE_SYMBOLS: &'static [&'static str] = &["", "p", "n", "b", "r", "q", "k"];"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"const PIECE_NAMES: &'static [&'static str] = &["", "pawn", "knight", "bishop", "rook", "queen", "king"];"#).unwrap();
    write!(&mut file, "\n");

    write!(&mut file, "static UNICODE_PIECE_SYMBOLS: phf::Map<&'static str, &'static str> = ").unwrap();
    phf_codegen::Map::new()
    	.entry("R", "\"♖\"")
		.entry("r", "\"♜\"")
        .entry("N", "\"♘\"")
        .entry("n", "\"♞\"")
        .entry("B", "\"♗\"")
        .entry("b", "\"♝\"")
        .entry("Q", "\"♕\"")
        .entry("q", "\"♛\"")
        .entry("K", "\"♔\"")
        .entry("k", "\"♚\"")
        .entry("P", "\"♙\"")
        .entry("p", "\"♟\"")
        .build(&mut file)
        .unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(&mut file, r#"const FILE_NAMES: &'static [&'static str] = &["a", "b", "c", "d", "e", "f", "g", "h"];"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"const RANK_NAME: &'static [&'static str] = &["1", "2", "3", "4", "5", "6", "7", "8"];"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"const STARTING_BOARD_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";"#).unwrap();
    write!(&mut file, "\n");

    write!(&mut file, r#"enum STATUS {{ StatusValid = 0, StatusNoWhiteKing = 1, StatusNoBlackKing = 2, StatusTooManyKings = 4, StatusTooManyWhitePawns = 8, StatusToomanyBlackPawns = 16, StatusPawnsOnBackRank = 32, StatusTooManyWhitePieces = 64, StatusTooManyBlackPieces = 128, StatusBadCastlingRights = 256, StatusInvalidEpSquare = 512, StatusOppositeCheck = 1024 }}"#).unwrap();
    write!(&mut file, "\n");

    let files = ["A", "B", "C", "D", "E", "F", "G", "H"];
    let ranks = ["1", "2", "3", "4", "5", "6", "7", "8"];
    for i in 0..64 {
        write!(&mut file, "const {}{}: u8 = {};\n", files[i%8], ranks[i/8], i).unwrap();
    }
    write!(&mut file, r#"const SQUARES: &'static [u8] = &[ A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3, H3, A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5, G5, H5, A6, B6, C6, D6, E6, F6, G6, H6, A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8 ];"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"const SQUARES_180: &'static [u8] = &[ A8, B8, C8, D8, E8, F8, G8, H8, A7, B7, C7, D7, E7, F7, G7, H7, A6, B6, C6, D6, E6, F6, G6, H6, A5, B5, C5, D5, E5, F5, G5, H5, A4, B4, C4, D4, E4, F4, G4, H4, A3, B3, C3, D3, E3, F3, G3, H3, A2, B2, C2, D2, E2, F2, G2, H2, A1, B1, C1, D1, E1, F1, G1, H1 ];"#).unwrap();
    write!(&mut file, "\n");


    fn file_index(square: u8) -> u8 { square & 7u8 }
    fn rank_index(square: u8) -> u8 { square >> 3u8 }
    fn square(file_index: u8, rank_index: u8) -> u8 { rank_index * 8u8 + file_index }
    write!(&mut file, r#"fn file_index(square: u8) -> u8 {{ square & 7u8 }}"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"fn rank_index(square: u8) -> u8 {{ square >> 3u8 }}"#).unwrap();
    write!(&mut file, "\n");
    write!(&mut file, r#"fn square(file_index: u8, rank_index: u8) -> u8 {{ rank_index * 8u8 + file_index }}"#).unwrap();
    write!(&mut file, "\n");

    const BB_VOID: u64 = 0b0000000000000000000000000000000000000000000000000000000000000000;
    const BB_ALL: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;
    let mut bb_sq = HashMap::new();
    write!(&mut file, "const BB_VOID: u64 = 0b{:064b};\n", BB_VOID).unwrap();
    write!(&mut file, "const BB_ALL: u64 = 0b{:064b};\n", BB_ALL).unwrap();
    for i in 0..64 {
        let file_name = files[i%8];
        let rank_name = ranks[i/8];
        let value: u64 = 1 << i;
        bb_sq.insert(format!("{}{}", file_name, rank_name), value);
        write!(&mut file, "const BB_{}{}: u64 = 0b{:064b};\n", file_name, rank_name, value).unwrap();
    }
    write!(&mut file, r#"const BB_SQUARES: &'static [u64] = &[ BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1, BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2, BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3, BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4, BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5, BB_A6, BB_B6, BB_C6, BB_D6, BB_E6, BB_F6, BB_G6, BB_H6, BB_A7, BB_B7, BB_C7, BB_D7, BB_E7, BB_F7, BB_G7, BB_H7, BB_A8, BB_B8, BB_C8, BB_D8, BB_E8, BB_F8, BB_G8, BB_H8 ];"#).unwrap();
    write!(&mut file, "\n");

    let mut bb_light_squares = BB_VOID;
    let mut bb_dark_squares = BB_VOID;
    for i in 0..64 {
        let mask: u64 = 1 << i;
        if (file_index(i) + rank_index(i)) % 2u8 != 0 {
            bb_light_squares |= mask;
        } else {
            bb_dark_squares |= mask;
        }
    }

    write!(&mut file, "const BB_LIGHT_SQUARES: u64 = 0b{:064b};\n", bb_light_squares).unwrap();
    write!(&mut file, "const BB_DARK_SQUARES: u64 = 0b{:064b};\n", bb_dark_squares).unwrap();
    let bb_file_a = bb_sq["A1"] | bb_sq["A2"] | bb_sq["A3"] | bb_sq["A4"] | bb_sq["A5"] | bb_sq["A6"] | bb_sq["A7"] | bb_sq["A8"];
    let bb_file_b = bb_sq["B1"] | bb_sq["B2"] | bb_sq["B3"] | bb_sq["B4"] | bb_sq["B5"] | bb_sq["B6"] | bb_sq["B7"] | bb_sq["B8"];
    let bb_file_c = bb_sq["C1"] | bb_sq["C2"] | bb_sq["C3"] | bb_sq["C4"] | bb_sq["C5"] | bb_sq["C6"] | bb_sq["C7"] | bb_sq["C8"];
    let bb_file_d = bb_sq["D1"] | bb_sq["D2"] | bb_sq["D3"] | bb_sq["D4"] | bb_sq["D5"] | bb_sq["D6"] | bb_sq["D7"] | bb_sq["D8"];
    let bb_file_e = bb_sq["E1"] | bb_sq["E2"] | bb_sq["E3"] | bb_sq["E4"] | bb_sq["E5"] | bb_sq["E6"] | bb_sq["E7"] | bb_sq["E8"];
    let bb_file_f = bb_sq["F1"] | bb_sq["F2"] | bb_sq["F3"] | bb_sq["F4"] | bb_sq["F5"] | bb_sq["F6"] | bb_sq["F7"] | bb_sq["F8"];
    let bb_file_g = bb_sq["G1"] | bb_sq["G2"] | bb_sq["G3"] | bb_sq["G4"] | bb_sq["G5"] | bb_sq["G6"] | bb_sq["G7"] | bb_sq["G8"];
    let bb_file_h = bb_sq["H1"] | bb_sq["H2"] | bb_sq["H3"] | bb_sq["H4"] | bb_sq["H5"] | bb_sq["H6"] | bb_sq["H7"] | bb_sq["H8"];
    write!(&mut file, "const BB_FILE_A: u64 = 0b{:064b};\n", bb_file_a);
    write!(&mut file, "const BB_FILE_B: u64 = 0b{:064b};\n", bb_file_b);
    write!(&mut file, "const BB_FILE_C: u64 = 0b{:064b};\n", bb_file_c);
    write!(&mut file, "const BB_FILE_D: u64 = 0b{:064b};\n", bb_file_d);
    write!(&mut file, "const BB_FILE_E: u64 = 0b{:064b};\n", bb_file_e);
    write!(&mut file, "const BB_FILE_F: u64 = 0b{:064b};\n", bb_file_f);
    write!(&mut file, "const BB_FILE_G: u64 = 0b{:064b};\n", bb_file_g);
    write!(&mut file, "const BB_FILE_H: u64 = 0b{:064b};\n", bb_file_h);
    write!(
        &mut file,
       "const BB_FILES: [u64, 8] =  [0b{:064b}, 0b{:064b}, 0b{:064b}, 0b{:064b}, 0b{:064b}, 0b{:064b}, 0b{:064b}, 0b{:064b}];\n",
       bb_file_a, bb_file_b, bb_file_c, bb_file_d, bb_file_e, bb_file_f, bb_file_g, bb_file_h
    );
    write!(&mut file, "static FILE_MASK: phf::Map<&'static u64, &'static u8> = ").unwrap();
    let file_mask = phf_codegen::Map::<u64>::new();
    file_mask.entry(0, "0");
    for (i, (square_name, mask)) in bb_sq.iter().enumerate() {
        file_mask.entry(
            *mask, //file_index(i as u8)
            //&format!("{}", mask),
            format!("{}", file_index(i as u8))
        );
    }
    file_mask.build(&mut file).unwrap();

}
