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

//------------------------------------------------------------------------------
// A pure rust chess library.
//
// Shamelessly patterned after the amazing python-chess library by Niklas Fiekas
//------------------------------------------------------------------------------

#![feature(plugin, const_fn)]
#![plugin(phf_macros)]

extern crate phf;
use std::ops::{BitAnd, Shr, Add, Mul};


enum Color {
    White,
    Black
}
const COLOR_NAMES: &'static [&'static str] = &["black", "white"];
enum PieceTypes {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6
}
const PIECE_SYMBOLS: &'static [&'static str] = &["", "p", "n", "b", "r", "q", "k"];
const PIECE_NAMES: &'static [&'static str] = &["", "pawn", "knight", "bishop", "rook", "queen", "king"];
static UNICODE_PIECE_SYMBOLS: phf::Map<&'static str, &'static str> = phf_map! {
    "R" => "♖", "r" => "♜",
    "N" => "♘", "n" => "♞",
    "B" => "♗", "b" => "♝",
    "Q" => "♕", "q" => "♛",
    "K" => "♔", "k" => "♚",
    "P" => "♙", "p" => "♟",
};

const FILE_NAMES: &'static [&'static str] = &["a", "b", "c", "d", "e", "f", "g", "h"];
const RANK_NAME: &'static [&'static str] = &["1", "2", "3", "4", "5", "6", "7", "8"];
const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const STARTING_BOARD_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

enum STATUS {
    StatusValid = 0,
    StatusNoWhiteKing = 1,
    StatusNoBlackKing = 2,
    StatusTooManyKings = 4,
    StatusTooManyWhitePawns = 8,
    StatusToomanyBlackPawns = 16,
    StatusPawnsOnBackRank = 32,
    StatusTooManyWhitePieces = 64,
    StatusTooManyBlackPieces = 128,
    StatusBadCastlingRights = 256,
    StatusInvalidEpSquare = 512,
    StatusOppositeCheck = 1024
}

const A1: u8 = 0;
const B1: u8 = 1;
const C1: u8 = 2;
const D1: u8 = 3;
const E1: u8 = 4;
const F1: u8 = 5;
const G1: u8 = 6;
const H1: u8 = 7;
const A2: u8 = 8;
const B2: u8 = 9;
const C2: u8 = 10;
const D2: u8 = 11;
const E2: u8 = 12;
const F2: u8 = 13;
const G2: u8 = 14;
const H2: u8 = 15;
const A3: u8 = 16;
const B3: u8 = 17;
const C3: u8 = 18;
const D3: u8 = 19;
const E3: u8 = 20;
const F3: u8 = 21;
const G3: u8 = 22;
const H3: u8 = 23;
const A4: u8 = 24;
const B4: u8 = 25;
const C4: u8 = 26;
const D4: u8 = 27;
const E4: u8 = 28;
const F4: u8 = 29;
const G4: u8 = 30;
const H4: u8 = 31;
const A5: u8 = 32;
const B5: u8 = 33;
const C5: u8 = 34;
const D5: u8 = 35;
const E5: u8 = 36;
const F5: u8 = 37;
const G5: u8 = 38;
const H5: u8 = 39;
const A6: u8 = 40;
const B6: u8 = 41;
const C6: u8 = 42;
const D6: u8 = 43;
const E6: u8 = 44;
const F6: u8 = 45;
const G6: u8 = 46;
const H6: u8 = 47;
const A7: u8 = 48;
const B7: u8 = 49;
const C7: u8 = 50;
const D7: u8 = 51;
const E7: u8 = 52;
const F7: u8 = 53;
const G7: u8 = 54;
const H7: u8 = 55;
const A8: u8 = 56;
const B8: u8 = 57;
const C8: u8 = 58;
const D8: u8 = 59;
const E8: u8 = 60;
const F8: u8 = 61;
const G8: u8 = 62;
const H8: u8 = 63;

const SQUARES: &'static [u8] = &[
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
];
const SQUARES_180: &'static [u8] = &[
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1
];

// TODO: figure out how to genericize these
const fn file_index(square: u8) -> u8 {
    square & 7u8 
}
const fn rank_index(square: u8) -> u8 {
    square >> 3u8 
}
const fn square(file_index: u8, rank_index: u8) -> u8 {
    rank_index * 8u8 + file_index 
}

const BB_VOID: u64 = 0b0000000000000000000000000000000000000000000000000000000000000000;
const BB_ALL: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

const BB_A1: u64 = 1 << 0;
const BB_B1: u64 = 1 << 1;
const BB_C1: u64 = 1 << 2;
const BB_D1: u64 = 1 << 3;
const BB_E1: u64 = 1 << 4;
const BB_F1: u64 = 1 << 5;
const BB_G1: u64 = 1 << 6;
const BB_H1: u64 = 1 << 7;
const BB_A2: u64 = 1 << 8;
const BB_B2: u64 = 1 << 9;
const BB_C2: u64 = 1 << 10;
const BB_D2: u64 = 1 << 11;
const BB_E2: u64 = 1 << 12;
const BB_F2: u64 = 1 << 13;
const BB_G2: u64 = 1 << 14;
const BB_H2: u64 = 1 << 15;
const BB_A3: u64 = 1 << 16;
const BB_B3: u64 = 1 << 17;
const BB_C3: u64 = 1 << 18;
const BB_D3: u64 = 1 << 19;
const BB_E3: u64 = 1 << 20;
const BB_F3: u64 = 1 << 21;
const BB_G3: u64 = 1 << 22;
const BB_H3: u64 = 1 << 23;
const BB_A4: u64 = 1 << 24;
const BB_B4: u64 = 1 << 25;
const BB_C4: u64 = 1 << 26;
const BB_D4: u64 = 1 << 27;
const BB_E4: u64 = 1 << 28;
const BB_F4: u64 = 1 << 29;
const BB_G4: u64 = 1 << 30;
const BB_H4: u64 = 1 << 31;
const BB_A5: u64 = 1 << 32;
const BB_B5: u64 = 1 << 33;
const BB_C5: u64 = 1 << 34;
const BB_D5: u64 = 1 << 35;
const BB_E5: u64 = 1 << 36;
const BB_F5: u64 = 1 << 37;
const BB_G5: u64 = 1 << 38;
const BB_H5: u64 = 1 << 39;
const BB_A6: u64 = 1 << 40;
const BB_B6: u64 = 1 << 41;
const BB_C6: u64 = 1 << 42;
const BB_D6: u64 = 1 << 43;
const BB_E6: u64 = 1 << 44;
const BB_F6: u64 = 1 << 45;
const BB_G6: u64 = 1 << 46;
const BB_H6: u64 = 1 << 47;
const BB_A7: u64 = 1 << 48;
const BB_B7: u64 = 1 << 49;
const BB_C7: u64 = 1 << 50;
const BB_D7: u64 = 1 << 51;
const BB_E7: u64 = 1 << 52;
const BB_F7: u64 = 1 << 53;
const BB_G7: u64 = 1 << 54;
const BB_H7: u64 = 1 << 55;
const BB_A8: u64 = 1 << 56;
const BB_B8: u64 = 1 << 57;
const BB_C8: u64 = 1 << 58;
const BB_D8: u64 = 1 << 59;
const BB_E8: u64 = 1 << 60;
const BB_F8: u64 = 1 << 61;
const BB_G8: u64 = 1 << 62;
const BB_H8: u64 = 1 << 63;

const BB_SQUARES: &'static [u64] = &[
    BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1,
    BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2,
    BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3,
    BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4,
    BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5,
    BB_A6, BB_B6, BB_C6, BB_D6, BB_E6, BB_F6, BB_G6, BB_H6,
    BB_A7, BB_B7, BB_C7, BB_D7, BB_E7, BB_F7, BB_G7, BB_H7,
    BB_A8, BB_B8, BB_C8, BB_D8, BB_E8, BB_F8, BB_G8, BB_H8
];

const fn calc_light_squares() -> u64 {
    BB_SQUARES.iter().enumerate().fold(BB_VOID, |acc, (i, mask)| match i%2 {
        0 => acc,
        _ => acc | mask,
    })
}
const BB_LIGHT_SQUARES: u64 = calc_light_squares();

const fn calc_dark_squares() -> u64 {
    let dark_squares = BB_VOID;
    for i in 0..64 {
        if (file_index(i) + rank_index(i)) % 2u8 == 0 {
            dark_squares |= BB_SQUARES[i as usize];
        }
    }
    dark_squares
}
const BB_DARK_SQUARES: u64 = calc_dark_squares();
const BB_FILE_A: u64 = BB_A1 | BB_A2 | BB_A3 | BB_A4 | BB_A5 | BB_A6 | BB_A7 | BB_A8;
const BB_FILE_B: u64 = BB_B1 | BB_B2 | BB_B3 | BB_B4 | BB_B5 | BB_B6 | BB_B7 | BB_B8;
const BB_FILE_C: u64 = BB_C1 | BB_C2 | BB_C3 | BB_C4 | BB_C5 | BB_C6 | BB_C7 | BB_C8;
const BB_FILE_D: u64 = BB_D1 | BB_D2 | BB_D3 | BB_D4 | BB_D5 | BB_D6 | BB_D7 | BB_D8;
const BB_FILE_E: u64 = BB_E1 | BB_E2 | BB_E3 | BB_E4 | BB_E5 | BB_E6 | BB_E7 | BB_E8;
const BB_FILE_F: u64 = BB_F1 | BB_F2 | BB_F3 | BB_F4 | BB_F5 | BB_F6 | BB_F7 | BB_F8;
const BB_FILE_G: u64 = BB_G1 | BB_G2 | BB_G3 | BB_G4 | BB_G5 | BB_G6 | BB_G7 | BB_G8;
const BB_FILE_H: u64 = BB_H1 | BB_H2 | BB_H3 | BB_H4 | BB_H5 | BB_H6 | BB_H7 | BB_H8;

const BB_FILES: &'static [u64] = &[
    BB_FILE_A,
    BB_FILE_B,
    BB_FILE_C,
    BB_FILE_D,
    BB_FILE_E,
    BB_FILE_F,
    BB_FILE_G,
    BB_FILE_H
];


#[cfg(test)]
mod tests {
    use SQUARES;
    #[test]
    fn squares_is_properly_initalized() {
        for i in 0..64 {
            assert!(SQUARES[i] == i as u8);
        }
    }
}
