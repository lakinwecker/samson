// samson - An engine focused on teaching humans.
// 
// Copyright (C) 2004-2008 Tord Romstad (Glaurung author)
// Copyright (C) 2008-2015 Marco Costalba, Joona Kiiski, Tord Romstad (Stockfish Authors)
// Copyright (C) 2015-2017 Marco Costalba, Joona Kiiski, Gary Linscott, Tord Romstad (Stockfish Authors)
// Copyright (C) 2017 Lakin Wecker 
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
use types::*;

struct StateInfo {
    // Copied when making a move
    pawn_key: Key,
    material_key: Key,
    non_pawn_material: [Value; COLOR_NB_USIZE],
    castling_rights: i32,
    rule50: i32,
    plies_from_null: i32,
    psq: Score,
    ep_square: Square,

    // Not copied when making a move (will be recomputed anyhow)
    key: Key,
    checkers_bb: Bitboard,
    captured_piece: Piece,
    previous: Option<Box<StateInfo>>,
    blockers_for_king: [Bitboard; COLOR_NB_USIZE],
    pinners_for_king: [Bitboard; COLOR_NB_USIZE],
    check_squares: [Bitboard; PIECE_TYPE_NB_USIZE]
}

// TODO: Figure out how this is used.
// typedef std::unique_ptr<std::deque<StateInfo>> StateListPtr;

struct Position {
  // Data members
  board: [Piece; SQUARE_NB_USIZE],
  by_type_bb: [Bitboard; PIECE_TYPE_NB_USIZE],
  by_color_bb: [Bitboard; COLOR_NB_USIZE],
  piece_count: [i32; PIECE_NB_USIZE],
  piece_list: [[Square; PIECE_NB_USIZE]; 16],
  index: [i32; SQUARE_NB_USIZE],
  castling_rights_mask: [i32;  SQUARE_NB_USIZE],
  castling_rook_square: [Square; CASTLING_RIGHT_NB_USIZE],
  castling_path: [Bitboard; CASTLING_RIGHT_NB_USIZE],
  nodes: u64,
  game_ply: u32,
  side_to_move: Color,
  // TODO: Thread* thisThread,
  st: Option<Box<StateInfo>>,
  chess960: bool
}
