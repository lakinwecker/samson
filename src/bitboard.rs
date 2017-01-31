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

lazy_static! {
    /*pub static ref SQUARE_DISTANCE: &'static [[i32; SQUARE_NB]; SQUARE_NB] = {

    };*/
    pub static ref SQUARE_BB: [Bitboard; 65] = {
        let mut square_bb = [Bitboard(0); 65];
        for s in (SQ_A1.0)..(SQ_H8.0) {
            square_bb[s as usize] = Bitboard(1u64 << s);
        }
        square_bb
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
