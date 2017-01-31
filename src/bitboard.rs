// lwchess - An engine focused on teaching humans.
// 
// Copyright (C) 2017 Lakin Wecker <lakin@wecker.ca>
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::types::*;

// De Bruijn sequences. See chessprogramming.wikispaces.com/BitScan
pub const DEBRUIJN_64: u64 = 0x3F79D71B4CB0A89u64;
pub const DEBRUIJN_32: u32 = 0x783A9B23u32;

pub const DARK_SQUARES: Bitboard = Bitboard(0xAA55AA55AA55AA55u64);

pub const FILE_ABB: Bitboard = Bitboard(0x0101010101010101u64);
pub const FILE_BBB: Bitboard = Bitboard(FILE_ABB.0 << 1);
pub const FILE_CBB: Bitboard = Bitboard(FILE_ABB.0 << 2);
pub const FILE_DBB: Bitboard = Bitboard(FILE_ABB.0 << 3);
pub const FILE_EBB: Bitboard = Bitboard(FILE_ABB.0 << 4);
pub const FILE_FBB: Bitboard = Bitboard(FILE_ABB.0 << 5);
pub const FILE_GBB: Bitboard = Bitboard(FILE_ABB.0 << 6);
pub const FILE_HBB: Bitboard = Bitboard(FILE_ABB.0 << 7);

pub const RANK_1BB: Bitboard = Bitboard(0xFF);
pub const RANK_2BB: Bitboard = Bitboard(RANK_1BB.0 << (8 * 1));
pub const RANK_3BB: Bitboard = Bitboard(RANK_1BB.0 << (8 * 2));
pub const RANK_4BB: Bitboard = Bitboard(RANK_1BB.0 << (8 * 3));
pub const RANK_5BB: Bitboard = Bitboard(RANK_1BB.0 << (8 * 4));
pub const RANK_6BB: Bitboard = Bitboard(RANK_1BB.0 << (8 * 5));
pub const RANK_7BB: Bitboard = Bitboard(RANK_1BB.0 << (8 * 6));
pub const RANK_8BB: Bitboard = Bitboard(RANK_1BB.0 << (8 * 7));

// popcount16() counts the non-zero bits using SWAR-Popcount algorithm
pub fn popcount16(u: u16) ->  u8 {
    let mut u = u - (u >> 1) & 0x5555u16;
    u = ((u >> 2) & 0x3333u16) + (u & 0x3333u16);
    u = ((u >> 4) + u) & 0x0F0Fu16;
    ((u * 0x0101u16) >> 8) as u8
}

// bsf_index() returns the index into BSFTable[] to look up the bitscan. Uses
// Matt Taylor's folding for 32 bit case, extended to 64 bit by Kim Walisch.
#[cfg(target_pointer_width="32")]
pub fn bsf_index(b: Bitboard) -> usize {
    let b = b.0 ^ (b.0 - 1);
    ((unsigned(b) ^ unsigned(b >> 32)) * DEBRUIJN_32) >> 26
}
#[cfg(target_pointer_width="64")]
pub fn bsf_index(b: Bitboard) -> usize {
    let b = b.0 ^ (b.0 - 1);
    ((b * DEBRUIJN_64) >> 58) as usize
}

lazy_static! {
    pub static ref POPCNT_16: [u8; 1<<16] = {
        let mut popcnt_16 = [0; 1<<16];
        for i in 0..(1<<16) {
            popcnt_16[i] = popcount16(i as u16);
        }
        popcnt_16
    };
    /*pub static ref SQUARE_DISTANCE: &'static [[i32; SQUARE_NB]; SQUARE_NB] = {

    };*/
    pub static ref SQUARE_BB: [Bitboard; 64] = {
        let mut square_bb = [Bitboard(0); 64];
        for s in (SQ_A1.0)..(SQ_H8.0) {
            square_bb[s as usize] = Bitboard(1u64 << s);
        }
        square_bb
    };
    pub static ref BSF_TABLE: [Square; 64] = {
        let mut bsf_table = [Square(0); 64];
        for s in (SQ_A1.0)..(SQ_H8.0) {
            bsf_table[bsf_index(SQUARE_BB[s as usize])] = Square(s);
        }
        bsf_table
    };

    /*
    pub Bitboard FileBB[FILE_NB];
    pub Bitboard RankBB[RANK_NB];
    pub Bitboard AdjacentFilesBB[FILE_NB];
    pub Bitboard InFrontBB[COLOR_NB][RANK_NB];
    pub Bitboard StepAttacksBB[PIECE_NB][SQUARE_NB];
    pub Bitboard BetweenBB[SQUARE_NB][SQUARE_NB];
    pub Bitboard LineBB[SQUARE_NB][SQUARE_NB];
    pub Bitboard DistanceRingBB[SQUARE_NB][8];
    pub Bitboard ForwardBB[COLOR_NB][SQUARE_NB];
    pub Bitboard PassedPawnMask[COLOR_NB][SQUARE_NB];
    pub Bitboard PawnAttackSpan[COLOR_NB][SQUARE_NB];
    pub Bitboard PseudoAttacks[PIECE_TYPE_NB][SQUARE_NB];*/

}
