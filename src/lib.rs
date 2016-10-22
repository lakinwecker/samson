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

#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum PieceType {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6
}

pub enum STATUS {
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

lazy_static! {
    pub static ref COLOR_NAMES: Vec<&'static str> = vec!["black", "white"];
    pub static ref PIECE_SYMBOLS_WHITE: Vec<char> = vec![' ', 'P', 'N', 'B', 'R', 'Q', 'K'];
    pub static ref PIECE_SYMBOLS_BLACK: Vec<char> = vec![' ', 'p', 'n', 'b', 'r', 'q', 'k'];
    pub static ref PIECE_NAMES: Vec<&'static str> = vec!["", "pawn", "knight", "bishop", "rook", "queen", "king"];
    pub static ref UNICODE_PIECE_SYMBOLS_BLACK: Vec<char> = vec![' ', '♖', '♘', '♗', '♕', '♔', '♙'];
    pub static ref UNICODE_PIECE_SYMBOLS_WHITE: Vec<char> = vec![' ', '♜', '♞', '♝', '♛', '♚', '♟'];
    pub static ref FILE_NAMES: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    pub static ref RANK_NAME: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8'];
    pub static ref STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub static ref STARTING_BOARD_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
}

pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;
pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;
pub const A3: u8 = 16;
pub const B3: u8 = 17;
pub const C3: u8 = 18;
pub const D3: u8 = 19;
pub const E3: u8 = 20;
pub const F3: u8 = 21;
pub const G3: u8 = 22;
pub const H3: u8 = 23;
pub const A4: u8 = 24;
pub const B4: u8 = 25;
pub const C4: u8 = 26;
pub const D4: u8 = 27;
pub const E4: u8 = 28;
pub const F4: u8 = 29;
pub const G4: u8 = 30;
pub const H4: u8 = 31;
pub const A5: u8 = 32;
pub const B5: u8 = 33;
pub const C5: u8 = 34;
pub const D5: u8 = 35;
pub const E5: u8 = 36;
pub const F5: u8 = 37;
pub const G5: u8 = 38;
pub const H5: u8 = 39;
pub const A6: u8 = 40;
pub const B6: u8 = 41;
pub const C6: u8 = 42;
pub const D6: u8 = 43;
pub const E6: u8 = 44;
pub const F6: u8 = 45;
pub const G6: u8 = 46;
pub const H6: u8 = 47;
pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;
pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;
pub const SQUARES: &'static [u8] = &[
	A1, B1, C1, D1, E1, F1, G1, H1,
	A2, B2, C2, D2, E2, F2, G2, H2,
	A3, B3, C3, D3, E3, F3, G3, H3,
	A4, B4, C4, D4, E4, F4, G4, H4,
	A5, B5, C5, D5, E5, F5, G5, H5,
	A6, B6, C6, D6, E6, F6, G6, H6,
	A7, B7, C7, D7, E7, F7, G7, H7,
	A8, B8, C8, D8, E8, F8, G8, H8
];
pub const SQUARES_180: &'static [u8] = &[
	A8, B8, C8, D8, E8, F8, G8, H8,
	A7, B7, C7, D7, E7, F7, G7, H7,
	A6, B6, C6, D6, E6, F6, G6, H6,
	A5, B5, C5, D5, E5, F5, G5, H5,
	A4, B4, C4, D4, E4, F4, G4, H4,
	A3, B3, C3, D3, E3, F3, G3, H3,
	A2, B2, C2, D2, E2, F2, G2, H2,
	A1, B1, C1, D1, E1, F1, G1, H1
];

pub const SQUARE_NAMES: &'static [&'static str] = &[
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];

// TODO: figure out how to genericize these
pub fn file_index(square: u8) -> u8 {
    square & 7u8 
}
pub fn rank_index(square: u8) -> u8 {
    square >> 3u8 
}
pub fn square(file_index: u8, rank_index: u8) -> u8 {
    rank_index * 8u8 + file_index 
}


pub const BB_VOID: u64 = 0b0000000000000000000000000000000000000000000000000000000000000000;
pub const BB_ALL: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

pub const BB_A1: u64 = 1 << 0;
pub const BB_B1: u64 = 1 << 1;
pub const BB_C1: u64 = 1 << 2;
pub const BB_D1: u64 = 1 << 3;
pub const BB_E1: u64 = 1 << 4;
pub const BB_F1: u64 = 1 << 5;
pub const BB_G1: u64 = 1 << 6;
pub const BB_H1: u64 = 1 << 7;
pub const BB_A2: u64 = 1 << 8;
pub const BB_B2: u64 = 1 << 9;
pub const BB_C2: u64 = 1 << 10;
pub const BB_D2: u64 = 1 << 11;
pub const BB_E2: u64 = 1 << 12;
pub const BB_F2: u64 = 1 << 13;
pub const BB_G2: u64 = 1 << 14;
pub const BB_H2: u64 = 1 << 15;
pub const BB_A3: u64 = 1 << 16;
pub const BB_B3: u64 = 1 << 17;
pub const BB_C3: u64 = 1 << 18;
pub const BB_D3: u64 = 1 << 19;
pub const BB_E3: u64 = 1 << 20;
pub const BB_F3: u64 = 1 << 21;
pub const BB_G3: u64 = 1 << 22;
pub const BB_H3: u64 = 1 << 23;
pub const BB_A4: u64 = 1 << 24;
pub const BB_B4: u64 = 1 << 25;
pub const BB_C4: u64 = 1 << 26;
pub const BB_D4: u64 = 1 << 27;
pub const BB_E4: u64 = 1 << 28;
pub const BB_F4: u64 = 1 << 29;
pub const BB_G4: u64 = 1 << 30;
pub const BB_H4: u64 = 1 << 31;
pub const BB_A5: u64 = 1 << 32;
pub const BB_B5: u64 = 1 << 33;
pub const BB_C5: u64 = 1 << 34;
pub const BB_D5: u64 = 1 << 35;
pub const BB_E5: u64 = 1 << 36;
pub const BB_F5: u64 = 1 << 37;
pub const BB_G5: u64 = 1 << 38;
pub const BB_H5: u64 = 1 << 39;
pub const BB_A6: u64 = 1 << 40;
pub const BB_B6: u64 = 1 << 41;
pub const BB_C6: u64 = 1 << 42;
pub const BB_D6: u64 = 1 << 43;
pub const BB_E6: u64 = 1 << 44;
pub const BB_F6: u64 = 1 << 45;
pub const BB_G6: u64 = 1 << 46;
pub const BB_H6: u64 = 1 << 47;
pub const BB_A7: u64 = 1 << 48;
pub const BB_B7: u64 = 1 << 49;
pub const BB_C7: u64 = 1 << 50;
pub const BB_D7: u64 = 1 << 51;
pub const BB_E7: u64 = 1 << 52;
pub const BB_F7: u64 = 1 << 53;
pub const BB_G7: u64 = 1 << 54;
pub const BB_H7: u64 = 1 << 55;
pub const BB_A8: u64 = 1 << 56;
pub const BB_B8: u64 = 1 << 57;
pub const BB_C8: u64 = 1 << 58;
pub const BB_D8: u64 = 1 << 59;
pub const BB_E8: u64 = 1 << 60;
pub const BB_F8: u64 = 1 << 61;
pub const BB_G8: u64 = 1 << 62;
pub const BB_H8: u64 = 1 << 63;
pub const BB_SQUARES: &'static [u64] = &[
	BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1,
	BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2,
	BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3,
	BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4,
	BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5,
	BB_A6, BB_B6, BB_C6, BB_D6, BB_E6, BB_F6, BB_G6, BB_H6,
	BB_A7, BB_B7, BB_C7, BB_D7, BB_E7, BB_F7, BB_G7, BB_H7,
	BB_A8, BB_B8, BB_C8, BB_D8, BB_E8, BB_F8, BB_G8, BB_H8
];

fn calc_light_squares() -> u64 {
	BB_SQUARES.iter().enumerate().fold(BB_VOID, |acc, (i, mask)| match i%2 {
		0 => acc,
		_ => acc | mask,
	})
}
fn calc_dark_squares() -> u64 {
	BB_SQUARES.iter().enumerate().fold(BB_VOID, |acc, (i, mask)| match i%2 {
		0 => acc | mask,
		_ => acc,
	})
}


lazy_static! {

    pub static ref BB_LIGHT_SQUARES: u64 = calc_light_squares();
    pub static ref BB_DARK_SQUARES: u64 = calc_dark_squares();
}

pub const BB_FILE_A: u64 = BB_A1 | BB_A2 | BB_A3 | BB_A4 | BB_A5 | BB_A6 | BB_A7 | BB_A8;
pub const BB_FILE_B: u64 = BB_B1 | BB_B2 | BB_B3 | BB_B4 | BB_B5 | BB_B6 | BB_B7 | BB_B8;
pub const BB_FILE_C: u64 = BB_C1 | BB_C2 | BB_C3 | BB_C4 | BB_C5 | BB_C6 | BB_C7 | BB_C8;
pub const BB_FILE_D: u64 = BB_D1 | BB_D2 | BB_D3 | BB_D4 | BB_D5 | BB_D6 | BB_D7 | BB_D8;
pub const BB_FILE_E: u64 = BB_E1 | BB_E2 | BB_E3 | BB_E4 | BB_E5 | BB_E6 | BB_E7 | BB_E8;
pub const BB_FILE_F: u64 = BB_F1 | BB_F2 | BB_F3 | BB_F4 | BB_F5 | BB_F6 | BB_F7 | BB_F8;
pub const BB_FILE_G: u64 = BB_G1 | BB_G2 | BB_G3 | BB_G4 | BB_G5 | BB_G6 | BB_G7 | BB_G8;
pub const BB_FILE_H: u64 = BB_H1 | BB_H2 | BB_H3 | BB_H4 | BB_H5 | BB_H6 | BB_H7 | BB_H8;

pub const BB_FILES: &'static [u64] = &[
	BB_FILE_A,
	BB_FILE_B,
	BB_FILE_C,
	BB_FILE_D,
	BB_FILE_E,
	BB_FILE_F,
	BB_FILE_G,
	BB_FILE_H
];

lazy_static! {
    pub static ref FILE_MASK: HashMap<u64, u8> = {
        let mut file_masks = HashMap::new();
        file_masks.insert(0u64, 0u8);
        for (square_index, mask) in BB_SQUARES.iter().enumerate() {
            file_masks.insert(*mask, file_index(square_index as u8));
        }
        file_masks
    };
}

pub const BB_RANK_1: u64 = BB_A1 | BB_B1 | BB_C1 | BB_D1 | BB_E1 | BB_F1 | BB_G1 | BB_H1;
pub const BB_RANK_2: u64 = BB_A2 | BB_B2 | BB_C2 | BB_D2 | BB_E2 | BB_F2 | BB_G2 | BB_H2;
pub const BB_RANK_3: u64 = BB_A3 | BB_B3 | BB_C3 | BB_D3 | BB_E3 | BB_F3 | BB_G3 | BB_H3;
pub const BB_RANK_4: u64 = BB_A4 | BB_B4 | BB_C4 | BB_D4 | BB_E4 | BB_F4 | BB_G4 | BB_H4;
pub const BB_RANK_5: u64 = BB_A5 | BB_B5 | BB_C5 | BB_D5 | BB_E5 | BB_F5 | BB_G5 | BB_H5;
pub const BB_RANK_6: u64 = BB_A6 | BB_B6 | BB_C6 | BB_D6 | BB_E6 | BB_F6 | BB_G6 | BB_H6;
pub const BB_RANK_7: u64 = BB_A7 | BB_B7 | BB_C7 | BB_D7 | BB_E7 | BB_F7 | BB_G7 | BB_H7;
pub const BB_RANK_8: u64 = BB_A8 | BB_B8 | BB_C8 | BB_D8 | BB_E8 | BB_F8 | BB_G8 | BB_H8;

pub const BB_RANKS: &'static [u64] = &[
	BB_RANK_1,
	BB_RANK_2,
	BB_RANK_3,
	BB_RANK_4,
	BB_RANK_5,
	BB_RANK_6,
	BB_RANK_7,
	BB_RANK_8
];

lazy_static! {
    pub static ref RANK_MASK: HashMap<u64, u8> = {
        let mut rank_masks = HashMap::new();
        rank_masks.insert(0u64, 0u8);
        for (square_index, mask) in BB_SQUARES.iter().enumerate() {
            rank_masks.insert(*mask, rank_index(square_index as u8));
        }
        rank_masks
    };
}

lazy_static! {
    pub static ref DIAG_MASK_NW: HashMap<u64, u8> = {
        let mut diag_mask_nw = HashMap::new();
        diag_mask_nw.insert(0u64, 0u8);
        for i in 0u64..8 {
            diag_mask_nw.insert(1 << i,  0u8);
            for j in 0u64..i+1u64 {
                let mask = diag_mask_nw.entry(1 << i).or_insert(0u8); 
                *mask |= 1 << (i + 7 * j)
            }
            for j in 0u64..i+1u64 {
                let value = *diag_mask_nw.entry(1 << i).or_insert(0u8);
                let mask = diag_mask_nw.entry(1 << (i + 7 * j)).or_insert(0u8);
                *mask = value;
            }
        }
        for i in 63u64..55 {
            diag_mask_nw.insert(1 << i, 0);
            for j in 0..64-i {
                let mask = diag_mask_nw.entry(1 << i).or_insert(0u8);
                *mask |= 1 << (i - 7 * j);
            }
            for j in 0..64-i {
                let value = *diag_mask_nw.entry(1 << i).or_insert(0u8);
                let mask = diag_mask_nw.entry(1 << (i - 7 * j)).or_insert(0u8);
                *mask = value;
            }
        }
        diag_mask_nw
    };
}

lazy_static! {
    pub static ref DIAG_MASK_NE: HashMap<u64, u8> = {
        let mut diag_mask_ne = HashMap::new();
        diag_mask_ne.insert(0u64, 0u8);
        for u in 7i64..-1 {
            // TODO: ewwwww
            let i = u as u64;
            diag_mask_ne.insert(1 << i, 0);
            for j in 0..8 - i {
                let mask = diag_mask_ne.entry(1 << i).or_insert(0);
                *mask |= 1 << (i + 9 * j);
            }
            for j in 0..8 - i {
                let value = *diag_mask_ne.entry(1 <<i).or_insert(0);
                let mask = diag_mask_ne.entry(1 << (i + 9 * j)).or_insert(0);
                *mask = value;
            }
        }

        for i in 56u64..64 {
            diag_mask_ne.insert(1 << i, 0);
            for j in 0..i-55 {
                let mask = diag_mask_ne.entry(1 << i).or_insert(0);
                *mask |= 1 << (i - 9 * j);
            }
            for j in 0..i-55 {
                let value = *diag_mask_ne.entry(1 << i).or_insert(0);
                let mask = diag_mask_ne.entry(1 << (i - 9 * j)).or_insert(0);
                *mask = value;
            }
        }
        diag_mask_ne
    };
}

// TODO: these are probably not worth it, but leaving them in until I finish
// porting
pub fn pop_count(b: u64) -> u32 {
    64 - b.count_zeros()
}

pub fn bit_scan(b: u64) -> u32 {
    b.trailing_zeros()
}

pub fn shift_down(b: u64) -> u64 {
    b >> 8
}

pub fn shift_2_down(b: u64) -> u64 {
    b >> 16
}

pub fn shift_up(b: u64) -> u64 {
    (b << 8) & BB_ALL
}

pub fn shift_2_up(b: u64) -> u64 {
    (b << 16) & BB_ALL
}

pub fn shift_right(b: u64) -> u64 {
    (b << 1) & !BB_FILE_A & BB_ALL
}

pub fn shift_2_right(b: u64) -> u64 {
    (b << 2) & !BB_FILE_A & !BB_FILE_B & BB_ALL
}

pub fn shift_left(b: u64) -> u64 {
    (b >> 1) & !BB_FILE_H
}

pub fn shift_2_left(b: u64) -> u64 {
    (b >> 2) & !BB_FILE_G & !BB_FILE_H
}

pub fn shift_up_left(b: u64) -> u64 {
    (b << 7) & !BB_FILE_H & BB_ALL
}

pub fn shift_up_right(b: u64) -> u64 {
    (b << 9) & !BB_FILE_A & BB_ALL
}

pub fn shift_down_left(b: u64) -> u64 {
    (b >> 9) & !BB_FILE_H
}

pub fn shift_down_right(b: u64) -> u64 {
    (b >> 7) & !BB_FILE_A
}

lazy_static! {
    pub static ref BB_KNIGHT_ATTACKS: Vec<u64> = {
        let mut bb_knight_attacks = Vec::new();
        for bb_square in BB_SQUARES.iter().cloned() {
            bb_knight_attacks.push(
                  shift_left(shift_2_up(bb_square))
                | shift_right(shift_2_up(bb_square))
                | shift_left(shift_2_down(bb_square))
                | shift_right(shift_2_down(bb_square))
                | shift_2_left(shift_up(bb_square))
                | shift_2_right(shift_up(bb_square))
                | shift_2_left(shift_down(bb_square))
                | shift_2_right(shift_down(bb_square))
            );
        }
        bb_knight_attacks
    };
}

lazy_static! {
    pub static ref BB_KING_ATTACKS: Vec<u64> = {
        let mut bb_king_attacks = Vec::new();
        for bb_square in BB_SQUARES.iter().cloned() {
            bb_king_attacks.push(
                  shift_left(bb_square)
                | shift_right(bb_square)
                | shift_up(bb_square)
                | shift_down(bb_square)
                | shift_up_left(bb_square)
                | shift_up_right(bb_square)
                | shift_down_left(bb_square)
                | shift_down_right(bb_square)
            );
        }
        bb_king_attacks
    };
}

lazy_static! {
    pub static ref BB_PAWN_ATTACKS: HashMap<Color, Vec<u64>> = {
        let mut bb_white_pawn_attacks = Vec::new();
        let mut bb_black_pawn_attacks = Vec::new();
        for bb_square in BB_SQUARES.iter().cloned() {
            bb_white_pawn_attacks.push(
                shift_up_left(bb_square) | shift_up_right(bb_square)
            );
            bb_black_pawn_attacks.push(
                shift_down_left(bb_square) | shift_down_right(bb_square)
            );
        }
        let mut bb_pawn_attacks: HashMap<Color, Vec<u64>> = HashMap::new();
        bb_pawn_attacks.insert(Color::White, bb_white_pawn_attacks);
        bb_pawn_attacks.insert(Color::Black, bb_black_pawn_attacks);
        bb_pawn_attacks
    };
}

fn _attack_table(square_lists: Vec<Vec<u64>>) -> HashMap<u64, HashMap<u64, u64>> {
    let mut attack_table: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    attack_table.insert(0, HashMap::new());
    attack_table.entry(0).or_insert_with(HashMap::new).insert(0, 0);
    for square_list in square_lists.iter() {
        let list_size = square_list.len();
        for current_position in 0..list_size {
            let current_bb = square_list[current_position];
            let mut sub_attack_table = attack_table.entry(current_bb).or_insert_with(HashMap::new);
            for occupation in 0u64..(1<<list_size) {
                let mut moves = 0u64;

                // TODO: There is likely some better more idiomatic way to to this in Rust. 
                for newsquare in num::range((current_position+1), list_size) {
                    moves |= square_list[newsquare];
                    if ((1 << newsquare) & occupation) != 0 {
                        break;
                    }
                }
                for newsquare in num::range_inclusive((current_position-1), 0) {
                    moves |= square_list[newsquare];
                    if ((1 << newsquare) & occupation) != 0 {
                        break;
                    }
                }

                let mut temp_bb = 0u64;
                let mut occupation = occupation;
                while occupation != 0 {
                    let lowest = 1 << (occupation.trailing_zeros()+1);
                    // Yes, unwrap is bad. however, AFAICT, this algorithm
                    // should never fail to find one. If it does, I want it to 
                    // crash
                    let i = BB_SQUARES.iter().position(|x| *x == lowest).unwrap();
                    temp_bb |= square_list[i];
                    occupation = occupation &  (occupation - 1)
                }
                sub_attack_table.insert(temp_bb,  moves);
            }
        }
    }
    attack_table
}

lazy_static! {
    pub static ref DIAG_ATTACKS_NE: HashMap<u64, HashMap<u64,u64>> = {
        _attack_table(vec![
                                        vec![BB_H1],
                                    vec![BB_H2, BB_G1],
                                vec![BB_H3, BB_G2, BB_F1],
                            vec![BB_H4, BB_G3, BB_F2, BB_E1],
                        vec![BB_H5, BB_G4, BB_F3, BB_E2, BB_D1],
                    vec![BB_H6, BB_G5, BB_F4, BB_E3, BB_D2, BB_C1],
                vec![BB_H7, BB_G6, BB_F5, BB_E4, BB_D3, BB_C2, BB_B1],
            vec![BB_H8, BB_G7, BB_F6, BB_E5, BB_D4, BB_C3, BB_B2, BB_A1],
                vec![BB_G8, BB_F7, BB_E6, BB_D5, BB_C4, BB_B3, BB_A2],
                    vec![BB_F8, BB_E7, BB_D6, BB_C5, BB_B4, BB_A3],
                        vec![BB_E8, BB_D7, BB_C6, BB_B5, BB_A4],
                            vec![BB_D8, BB_C7, BB_B6, BB_A5],
                                vec![BB_C8, BB_B7, BB_A6],
                                    vec![BB_B8, BB_A7],
                                        vec![BB_A8]
        ])
    };
}
lazy_static! {
    pub static ref DIAG_ATTACKS_NW: HashMap<u64, HashMap<u64,u64>> = {
        _attack_table(vec![
                                    vec![BB_A1],
                                vec![BB_B1, BB_A2],
                            vec![BB_C1, BB_B2, BB_A3],
                        vec![BB_D1, BB_C2, BB_B3, BB_A4],
                    vec![BB_E1, BB_D2, BB_C3, BB_B4, BB_A5],
                vec![BB_F1, BB_E2, BB_D3, BB_C4, BB_B5, BB_A6],
            vec![BB_G1, BB_F2, BB_E3, BB_D4, BB_C5, BB_B6, BB_A7],
        vec![BB_H1, BB_G2, BB_F3, BB_E4, BB_D5, BB_C6, BB_B7, BB_A8],
            vec![BB_H2, BB_G3, BB_F4, BB_E5, BB_D6, BB_C7, BB_B8],
                vec![BB_H3, BB_G4, BB_F5, BB_E6, BB_D7, BB_C8],
                    vec![BB_H4, BB_G5, BB_F6, BB_E7, BB_D8],
                        vec![BB_H5, BB_G6, BB_F7, BB_E8],
                            vec![BB_H6, BB_G7, BB_F8],
                                vec![BB_H7, BB_G8],
                                    vec![BB_H8]
        ])
    };
}
lazy_static! {
    pub static ref FILE_ATTACKS: HashMap<u64, HashMap<u64,u64>> = {
        _attack_table(vec![
            vec![BB_A1, BB_A2, BB_A3, BB_A4, BB_A5, BB_A6, BB_A7, BB_A8],
            vec![BB_B1, BB_B2, BB_B3, BB_B4, BB_B5, BB_B6, BB_B7, BB_B8],
            vec![BB_C1, BB_C2, BB_C3, BB_C4, BB_C5, BB_C6, BB_C7, BB_C8],
            vec![BB_D1, BB_D2, BB_D3, BB_D4, BB_D5, BB_D6, BB_D7, BB_D8],
            vec![BB_E1, BB_E2, BB_E3, BB_E4, BB_E5, BB_E6, BB_E7, BB_E8],
            vec![BB_F1, BB_F2, BB_F3, BB_F4, BB_F5, BB_F6, BB_F7, BB_F8],
            vec![BB_G1, BB_G2, BB_G3, BB_G4, BB_G5, BB_G6, BB_G7, BB_G8],
            vec![BB_H1, BB_H2, BB_H3, BB_H4, BB_H5, BB_H6, BB_H7, BB_H8]
        ])
    };
}
lazy_static! {
    pub static ref RANK_ATTACKS: HashMap<u64, HashMap<u64,u64>> = {
        _attack_table(vec![
            vec![BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1],
            vec![BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2],
            vec![BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3],
            vec![BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4],
            vec![BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5],
            vec![BB_A6, BB_B6, BB_C6, BB_D6, BB_E6, BB_F6, BB_G6, BB_H6],
            vec![BB_A7, BB_B7, BB_C7, BB_D7, BB_E7, BB_F7, BB_G7, BB_H7],
            vec![BB_A8, BB_B8, BB_C8, BB_D8, BB_E8, BB_F8, BB_G8, BB_H8]
        ])
    };
}

pub const POLYGLOT_RANDOM_ARRAY: &'static [u64] = &[
    0x9D39247E33776D41, 0x2AF7398005AAA5C7, 0x44DB015024623547, 0x9C15F73E62A76AE2,
    0x75834465489C0C89, 0x3290AC3A203001BF, 0x0FBBAD1F61042279, 0xE83A908FF2FB60CA,
    0x0D7E765D58755C10, 0x1A083822CEAFE02D, 0x9605D5F0E25EC3B0, 0xD021FF5CD13A2ED5,
    0x40BDF15D4A672E32, 0x011355146FD56395, 0x5DB4832046F3D9E5, 0x239F8B2D7FF719CC,
    0x05D1A1AE85B49AA1, 0x679F848F6E8FC971, 0x7449BBFF801FED0B, 0x7D11CDB1C3B7ADF0,
    0x82C7709E781EB7CC, 0xF3218F1C9510786C, 0x331478F3AF51BBE6, 0x4BB38DE5E7219443,
    0xAA649C6EBCFD50FC, 0x8DBD98A352AFD40B, 0x87D2074B81D79217, 0x19F3C751D3E92AE1,
    0xB4AB30F062B19ABF, 0x7B0500AC42047AC4, 0xC9452CA81A09D85D, 0x24AA6C514DA27500,
    0x4C9F34427501B447, 0x14A68FD73C910841, 0xA71B9B83461CBD93, 0x03488B95B0F1850F,
    0x637B2B34FF93C040, 0x09D1BC9A3DD90A94, 0x3575668334A1DD3B, 0x735E2B97A4C45A23,
    0x18727070F1BD400B, 0x1FCBACD259BF02E7, 0xD310A7C2CE9B6555, 0xBF983FE0FE5D8244,
    0x9F74D14F7454A824, 0x51EBDC4AB9BA3035, 0x5C82C505DB9AB0FA, 0xFCF7FE8A3430B241,
    0x3253A729B9BA3DDE, 0x8C74C368081B3075, 0xB9BC6C87167C33E7, 0x7EF48F2B83024E20,
    0x11D505D4C351BD7F, 0x6568FCA92C76A243, 0x4DE0B0F40F32A7B8, 0x96D693460CC37E5D,
    0x42E240CB63689F2F, 0x6D2BDCDAE2919661, 0x42880B0236E4D951, 0x5F0F4A5898171BB6,
    0x39F890F579F92F88, 0x93C5B5F47356388B, 0x63DC359D8D231B78, 0xEC16CA8AEA98AD76,
    0x5355F900C2A82DC7, 0x07FB9F855A997142, 0x5093417AA8A7ED5E, 0x7BCBC38DA25A7F3C,
    0x19FC8A768CF4B6D4, 0x637A7780DECFC0D9, 0x8249A47AEE0E41F7, 0x79AD695501E7D1E8,
    0x14ACBAF4777D5776, 0xF145B6BECCDEA195, 0xDABF2AC8201752FC, 0x24C3C94DF9C8D3F6,
    0xBB6E2924F03912EA, 0x0CE26C0B95C980D9, 0xA49CD132BFBF7CC4, 0xE99D662AF4243939,
    0x27E6AD7891165C3F, 0x8535F040B9744FF1, 0x54B3F4FA5F40D873, 0x72B12C32127FED2B,
    0xEE954D3C7B411F47, 0x9A85AC909A24EAA1, 0x70AC4CD9F04F21F5, 0xF9B89D3E99A075C2,
    0x87B3E2B2B5C907B1, 0xA366E5B8C54F48B8, 0xAE4A9346CC3F7CF2, 0x1920C04D47267BBD,
    0x87BF02C6B49E2AE9, 0x092237AC237F3859, 0xFF07F64EF8ED14D0, 0x8DE8DCA9F03CC54E,
    0x9C1633264DB49C89, 0xB3F22C3D0B0B38ED, 0x390E5FB44D01144B, 0x5BFEA5B4712768E9,
    0x1E1032911FA78984, 0x9A74ACB964E78CB3, 0x4F80F7A035DAFB04, 0x6304D09A0B3738C4,
    0x2171E64683023A08, 0x5B9B63EB9CEFF80C, 0x506AACF489889342, 0x1881AFC9A3A701D6,
    0x6503080440750644, 0xDFD395339CDBF4A7, 0xEF927DBCF00C20F2, 0x7B32F7D1E03680EC,
    0xB9FD7620E7316243, 0x05A7E8A57DB91B77, 0xB5889C6E15630A75, 0x4A750A09CE9573F7,
    0xCF464CEC899A2F8A, 0xF538639CE705B824, 0x3C79A0FF5580EF7F, 0xEDE6C87F8477609D,
    0x799E81F05BC93F31, 0x86536B8CF3428A8C, 0x97D7374C60087B73, 0xA246637CFF328532,
    0x043FCAE60CC0EBA0, 0x920E449535DD359E, 0x70EB093B15B290CC, 0x73A1921916591CBD,
    0x56436C9FE1A1AA8D, 0xEFAC4B70633B8F81, 0xBB215798D45DF7AF, 0x45F20042F24F1768,
    0x930F80F4E8EB7462, 0xFF6712FFCFD75EA1, 0xAE623FD67468AA70, 0xDD2C5BC84BC8D8FC,
    0x7EED120D54CF2DD9, 0x22FE545401165F1C, 0xC91800E98FB99929, 0x808BD68E6AC10365,
    0xDEC468145B7605F6, 0x1BEDE3A3AEF53302, 0x43539603D6C55602, 0xAA969B5C691CCB7A,
    0xA87832D392EFEE56, 0x65942C7B3C7E11AE, 0xDED2D633CAD004F6, 0x21F08570F420E565,
    0xB415938D7DA94E3C, 0x91B859E59ECB6350, 0x10CFF333E0ED804A, 0x28AED140BE0BB7DD,
    0xC5CC1D89724FA456, 0x5648F680F11A2741, 0x2D255069F0B7DAB3, 0x9BC5A38EF729ABD4,
    0xEF2F054308F6A2BC, 0xAF2042F5CC5C2858, 0x480412BAB7F5BE2A, 0xAEF3AF4A563DFE43,
    0x19AFE59AE451497F, 0x52593803DFF1E840, 0xF4F076E65F2CE6F0, 0x11379625747D5AF3,
    0xBCE5D2248682C115, 0x9DA4243DE836994F, 0x066F70B33FE09017, 0x4DC4DE189B671A1C,
    0x51039AB7712457C3, 0xC07A3F80C31FB4B4, 0xB46EE9C5E64A6E7C, 0xB3819A42ABE61C87,
    0x21A007933A522A20, 0x2DF16F761598AA4F, 0x763C4A1371B368FD, 0xF793C46702E086A0,
    0xD7288E012AEB8D31, 0xDE336A2A4BC1C44B, 0x0BF692B38D079F23, 0x2C604A7A177326B3,
    0x4850E73E03EB6064, 0xCFC447F1E53C8E1B, 0xB05CA3F564268D99, 0x9AE182C8BC9474E8,
    0xA4FC4BD4FC5558CA, 0xE755178D58FC4E76, 0x69B97DB1A4C03DFE, 0xF9B5B7C4ACC67C96,
    0xFC6A82D64B8655FB, 0x9C684CB6C4D24417, 0x8EC97D2917456ED0, 0x6703DF9D2924E97E,
    0xC547F57E42A7444E, 0x78E37644E7CAD29E, 0xFE9A44E9362F05FA, 0x08BD35CC38336615,
    0x9315E5EB3A129ACE, 0x94061B871E04DF75, 0xDF1D9F9D784BA010, 0x3BBA57B68871B59D,
    0xD2B7ADEEDED1F73F, 0xF7A255D83BC373F8, 0xD7F4F2448C0CEB81, 0xD95BE88CD210FFA7,
    0x336F52F8FF4728E7, 0xA74049DAC312AC71, 0xA2F61BB6E437FDB5, 0x4F2A5CB07F6A35B3,
    0x87D380BDA5BF7859, 0x16B9F7E06C453A21, 0x7BA2484C8A0FD54E, 0xF3A678CAD9A2E38C,
    0x39B0BF7DDE437BA2, 0xFCAF55C1BF8A4424, 0x18FCF680573FA594, 0x4C0563B89F495AC3,
    0x40E087931A00930D, 0x8CFFA9412EB642C1, 0x68CA39053261169F, 0x7A1EE967D27579E2,
    0x9D1D60E5076F5B6F, 0x3810E399B6F65BA2, 0x32095B6D4AB5F9B1, 0x35CAB62109DD038A,
    0xA90B24499FCFAFB1, 0x77A225A07CC2C6BD, 0x513E5E634C70E331, 0x4361C0CA3F692F12,
    0xD941ACA44B20A45B, 0x528F7C8602C5807B, 0x52AB92BEB9613989, 0x9D1DFA2EFC557F73,
    0x722FF175F572C348, 0x1D1260A51107FE97, 0x7A249A57EC0C9BA2, 0x04208FE9E8F7F2D6,
    0x5A110C6058B920A0, 0x0CD9A497658A5698, 0x56FD23C8F9715A4C, 0x284C847B9D887AAE,
    0x04FEABFBBDB619CB, 0x742E1E651C60BA83, 0x9A9632E65904AD3C, 0x881B82A13B51B9E2,
    0x506E6744CD974924, 0xB0183DB56FFC6A79, 0x0ED9B915C66ED37E, 0x5E11E86D5873D484,
    0xF678647E3519AC6E, 0x1B85D488D0F20CC5, 0xDAB9FE6525D89021, 0x0D151D86ADB73615,
    0xA865A54EDCC0F019, 0x93C42566AEF98FFB, 0x99E7AFEABE000731, 0x48CBFF086DDF285A,
    0x7F9B6AF1EBF78BAF, 0x58627E1A149BBA21, 0x2CD16E2ABD791E33, 0xD363EFF5F0977996,
    0x0CE2A38C344A6EED, 0x1A804AADB9CFA741, 0x907F30421D78C5DE, 0x501F65EDB3034D07,
    0x37624AE5A48FA6E9, 0x957BAF61700CFF4E, 0x3A6C27934E31188A, 0xD49503536ABCA345,
    0x088E049589C432E0, 0xF943AEE7FEBF21B8, 0x6C3B8E3E336139D3, 0x364F6FFA464EE52E,
    0xD60F6DCEDC314222, 0x56963B0DCA418FC0, 0x16F50EDF91E513AF, 0xEF1955914B609F93,
    0x565601C0364E3228, 0xECB53939887E8175, 0xBAC7A9A18531294B, 0xB344C470397BBA52,
    0x65D34954DAF3CEBD, 0xB4B81B3FA97511E2, 0xB422061193D6F6A7, 0x071582401C38434D,
    0x7A13F18BBEDC4FF5, 0xBC4097B116C524D2, 0x59B97885E2F2EA28, 0x99170A5DC3115544,
    0x6F423357E7C6A9F9, 0x325928EE6E6F8794, 0xD0E4366228B03343, 0x565C31F7DE89EA27,
    0x30F5611484119414, 0xD873DB391292ED4F, 0x7BD94E1D8E17DEBC, 0xC7D9F16864A76E94,
    0x947AE053EE56E63C, 0xC8C93882F9475F5F, 0x3A9BF55BA91F81CA, 0xD9A11FBB3D9808E4,
    0x0FD22063EDC29FCA, 0xB3F256D8ACA0B0B9, 0xB03031A8B4516E84, 0x35DD37D5871448AF,
    0xE9F6082B05542E4E, 0xEBFAFA33D7254B59, 0x9255ABB50D532280, 0xB9AB4CE57F2D34F3,
    0x693501D628297551, 0xC62C58F97DD949BF, 0xCD454F8F19C5126A, 0xBBE83F4ECC2BDECB,
    0xDC842B7E2819E230, 0xBA89142E007503B8, 0xA3BC941D0A5061CB, 0xE9F6760E32CD8021,
    0x09C7E552BC76492F, 0x852F54934DA55CC9, 0x8107FCCF064FCF56, 0x098954D51FFF6580,
    0x23B70EDB1955C4BF, 0xC330DE426430F69D, 0x4715ED43E8A45C0A, 0xA8D7E4DAB780A08D,
    0x0572B974F03CE0BB, 0xB57D2E985E1419C7, 0xE8D9ECBE2CF3D73F, 0x2FE4B17170E59750,
    0x11317BA87905E790, 0x7FBF21EC8A1F45EC, 0x1725CABFCB045B00, 0x964E915CD5E2B207,
    0x3E2B8BCBF016D66D, 0xBE7444E39328A0AC, 0xF85B2B4FBCDE44B7, 0x49353FEA39BA63B1,
    0x1DD01AAFCD53486A, 0x1FCA8A92FD719F85, 0xFC7C95D827357AFA, 0x18A6A990C8B35EBD,
    0xCCCB7005C6B9C28D, 0x3BDBB92C43B17F26, 0xAA70B5B4F89695A2, 0xE94C39A54A98307F,
    0xB7A0B174CFF6F36E, 0xD4DBA84729AF48AD, 0x2E18BC1AD9704A68, 0x2DE0966DAF2F8B1C,
    0xB9C11D5B1E43A07E, 0x64972D68DEE33360, 0x94628D38D0C20584, 0xDBC0D2B6AB90A559,
    0xD2733C4335C6A72F, 0x7E75D99D94A70F4D, 0x6CED1983376FA72B, 0x97FCAACBF030BC24,
    0x7B77497B32503B12, 0x8547EDDFB81CCB94, 0x79999CDFF70902CB, 0xCFFE1939438E9B24,
    0x829626E3892D95D7, 0x92FAE24291F2B3F1, 0x63E22C147B9C3403, 0xC678B6D860284A1C,
    0x5873888850659AE7, 0x0981DCD296A8736D, 0x9F65789A6509A440, 0x9FF38FED72E9052F,
    0xE479EE5B9930578C, 0xE7F28ECD2D49EECD, 0x56C074A581EA17FE, 0x5544F7D774B14AEF,
    0x7B3F0195FC6F290F, 0x12153635B2C0CF57, 0x7F5126DBBA5E0CA7, 0x7A76956C3EAFB413,
    0x3D5774A11D31AB39, 0x8A1B083821F40CB4, 0x7B4A38E32537DF62, 0x950113646D1D6E03,
    0x4DA8979A0041E8A9, 0x3BC36E078F7515D7, 0x5D0A12F27AD310D1, 0x7F9D1A2E1EBE1327,
    0xDA3A361B1C5157B1, 0xDCDD7D20903D0C25, 0x36833336D068F707, 0xCE68341F79893389,
    0xAB9090168DD05F34, 0x43954B3252DC25E5, 0xB438C2B67F98E5E9, 0x10DCD78E3851A492,
    0xDBC27AB5447822BF, 0x9B3CDB65F82CA382, 0xB67B7896167B4C84, 0xBFCED1B0048EAC50,
    0xA9119B60369FFEBD, 0x1FFF7AC80904BF45, 0xAC12FB171817EEE7, 0xAF08DA9177DDA93D,
    0x1B0CAB936E65C744, 0xB559EB1D04E5E932, 0xC37B45B3F8D6F2BA, 0xC3A9DC228CAAC9E9,
    0xF3B8B6675A6507FF, 0x9FC477DE4ED681DA, 0x67378D8ECCEF96CB, 0x6DD856D94D259236,
    0xA319CE15B0B4DB31, 0x073973751F12DD5E, 0x8A8E849EB32781A5, 0xE1925C71285279F5,
    0x74C04BF1790C0EFE, 0x4DDA48153C94938A, 0x9D266D6A1CC0542C, 0x7440FB816508C4FE,
    0x13328503DF48229F, 0xD6BF7BAEE43CAC40, 0x4838D65F6EF6748F, 0x1E152328F3318DEA,
    0x8F8419A348F296BF, 0x72C8834A5957B511, 0xD7A023A73260B45C, 0x94EBC8ABCFB56DAE,
    0x9FC10D0F989993E0, 0xDE68A2355B93CAE6, 0xA44CFE79AE538BBE, 0x9D1D84FCCE371425,
    0x51D2B1AB2DDFB636, 0x2FD7E4B9E72CD38C, 0x65CA5B96B7552210, 0xDD69A0D8AB3B546D,
    0x604D51B25FBF70E2, 0x73AA8A564FB7AC9E, 0x1A8C1E992B941148, 0xAAC40A2703D9BEA0,
    0x764DBEAE7FA4F3A6, 0x1E99B96E70A9BE8B, 0x2C5E9DEB57EF4743, 0x3A938FEE32D29981,
    0x26E6DB8FFDF5ADFE, 0x469356C504EC9F9D, 0xC8763C5B08D1908C, 0x3F6C6AF859D80055,
    0x7F7CC39420A3A545, 0x9BFB227EBDF4C5CE, 0x89039D79D6FC5C5C, 0x8FE88B57305E2AB6,
    0xA09E8C8C35AB96DE, 0xFA7E393983325753, 0xD6B6D0ECC617C699, 0xDFEA21EA9E7557E3,
    0xB67C1FA481680AF8, 0xCA1E3785A9E724E5, 0x1CFC8BED0D681639, 0xD18D8549D140CAEA,
    0x4ED0FE7E9DC91335, 0xE4DBF0634473F5D2, 0x1761F93A44D5AEFE, 0x53898E4C3910DA55,
    0x734DE8181F6EC39A, 0x2680B122BAA28D97, 0x298AF231C85BAFAB, 0x7983EED3740847D5,
    0x66C1A2A1A60CD889, 0x9E17E49642A3E4C1, 0xEDB454E7BADC0805, 0x50B704CAB602C329,
    0x4CC317FB9CDDD023, 0x66B4835D9EAFEA22, 0x219B97E26FFC81BD, 0x261E4E4C0A333A9D,
    0x1FE2CCA76517DB90, 0xD7504DFA8816EDBB, 0xB9571FA04DC089C8, 0x1DDC0325259B27DE,
    0xCF3F4688801EB9AA, 0xF4F5D05C10CAB243, 0x38B6525C21A42B0E, 0x36F60E2BA4FA6800,
    0xEB3593803173E0CE, 0x9C4CD6257C5A3603, 0xAF0C317D32ADAA8A, 0x258E5A80C7204C4B,
    0x8B889D624D44885D, 0xF4D14597E660F855, 0xD4347F66EC8941C3, 0xE699ED85B0DFB40D,
    0x2472F6207C2D0484, 0xC2A1E7B5B459AEB5, 0xAB4F6451CC1D45EC, 0x63767572AE3D6174,
    0xA59E0BD101731A28, 0x116D0016CB948F09, 0x2CF9C8CA052F6E9F, 0x0B090A7560A968E3,
    0xABEEDDB2DDE06FF1, 0x58EFC10B06A2068D, 0xC6E57A78FBD986E0, 0x2EAB8CA63CE802D7,
    0x14A195640116F336, 0x7C0828DD624EC390, 0xD74BBE77E6116AC7, 0x804456AF10F5FB53,
    0xEBE9EA2ADF4321C7, 0x03219A39EE587A30, 0x49787FEF17AF9924, 0xA1E9300CD8520548,
    0x5B45E522E4B1B4EF, 0xB49C3B3995091A36, 0xD4490AD526F14431, 0x12A8F216AF9418C2,
    0x001F837CC7350524, 0x1877B51E57A764D5, 0xA2853B80F17F58EE, 0x993E1DE72D36D310,
    0xB3598080CE64A656, 0x252F59CF0D9F04BB, 0xD23C8E176D113600, 0x1BDA0492E7E4586E,
    0x21E0BD5026C619BF, 0x3B097ADAF088F94E, 0x8D14DEDB30BE846E, 0xF95CFFA23AF5F6F4,
    0x3871700761B3F743, 0xCA672B91E9E4FA16, 0x64C8E531BFF53B55, 0x241260ED4AD1E87D,
    0x106C09B972D2E822, 0x7FBA195410E5CA30, 0x7884D9BC6CB569D8, 0x0647DFEDCD894A29,
    0x63573FF03E224774, 0x4FC8E9560F91B123, 0x1DB956E450275779, 0xB8D91274B9E9D4FB,
    0xA2EBEE47E2FBFCE1, 0xD9F1F30CCD97FB09, 0xEFED53D75FD64E6B, 0x2E6D02C36017F67F,
    0xA9AA4D20DB084E9B, 0xB64BE8D8B25396C1, 0x70CB6AF7C2D5BCF0, 0x98F076A4F7A2322E,
    0xBF84470805E69B5F, 0x94C3251F06F90CF3, 0x3E003E616A6591E9, 0xB925A6CD0421AFF3,
    0x61BDD1307C66E300, 0xBF8D5108E27E0D48, 0x240AB57A8B888B20, 0xFC87614BAF287E07,
    0xEF02CDD06FFDB432, 0xA1082C0466DF6C0A, 0x8215E577001332C8, 0xD39BB9C3A48DB6CF,
    0x2738259634305C14, 0x61CF4F94C97DF93D, 0x1B6BACA2AE4E125B, 0x758F450C88572E0B,
    0x959F587D507A8359, 0xB063E962E045F54D, 0x60E8ED72C0DFF5D1, 0x7B64978555326F9F,
    0xFD080D236DA814BA, 0x8C90FD9B083F4558, 0x106F72FE81E2C590, 0x7976033A39F7D952,
    0xA4EC0132764CA04B, 0x733EA705FAE4FA77, 0xB4D8F77BC3E56167, 0x9E21F4F903B33FD9,
    0x9D765E419FB69F6D, 0xD30C088BA61EA5EF, 0x5D94337FBFAF7F5B, 0x1A4E4822EB4D7A59,
    0x6FFE73E81B637FB3, 0xDDF957BC36D8B9CA, 0x64D0E29EEA8838B3, 0x08DD9BDFD96B9F63,
    0x087E79E5A57D1D13, 0xE328E230E3E2B3FB, 0x1C2559E30F0946BE, 0x720BF5F26F4D2EAA,
    0xB0774D261CC609DB, 0x443F64EC5A371195, 0x4112CF68649A260E, 0xD813F2FAB7F5C5CA,
    0x660D3257380841EE, 0x59AC2C7873F910A3, 0xE846963877671A17, 0x93B633ABFA3469F8,
    0xC0C0F5A60EF4CDCF, 0xCAF21ECD4377B28C, 0x57277707199B8175, 0x506C11B9D90E8B1D,
    0xD83CC2687A19255F, 0x4A29C6465A314CD1, 0xED2DF21216235097, 0xB5635C95FF7296E2,
    0x22AF003AB672E811, 0x52E762596BF68235, 0x9AEBA33AC6ECC6B0, 0x944F6DE09134DFB6,
    0x6C47BEC883A7DE39, 0x6AD047C430A12104, 0xA5B1CFDBA0AB4067, 0x7C45D833AFF07862,
    0x5092EF950A16DA0B, 0x9338E69C052B8E7B, 0x455A4B4CFE30E3F5, 0x6B02E63195AD0CF8,
    0x6B17B224BAD6BF27, 0xD1E0CCD25BB9C169, 0xDE0C89A556B9AE70, 0x50065E535A213CF6,
    0x9C1169FA2777B874, 0x78EDEFD694AF1EED, 0x6DC93D9526A50E68, 0xEE97F453F06791ED,
    0x32AB0EDB696703D3, 0x3A6853C7E70757A7, 0x31865CED6120F37D, 0x67FEF95D92607890,
    0x1F2B1D1F15F6DC9C, 0xB69E38A8965C6B65, 0xAA9119FF184CCCF4, 0xF43C732873F24C13,
    0xFB4A3D794A9A80D2, 0x3550C2321FD6109C, 0x371F77E76BB8417E, 0x6BFA9AAE5EC05779,
    0xCD04F3FF001A4778, 0xE3273522064480CA, 0x9F91508BFFCFC14A, 0x049A7F41061A9E60,
    0xFCB6BE43A9F2FE9B, 0x08DE8A1C7797DA9B, 0x8F9887E6078735A1, 0xB5B4071DBFC73A66,
    0x230E343DFBA08D33, 0x43ED7F5A0FAE657D, 0x3A88A0FBBCB05C63, 0x21874B8B4D2DBC4F,
    0x1BDEA12E35F6A8C9, 0x53C065C6C8E63528, 0xE34A1D250E7A8D6B, 0xD6B04D3B7651DD7E,
    0x5E90277E7CB39E2D, 0x2C046F22062DC67D, 0xB10BB459132D0A26, 0x3FA9DDFB67E2F199,
    0x0E09B88E1914F7AF, 0x10E8B35AF3EEAB37, 0x9EEDECA8E272B933, 0xD4C718BC4AE8AE5F,
    0x81536D601170FC20, 0x91B534F885818A06, 0xEC8177F83F900978, 0x190E714FADA5156E,
    0xB592BF39B0364963, 0x89C350C893AE7DC1, 0xAC042E70F8B383F2, 0xB49B52E587A1EE60,
    0xFB152FE3FF26DA89, 0x3E666E6F69AE2C15, 0x3B544EBE544C19F9, 0xE805A1E290CF2456,
    0x24B33C9D7ED25117, 0xE74733427B72F0C1, 0x0A804D18B7097475, 0x57E3306D881EDB4F,
    0x4AE7D6A36EB5DBCB, 0x2D8D5432157064C8, 0xD1E649DE1E7F268B, 0x8A328A1CEDFE552C,
    0x07A3AEC79624C7DA, 0x84547DDC3E203C94, 0x990A98FD5071D263, 0x1A4FF12616EEFC89,
    0xF6F7FD1431714200, 0x30C05B1BA332F41C, 0x8D2636B81555A786, 0x46C9FEB55D120902,
    0xCCEC0A73B49C9921, 0x4E9D2827355FC492, 0x19EBB029435DCB0F, 0x4659D2B743848A2C,
    0x963EF2C96B33BE31, 0x74F85198B05A2E7D, 0x5A0F544DD2B1FB18, 0x03727073C2E134B1,
    0xC7F6AA2DE59AEA61, 0x352787BAA0D7C22F, 0x9853EAB63B5E0B35, 0xABBDCDD7ED5C0860,
    0xCF05DAF5AC8D77B0, 0x49CAD48CEBF4A71E, 0x7A4C10EC2158C4A6, 0xD9E92AA246BF719E,
    0x13AE978D09FE5557, 0x730499AF921549FF, 0x4E4B705B92903BA4, 0xFF577222C14F0A3A,
    0x55B6344CF97AAFAE, 0xB862225B055B6960, 0xCAC09AFBDDD2CDB4, 0xDAF8E9829FE96B5F,
    0xB5FDFC5D3132C498, 0x310CB380DB6F7503, 0xE87FBB46217A360E, 0x2102AE466EBB1148,
    0xF8549E1A3AA5E00D, 0x07A69AFDCC42261A, 0xC4C118BFE78FEAAE, 0xF9F4892ED96BD438,
    0x1AF3DBE25D8F45DA, 0xF5B4B0B0D2DEEEB4, 0x962ACEEFA82E1C84, 0x046E3ECAAF453CE9,
    0xF05D129681949A4C, 0x964781CE734B3C84, 0x9C2ED44081CE5FBD, 0x522E23F3925E319E,
    0x177E00F9FC32F791, 0x2BC60A63A6F3B3F2, 0x222BBFAE61725606, 0x486289DDCC3D6780,
    0x7DC7785B8EFDFC80, 0x8AF38731C02BA980, 0x1FAB64EA29A2DDF7, 0xE4D9429322CD065A,
    0x9DA058C67844F20C, 0x24C0E332B70019B0, 0x233003B5A6CFE6AD, 0xD586BD01C5C217F6,
    0x5E5637885F29BC2B, 0x7EBA726D8C94094B, 0x0A56A5F0BFE39272, 0xD79476A84EE20D06,
    0x9E4C1269BAA4BF37, 0x17EFEE45B0DEE640, 0x1D95B0A5FCF90BC6, 0x93CBE0B699C2585D,
    0x65FA4F227A2B6D79, 0xD5F9E858292504D5, 0xC2B5A03F71471A6F, 0x59300222B4561E00,
    0xCE2F8642CA0712DC, 0x7CA9723FBB2E8988, 0x2785338347F2BA08, 0xC61BB3A141E50E8C,
    0x150F361DAB9DEC26, 0x9F6A419D382595F4, 0x64A53DC924FE7AC9, 0x142DE49FFF7A7C3D,
    0x0C335248857FA9E7, 0x0A9C32D5EAE45305, 0xE6C42178C4BBB92E, 0x71F1CE2490D20B07,
    0xF1BCC3D275AFE51A, 0xE728E8C83C334074, 0x96FBF83A12884624, 0x81A1549FD6573DA5,
    0x5FA7867CAF35E149, 0x56986E2EF3ED091B, 0x917F1DD5F8886C61, 0xD20D8C88C8FFE65F,
    0x31D71DCE64B2C310, 0xF165B587DF898190, 0xA57E6339DD2CF3A0, 0x1EF6E6DBB1961EC9,
    0x70CC73D90BC26E24, 0xE21A6B35DF0C3AD7, 0x003A93D8B2806962, 0x1C99DED33CB890A1,
    0xCF3145DE0ADD4289, 0xD0E4427A5514FB72, 0x77C621CC9FB3A483, 0x67A34DAC4356550B,
    0xF8D626AAAF278509
];

lazy_static! {
    pub static ref SAN_REGEX: Regex = Regex::new("^([NBKRQ])?([a-h])?([1-8])?x?([a-h][1-8])(=?[nbrqNBRQ])?(\\+|#)?$").unwrap();
}

lazy_static! {
    pub static ref FEN_CASTLING_REGEX: Regex = Regex::new("^-|[KQABCDEFGH]{0,2}[kqabcdefgh]{0,2}$").unwrap();
}


#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct Piece {
    piece_type: PieceType,
    color: Color
}

impl Piece {
    pub fn from_symbol(symbol: char) -> Option<Piece> {
        match symbol {
            'P' => Some(Piece{piece_type: PieceType::Pawn, color: Color::White}),
            'N' => Some(Piece{piece_type: PieceType::Knight, color: Color::White}),
            'B' => Some(Piece{piece_type: PieceType::Bishop, color: Color::White}),
            'R' => Some(Piece{piece_type: PieceType::Rook, color: Color::White}),
            'K' => Some(Piece{piece_type: PieceType::King, color: Color::White}),
            'Q' => Some(Piece{piece_type: PieceType::Queen, color: Color::White}),
            'p' => Some(Piece{piece_type: PieceType::Pawn, color: Color::Black}),
            'n' => Some(Piece{piece_type: PieceType::Knight, color: Color::Black}),
            'b' => Some(Piece{piece_type: PieceType::Bishop, color: Color::Black}),
            'r' => Some(Piece{piece_type: PieceType::Rook, color: Color::Black}),
            'k' => Some(Piece{piece_type: PieceType::King, color: Color::Black}),
            'q' => Some(Piece{piece_type: PieceType::Queen, color: Color::Black}),
            _ => None
        }
    }
    pub fn symbol(&self) -> char {
        match self.color {
            Color::White => PIECE_SYMBOLS_WHITE[self.piece_type as usize],
            Color::Black => PIECE_SYMBOLS_BLACK[self.piece_type as usize]
        }
    }

    pub fn unicode_symbol(&self) -> char {
        match self.color {
            Color::White => UNICODE_PIECE_SYMBOLS_WHITE[self.piece_type as usize],
            Color::Black => UNICODE_PIECE_SYMBOLS_BLACK[self.piece_type as usize],
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.unicode_symbol())
    }
}
impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Piece.from_symbol('{}')", self.symbol())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_is_properly_initalized() {
        for i in 0..64 {
            assert!(SQUARES[i] == i as u8);
        }
    }
    #[test]
    fn test_square() {
        for sq in SQUARES {
            let file_index = file_index(*sq);
            let rank_index = rank_index(*sq);
            assert!(square(file_index, rank_index) == *sq);
        }
    }

    fn test_shift<F>(shift: F)
        where F: Fn(u64) -> u64
    {
        for bb_square in BB_SQUARES {
            let shifted = shift(*bb_square);
            let c = pop_count(shifted);
            assert!(c <= 1);
            assert!(c == pop_count(shifted & BB_ALL));
        }
    }
    #[test]
    fn test_shifts() {
        test_shift(|x| shift_down(x));
        test_shift(|x| shift_2_down(x));
        test_shift(|x| shift_up(x));
        test_shift(|x| shift_2_up(x));
        test_shift(|x| shift_right(x));
        test_shift(|x| shift_2_right(x));
        test_shift(|x| shift_left(x));
        test_shift(|x| shift_2_left(x));
        test_shift(|x| shift_up_left(x));
        test_shift(|x| shift_up_right(x));
        test_shift(|x| shift_down_left(x));
        test_shift(|x| shift_down_right(x));
    }

    #[test]
    fn test_bit_scan() {
        assert!(0 == bit_scan(0b00000001));
        assert!(1 == bit_scan(0b00000010));
        assert!(2 == bit_scan(0b00000100));
        assert!(3 == bit_scan(0b00001000));
        assert!(4 == bit_scan(0b00010000));
        assert!(5 == bit_scan(0b00100000));
        assert!(6 == bit_scan(0b01000000));
        assert!(7 == bit_scan(0b10000000));

        assert!(0 == bit_scan(0b00000011));
        assert!(1 == bit_scan(0b00000110));
        assert!(2 == bit_scan(0b00001100));
        assert!(3 == bit_scan(0b00011000));
        assert!(4 == bit_scan(0b00110000));
        assert!(5 == bit_scan(0b01100000));
        assert!(6 == bit_scan(0b11000000));
        assert!(7 == bit_scan(0b10000000));

        assert!(0 == bit_scan(0b11111111));
        assert!(1 == bit_scan(0b11111110));
        assert!(2 == bit_scan(0b11111100));
        assert!(3 == bit_scan(0b11111000));
        assert!(4 == bit_scan(0b11110000));
        assert!(5 == bit_scan(0b11100000));
        assert!(6 == bit_scan(0b11000000));
        assert!(7 == bit_scan(0b10000000));
    }

    #[test]
    fn test_piece_equality() {
        let a = Piece{piece_type: PieceType::Bishop, color: Color::White};
        let b = Piece{piece_type: PieceType::King, color: Color::Black};
        let c = Piece{piece_type: PieceType::King, color: Color::White};
        let d1 = Piece{piece_type: PieceType::Bishop, color: Color::White};
        let d2 = Piece{piece_type: PieceType::Bishop, color: Color::White};

        assert!(a == d1);
        assert!(d1 == a);
        assert!(d1 == d2);

		assert!(format!("{}", a) == format!("{}", d1));

        assert!(a != b);
        assert!(b != c);
        assert!(b != d1);
        assert!(a != c);
        assert!((d1 != d2) == false);

		assert!(format!("{}", a) != format!("{}", b));
		println!("{}", b);
        println!("{}", c);
		assert!(format!("{}", b) != format!("{}", c));
		assert!(format!("{}", b) != format!("{}", d1));
		assert!(format!("{}", a) != format!("{}", c));
    }
    #[test]
    fn test_from_symbol() {
        if let Some(white_knight) = Piece::from_symbol('N') {
            assert!(white_knight.color == Color::White);
            assert!(white_knight.piece_type == PieceType::Knight);
            assert!(white_knight.symbol() == 'N');
        } else {
            assert!(false, "Unable to create a white knight from symbol N");
        }

        if let Some(black_queen) = Piece::from_symbol('q') {
            assert!(black_queen.color == Color::Black);
            assert!(black_queen.piece_type == PieceType::Queen);
            assert!(black_queen.symbol() == 'q');
        } else {
            assert!(false, "Unable to create a black queen from symbol q");
        }

    }
}
