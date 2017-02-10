// samson - An engine focused on teaching humans.
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

use std::ops::*;


// New types don't inherit traits from their contained attribute type
// which is a good thing. But in this case we want to allow our types
// to be used together in ways similar to the original type, but still
// get compile time checking when appropriate.
//
// http://stackoverflow.com/questions/24831573/automatically-implement-traits-of-enclosed-type-for-rust-newtypes-tuple-structs


// Note that this is heavily based on the awesome stockfish engine.

///-----------------------------------------------------------------------------
macro_rules! enable_bitwise_operators_on {
    ($type_: ident) => {
        impl BitAnd for $type_ {
            type Output = Self;
            fn bitand(self, rhs: Self) -> $type_ { $type_(self.0 & rhs.0) }
        }
        impl BitAndAssign for $type_ {
            fn bitand_assign(&mut self, rhs: Self) { *self = $type_(self.0 & rhs.0) }
        }
        impl BitOr for $type_ {
            type Output = Self;
            fn bitor(self, rhs: Self) -> $type_ { $type_(self.0 | rhs.0) }
        }
        impl BitOrAssign for $type_ {
            fn bitor_assign(&mut self, rhs: Self) { *self = $type_(self.0 | rhs.0) }
        }
        impl BitXor for $type_ {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> $type_ { $type_(self.0 ^ rhs.0) }
        }
        impl BitXorAssign for $type_ {
            fn bitxor_assign(&mut self, rhs: Self) { *self = $type_(self.0 ^ rhs.0) }
        }
        impl Shl<$type_> for $type_ {
            type Output = Self;
            fn shl(self, rhs: Self) -> $type_ { $type_(self.0 << rhs.0) }
        }
        impl ShlAssign<$type_> for $type_ {
            fn shl_assign(&mut self, rhs: Self) { *self = $type_(self.0 << rhs.0) }
        }
        impl Shr<$type_> for $type_ {
            type Output = Self;
            fn shr(self, rhs: Self) -> $type_ { $type_(self.0 >> rhs.0) }
        }
        impl ShrAssign<$type_> for $type_ {
            fn shr_assign(&mut self, rhs: Self) { *self = $type_(self.0 >> rhs.0) }
        }
        impl Not for $type_ {
            type Output = $type_;
            fn not(self) -> $type_ { $type_(!self.0) }
        }
    }
}

///-----------------------------------------------------------------------------
macro_rules! enable_base_operators_on {
    ($type_: ident, $integral_type_: ident) => {
        impl Add for $type_ {
            type Output = Self;
            fn add(self, rhs: Self) -> $type_ { $type_(self.0 + rhs.0) }
        }
        impl Sub for $type_ {
            type Output = Self;
            fn sub(self, rhs: Self) -> $type_ { $type_(self.0 - rhs.0) }
        }
        impl Mul<$integral_type_> for $type_ {
            type Output = Self;
            fn mul(self, rhs: $integral_type_) -> $type_ { $type_(self.0 * rhs) }
        }

        impl AddAssign for $type_ {
            fn add_assign(&mut self, rhs: Self) { *self = $type_(self.0 + rhs.0) }
        }
        impl SubAssign for $type_ {
            fn sub_assign(&mut self, rhs: Self) { *self = $type_(self.0 - rhs.0) }
        }
        impl MulAssign<$integral_type_> for $type_ {
            fn mul_assign(&mut self, rhs: $integral_type_) { *self = $type_(self.0 * rhs) }
        }
    }
}

///-----------------------------------------------------------------------------
macro_rules! enable_signed_operators_on {
    ($type_: ident) => {
        impl Neg for $type_ {
            type Output = Self;
            fn neg(self) -> $type_ { $type_(-self.0) }
        }
    }
}

///-----------------------------------------------------------------------------
macro_rules! enable_full_operators_on {
    ($type_: ident, $integral_type_: ident) => {
        enable_base_operators_on! { $type_, $integral_type_ }
        impl Div for $type_ {
            type Output = Self;
            fn div(self, rhs: Self) -> $type_ { $type_(self.0 / rhs.0) }
        }
        impl Div<$integral_type_> for $type_ {
            type Output = Self;
            fn div(self, rhs: $integral_type_) -> $type_ { $type_(self.0 / rhs) }
        }
        impl DivAssign<$integral_type_> for $type_ {
            fn div_assign(&mut self, rhs: $integral_type_) { *self = $type_(self.0 / rhs) }
        }
    }
}

///-----------------------------------------------------------------------------
macro_rules! const_vals {
    ($type_: ident: $( $name_: ident = $value_: expr ),* ) => {
        $(
            pub const $name_: $type_ = $type_($value_);
        )*
    }
}


///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Key(pub u64);
enable_bitwise_operators_on! { Key }


///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Bitboard(pub u64);
enable_bitwise_operators_on! { Bitboard }

pub const MAX_MOVES: i16 = 256;
pub const MAX_PLY: i16   = 128;

///-----------------------------------------------------------------------------
/// A move needs 16 bits to be stored
///
/// bit  0- 5: destination square (from 0 to 63)
/// bit  6-11: origin square (from 0 to 63)
/// bit 12-13: promotion piece type - 2 (from KNIGHT-2 to QUEEN-2)
/// bit 14-15: special move flag: promotion (1), en passant (2), castling (3)
/// NOTE: EN-PASSANT bit is set only when a pawn can be captured
///
/// Special cases are MOVE_NONE and MOVE_NULL. We can sneak these in because in
/// any normal move destination square is always different from origin square
/// while MOVE_NONE and MOVE_NULL have the same origin and destination square.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Move(pub u16);
enable_bitwise_operators_on! { Move }
const_vals! { Move:
        MOVE_NONE = 0,
        MOVE_NULL = 65
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct MoveType(pub u16);
enable_bitwise_operators_on! { MoveType }
const_vals! { MoveType:
    NORMAL = 0,
    PROMOTION = 1 << 14,
    ENPASSANT = 2 << 14,
    CASTLING = 3 << 14
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Color(pub i8);
enable_bitwise_operators_on! { Color }
enable_full_operators_on! { Color, i8 }
const_vals! { Color:
    WHITE = 0,
    BLACK = 1,
    NO_COLOR = 2,
    COLOR_NB = 2
}
impl Neg for Color {
    type Output = Color;
    fn neg(self) -> Color { Color(self.0 ^ BLACK.0) }
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct CastlingSide(pub u16);
enable_bitwise_operators_on! { CastlingSide }
const_vals! { CastlingSide:
    KING_SIDE = 0,
    QUEEN_SIDE = 1,
    CASTLING_SIDE_NB = 2
}
impl BitOr<Color> for CastlingSide {
    type Output = CastlingRight;
    fn bitor(self, c: Color) -> CastlingRight {
        match self == QUEEN_SIDE {
            true => CastlingRight(WHITE_OO.0 << (1 + 2 * c.0)),
            false => CastlingRight(WHITE_OO.0 << (0 + 2 * c.0))
        }
    }
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct CastlingRight(pub u16);
enable_bitwise_operators_on! { CastlingRight }
const_vals! { CastlingRight:
    NO_CASTLING = 0,
    WHITE_OO = 1,
    WHITE_OOO = 1 << 1,
    BLACK_OO = 1 << 2,
    BLACK_OOO = 1 << 3,
    ANY_CASTLING = WHITE_OO.0 | WHITE_OOO.0 | BLACK_OO.0 | BLACK_OOO.0,
    CASTLING_RIGHT_NB = 16
}


// TODO: MakeCastling
/*
template<Color C, CastlingSide S> struct MakeCastling {
  static const CastlingRight
  right = C == WHITE ? S == QUEEN_SIDE ? WHITE_OOO : WHITE_OO
                     : S == QUEEN_SIDE ? BLACK_OOO : BLACK_OO;
};
*/

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Phase(pub u16);
enable_bitwise_operators_on! { Phase }
const_vals! { Phase:
    PHASE_ENDGAME = 3,
    PHASE_MIDGAME = 128,
    MG = 0,
    EG = 1,
    PHASE_NB = 2
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ScaleFactor(pub u8);
enable_bitwise_operators_on! { ScaleFactor }
const_vals! { Phase:
    SCALE_FACTOR_DRAW = 0,
    SCALE_FACTOR_ONEPAWN = 48,
    SCALE_FACTOR_NORMAL = 64,
    SCALE_FACTOR_MAX = 128,
    SCALE_FACTOR_NONE = 255
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Bound(pub u8);
enable_bitwise_operators_on! { Bound }
const_vals! { Bound:
    BOUND_NONE = 0,
    BOUND_UPPER = 1,
    BOUND_LOWER = 2,
    BOUND_EXACT = BOUND_UPPER.0 | BOUND_LOWER.0
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Value(pub i32);
enable_bitwise_operators_on! { Value }
enable_full_operators_on! { Value, i32 }
enable_signed_operators_on! { Value }
const_vals! { Value:
    VALUE_ZERO      = 0,
    VALUE_DRAW      = 0,
    VALUE_KNOWN_WIN = 10000,
    VALUE_MATE      = 32000,
    VALUE_INFINITE  = 32001,
    VALUE_NONE      = 32002,

    VALUE_MATE_IN_MAX_PLY  =  VALUE_MATE.0 - 2 * MAX_PLY as i32,
    VALUE_MATED_IN_MAX_PLY = -VALUE_MATE.0 + 2 * MAX_PLY as i32,

    PAWN_VALUE_MG   = 188,   PAWN_VALUE_EG   = 248,
    KNIGHT_VALUE_MG = 753,   KNIGHT_VALUE_EG = 832,
    BISHOP_VALUE_MG = 826,   BISHOP_VALUE_EG = 897,
    ROOK_VALUE_MG   = 1285,  ROOK_VALUE_EG   = 1371,
    QUEEN_VALUE_MG  = 2513,  QUEEN_VALUE_EG  = 2650,

    MID_GAME_LIMIT  = 15258, END_GAME_LIMIT  = 3915
}

impl Add<i32> for Value {
    type Output = Value;
    fn add(self, rhs: i32) -> Value { Value(self.0 + rhs) }
}
impl AddAssign<i32> for Value {
    fn add_assign(&mut self, rhs: i32) { *self = Value(self.0 + rhs) }
}
impl Sub<i32> for Value {
    type Output = Value;
    fn sub(self, rhs: i32) -> Value { Value(self.0 - rhs) }
}
impl SubAssign<i32> for Value {
    fn sub_assign(&mut self, rhs: i32) { *self = Value(self.0 - rhs) }
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct PieceType(pub i8);
enable_bitwise_operators_on! { PieceType }
enable_full_operators_on! { PieceType, i8 }
enable_signed_operators_on! { PieceType }
const_vals! { PieceType: 
    NO_PIECE_TYPE = 0, PAWN = 1, KNIGHT = 2, BISHOP = 3, ROOK = 4, QUEEN = 5, KING = 6,
    ALL_PIECES = 0,
    PIECE_TYPE_NB = 8
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Piece(pub i8);
enable_bitwise_operators_on! { Piece }
enable_full_operators_on! { Piece, i8 }
const_vals! { Piece:
    NO_PIECE = 0,
    W_PAWN = 1, W_KNIGHT = 2, W_BISHOP = 3, W_ROOK = 4, W_QUEEN = 5, W_KING = 6,
    B_PAWN = 9, B_KNIGHT = 10, B_BISHOP = 11, B_ROOK = 12, B_QUEEN = 13, B_KING = 14,
    PIECE_NB = 16
}

impl Neg for Piece {
    type Output = Piece;
    fn neg(self) -> Piece { Piece(self.0 ^ 8) }
}

pub static PIECES: &'static [Piece] = &[ W_PAWN, W_KNIGHT, W_BISHOP, W_ROOK, W_QUEEN, W_KING,
                         B_PAWN, B_KNIGHT, B_BISHOP, B_ROOK, B_QUEEN, B_KING ];
// TODO: 
// extern Value PieceValue[PHASE_NB][PIECE_NB];

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Depth(pub i16);
enable_bitwise_operators_on!{ Depth }
enable_full_operators_on! { Depth, i16 }
enable_signed_operators_on! { Depth }
const_vals! { Depth:
    ONE_PLY = 1,

    DEPTH_ZERO          =  0 * ONE_PLY.0,
    DEPTH_QS_CHECKS     =  0 * ONE_PLY.0,
    DEPTH_QS_NO_CHECKS  = -1 * ONE_PLY.0,
    DEPTH_QS_RECAPTURES = -5 * ONE_PLY.0,

    DEPTH_NONE = -6 * ONE_PLY.0,
    DEPTH_MAX  = MAX_PLY * ONE_PLY.0
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Square(pub i8);
enable_bitwise_operators_on!{ Square }
enable_full_operators_on! { Square, i8 }
const_vals! { Square:
    SQ_A1=0,  SQ_B1=1,  SQ_C1=2,  SQ_D1=3,  SQ_E1=4,  SQ_F1=5,  SQ_G1=6,  SQ_H1=7,
    SQ_A2=8,  SQ_B2=9,  SQ_C2=10, SQ_D2=11, SQ_E2=12, SQ_F2=13, SQ_G2=14, SQ_H2=15,
    SQ_A3=16, SQ_B3=17, SQ_C3=18, SQ_D3=19, SQ_E3=20, SQ_F3=21, SQ_G3=22, SQ_H3=23,
    SQ_A4=24, SQ_B4=25, SQ_C4=26, SQ_D4=27, SQ_E4=28, SQ_F4=29, SQ_G4=30, SQ_H4=31,
    SQ_A5=32, SQ_B5=33, SQ_C5=34, SQ_D5=35, SQ_E5=36, SQ_F5=37, SQ_G5=38, SQ_H5=39,
    SQ_A6=40, SQ_B6=41, SQ_C6=42, SQ_D6=43, SQ_E6=44, SQ_F6=45, SQ_G6=46, SQ_H6=47,
    SQ_A7=48, SQ_B7=49, SQ_C7=50, SQ_D7=51, SQ_E7=52, SQ_F7=53, SQ_G7=54, SQ_H7=55,
    SQ_A8=56, SQ_B8=57, SQ_C8=58, SQ_D8=59, SQ_E8=60, SQ_F8=61, SQ_G8=62, SQ_H8=63,
    SQ_NONE=64,

    SQUARE_NB = 64,

    NORTH =  8,
    EAST  =  1,
    SOUTH = -8,
    WEST  = -1,

    NORTH_EAST = NORTH.0 + EAST.0,
    SOUTH_EAST = SOUTH.0 + EAST.0,
    SOUTH_WEST = SOUTH.0 + WEST.0,
    NORTH_WEST = NORTH.0 + WEST.0
}

impl Neg for Square {
    type Output = Square;
    fn neg(self) -> Square { Square(self.0 ^ SQ_A8.0) }
}

/*pub static SQUARES: &'static [File] = &[
    SQ_A1, SQ_A2, SQ_A3, SQ_A4, SQ_A5, SQ_A6, SQ_A7, SQ_A8,
    SQ_B1, SQ_B2, SQ_B3, SQ_B4, SQ_B5, SQ_B6, SQ_B7, SQ_B8,
    SQ_C1, SQ_C2, SQ_C3, SQ_C4, SQ_C5, SQ_C6, SQ_C7, SQ_C8,
    SQ_D1, SQ_D2, SQ_D3, SQ_D4, SQ_D5, SQ_D6, SQ_D7, SQ_D8,
    SQ_E1, SQ_E2, SQ_E3, SQ_E4, SQ_E5, SQ_E6, SQ_E7, SQ_E8,
    SQ_F1, SQ_F2, SQ_F3, SQ_F4, SQ_F5, SQ_F6, SQ_F7, SQ_F8,
    SQ_G1, SQ_G2, SQ_G3, SQ_G4, SQ_G5, SQ_G6, SQ_G7, SQ_G8,
    SQ_A1, SQ_A2, SQ_A3, SQ_A4, SQ_A5, SQ_A6, SQ_A7, SQ_A8,
];*/

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct File(pub i8);
enable_bitwise_operators_on!{ File }
enable_full_operators_on! { File, i8 }
enable_signed_operators_on! { File }
const_vals! { File:
    FILE_A=0, FILE_B=1, FILE_C=2, FILE_D=3, FILE_E=4, FILE_F=5, FILE_G=6, FILE_H=7, FILE_NB=8
}
pub static FILES: &'static [File] = &[ FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H ];

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Rank(pub i8);
enable_bitwise_operators_on!{ Rank }
enable_full_operators_on! { Rank, i8 }
enable_signed_operators_on! { Rank }
const_vals! { Rank:
    RANK_1=0, RANK_2=1, RANK_3=2, RANK_4=3, RANK_5=4, RANK_6=5, RANK_7=6, RANK_8=7, RANK_NB=8
}
pub static RANKS: &'static [Rank] = &[ RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8 ];

///-----------------------------------------------------------------------------
/// Score enum stores a middlegame and an endgame value in a single integer
/// (enum). The least significant 16 bits are used to store the endgame value
/// and the upper 16 bits are used to store the middlegame value. Take some
/// care to avoid left-shifting a signed int to avoid undefined behavior.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Score(pub u32);
enable_bitwise_operators_on!{ Score }
enable_base_operators_on! { Score, u32 }
const_vals! { Score:
    SCORE_ZERO=0
}

pub fn make_score(mg: u32, eg: u32) -> Score {
    return Score((eg << 16) | mg);
}

pub fn eg_value(s: Score) -> Score {
    return Score(s.0 >> 16);
}

pub fn mg_value(s: Score) -> Score {
    return Score((s.0 << 16) >> 16);
}

/// Note that multiplication for Score is intentionally left out.
/// There is a very high risk of overflow.

/// Division of a Score must be handled separately for each term
impl Div<u32> for Score {
    type Output = Score;
    fn div(self, rhs: u32) -> Score {
        make_score(mg_value(self).0 / rhs, eg_value(self).0 / rhs)
    }
}

///-----------------------------------------------------------------------------
pub fn mate_in(ply: i32) -> Value {
    return VALUE_MATE - ply;
}

pub fn mated_in(ply: i32) -> Value {
    return -VALUE_MATE + ply;
}

pub fn make_square(f: File, r: Rank) -> Square {
    return Square((r.0 << 3) + f.0);
}

pub fn make_piece(c: Color, pt: PieceType) -> Piece {
    return Piece((c.0 << 3) + pt.0);
}

pub fn type_of_piece(pc: Piece) -> PieceType {
    return PieceType(pc.0 & 7);
}

pub fn color_of(pc: Piece) -> Color {
    assert!(pc != NO_PIECE);
    return Color(pc.0 >> 3);
}

pub fn is_square_ok(s: Square) -> bool {
    return s >= SQ_A1 && s <= SQ_H8;
}

pub fn file_of(s: Square) -> File {
    return File(s.0 & 7);
}

pub fn rank_of(s: Square) -> Rank {
    return Rank(s.0 >> 3);
}

/// The relative functions give you the equivalent square / rank to the one
/// passed in if you were sitting on the color's side of the board. Sometimes
/// this is the same square / rank you passed in
pub fn relative_square(c: Color, s: Square) -> Square {
    return Square(s.0 ^ (c.0 * 56));
}

pub fn relative_rank(c: Color, r: Rank) -> Rank {
    return Rank(r.0 ^ (c.0 * 7));
}

pub fn relative_rank_from_square(c: Color, s: Square) -> Rank {
    return relative_rank(c, rank_of(s));
}


pub fn opposite_colors(s1: Square, s2: Square) -> bool {
    let s: i8 = s1.0 ^ s2.0;
    (((s >> 3) ^ s) & 1) > 0
}

pub fn pawn_push(c: Color) -> Square {
    match c {
        WHITE => NORTH,
        _ => SOUTH
    }
}


pub fn from_square(m: Move) -> Square {
    return Square(((m.0 >> 6) & 0x3F) as i8);
}

pub fn to_square(m: Move) -> Square {
    return Square((m.0 & 0x3F) as i8);
}

pub fn make_move_simple(from: Square, to: Square) -> Move {
    return Move((((from.0 as u16) << 6) | (to.0 as u16)));
}

pub fn type_of_move(m: Move) -> MoveType {
    return MoveType(m.0 & (3 << 14));
}

pub fn promotion_type(m: Move) -> PieceType {
    let knight = KNIGHT.0 as i8;
    return PieceType((((m.0 >> 12) as i8) & 3) + knight);
}


// TODO: These could probably be optimized and compile time checked
pub fn make_move_with_promotion(from: Square, to: Square, pt: PieceType) -> Move {
    let from = from.0 as u16;
    let to = to.0 as u16;
    let pt = pt.0 as u16;
    let promotion = PROMOTION.0 as u16;
    let knight = KNIGHT.0 as u16;
    return Move(promotion | ((pt - knight) << 12) | (from << 6) | to);
}

pub fn make_move_(from: Square, to: Square) -> Move {
    let from = from.0 as u16;
    let to = to.0 as u16;
    let enpassant = ENPASSANT.0 as u16;
    return Move(enpassant | ((from << 6) | to));
}

pub fn make_castling_move(from: Square, to: Square) -> Move {
    let from = from.0 as u16;
    let to = to.0 as u16;
    let castling = CASTLING.0 as u16;
    return Move(castling | ((from << 6) | to));
}
pub fn is_move_ok(m: Move) -> bool {
    return from_square(m) != to_square(m); // Catch MOVE_NULL and MOVE_NONE
}

///-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

        #[test]
    fn test_one_ply_is_power_of_2() {
        assert!((!(ONE_PLY & (ONE_PLY - Depth(1)))).0 != 0, "ONE_PLY is not a power of 2");
    }

    #[test]
    fn test_make_score() {
        assert_eq!(
            make_score(
                0b0000_0000_0000_0000_1010_1010_1010_1010,
                0b0000_0000_0000_0000_0000_0000_0000_0000
            ),
            Score(0b0000_0000_0000_0000_1010_1010_1010_1010)
        );
        assert_eq!(
            make_score(
                0b0000_0000_0000_0000_0000_0000_0000_0000,
                0b0000_0000_0000_0000_1010_1010_1010_1010
            ),
            Score(0b1010_1010_1010_1010_0000_0000_0000_0000)
        );
        assert_eq!(
            make_score(
                0b0000_0000_0000_0000_0101_0101_0101_0101,
                0b0000_0000_0000_0000_1010_1010_1010_1010
            ),
            Score(0b1010_1010_1010_1010_0101_0101_0101_0101)
        );
    }

    #[test]
    fn test_eg_value() {
        assert_eq!(
            eg_value(Score(0b0000_0000_0000_0000_1010_1010_1010_1010)),
            Score(0b0000_0000_0000_0000_0000_0000_0000_0000)
        );
        assert_eq!(
            eg_value(Score(0b1010_1010_1010_1010_0000_0000_0000_0000)),
            Score(0b0000_0000_0000_0000_1010_1010_1010_1010)
        );
    }

    #[test]
    fn test_mg_value() {
        assert_eq!(
            mg_value(Score(0b0000_0000_0000_0000_1010_1010_1010_1010)),
            Score(0b0000_0000_0000_0000_1010_1010_1010_1010)
        );
        assert_eq!(
            mg_value(Score(0b1010_1010_1010_1010_0000_0000_0000_0000)),
            Score(0b0000_0000_0000_0000_0000_0000_0000_0000)
        );
    }
    #[test]
    fn test_make_square() {
        assert_eq!(SQ_A1, make_square(FILE_A, RANK_1));
        assert_eq!(SQ_A2, make_square(FILE_A, RANK_2));
        assert_eq!(SQ_A3, make_square(FILE_A, RANK_3));
        assert_eq!(SQ_A4, make_square(FILE_A, RANK_4));
        assert_eq!(SQ_A5, make_square(FILE_A, RANK_5));
        assert_eq!(SQ_A6, make_square(FILE_A, RANK_6));
        assert_eq!(SQ_A7, make_square(FILE_A, RANK_7));
        assert_eq!(SQ_A8, make_square(FILE_A, RANK_8));

        assert_eq!(SQ_B1, make_square(FILE_B, RANK_1));
        assert_eq!(SQ_B2, make_square(FILE_B, RANK_2));
        assert_eq!(SQ_B3, make_square(FILE_B, RANK_3));
        assert_eq!(SQ_B4, make_square(FILE_B, RANK_4));
        assert_eq!(SQ_B5, make_square(FILE_B, RANK_5));
        assert_eq!(SQ_B6, make_square(FILE_B, RANK_6));
        assert_eq!(SQ_B7, make_square(FILE_B, RANK_7));
        assert_eq!(SQ_B8, make_square(FILE_B, RANK_8));

        assert_eq!(SQ_C1, make_square(FILE_C, RANK_1));
        assert_eq!(SQ_C2, make_square(FILE_C, RANK_2));
        assert_eq!(SQ_C3, make_square(FILE_C, RANK_3));
        assert_eq!(SQ_C4, make_square(FILE_C, RANK_4));
        assert_eq!(SQ_C5, make_square(FILE_C, RANK_5));
        assert_eq!(SQ_C6, make_square(FILE_C, RANK_6));
        assert_eq!(SQ_C7, make_square(FILE_C, RANK_7));
        assert_eq!(SQ_C8, make_square(FILE_C, RANK_8));

        assert_eq!(SQ_D1, make_square(FILE_D, RANK_1));
        assert_eq!(SQ_D2, make_square(FILE_D, RANK_2));
        assert_eq!(SQ_D3, make_square(FILE_D, RANK_3));
        assert_eq!(SQ_D4, make_square(FILE_D, RANK_4));
        assert_eq!(SQ_D5, make_square(FILE_D, RANK_5));
        assert_eq!(SQ_D6, make_square(FILE_D, RANK_6));
        assert_eq!(SQ_D7, make_square(FILE_D, RANK_7));
        assert_eq!(SQ_D8, make_square(FILE_D, RANK_8));

        assert_eq!(SQ_E1, make_square(FILE_E, RANK_1));
        assert_eq!(SQ_E2, make_square(FILE_E, RANK_2));
        assert_eq!(SQ_E3, make_square(FILE_E, RANK_3));
        assert_eq!(SQ_E4, make_square(FILE_E, RANK_4));
        assert_eq!(SQ_E5, make_square(FILE_E, RANK_5));
        assert_eq!(SQ_E6, make_square(FILE_E, RANK_6));
        assert_eq!(SQ_E7, make_square(FILE_E, RANK_7));
        assert_eq!(SQ_E8, make_square(FILE_E, RANK_8));

        assert_eq!(SQ_F1, make_square(FILE_F, RANK_1));
        assert_eq!(SQ_F2, make_square(FILE_F, RANK_2));
        assert_eq!(SQ_F3, make_square(FILE_F, RANK_3));
        assert_eq!(SQ_F4, make_square(FILE_F, RANK_4));
        assert_eq!(SQ_F5, make_square(FILE_F, RANK_5));
        assert_eq!(SQ_F6, make_square(FILE_F, RANK_6));
        assert_eq!(SQ_F7, make_square(FILE_F, RANK_7));
        assert_eq!(SQ_F8, make_square(FILE_F, RANK_8));

        assert_eq!(SQ_G1, make_square(FILE_G, RANK_1));
        assert_eq!(SQ_G2, make_square(FILE_G, RANK_2));
        assert_eq!(SQ_G3, make_square(FILE_G, RANK_3));
        assert_eq!(SQ_G4, make_square(FILE_G, RANK_4));
        assert_eq!(SQ_G5, make_square(FILE_G, RANK_5));
        assert_eq!(SQ_G6, make_square(FILE_G, RANK_6));
        assert_eq!(SQ_G7, make_square(FILE_G, RANK_7));
        assert_eq!(SQ_G8, make_square(FILE_G, RANK_8));

        assert_eq!(SQ_H1, make_square(FILE_H, RANK_1));
        assert_eq!(SQ_H2, make_square(FILE_H, RANK_2));
        assert_eq!(SQ_H3, make_square(FILE_H, RANK_3));
        assert_eq!(SQ_H4, make_square(FILE_H, RANK_4));
        assert_eq!(SQ_H5, make_square(FILE_H, RANK_5));
        assert_eq!(SQ_H6, make_square(FILE_H, RANK_6));
        assert_eq!(SQ_H7, make_square(FILE_H, RANK_7));
        assert_eq!(SQ_H8, make_square(FILE_H, RANK_8));
    }
    #[test]
    fn test_make_piece() {
        assert_eq!(W_PAWN, make_piece(WHITE, PAWN));
        assert_eq!(W_KNIGHT, make_piece(WHITE, KNIGHT));
        assert_eq!(W_BISHOP, make_piece(WHITE, BISHOP));
        assert_eq!(W_ROOK, make_piece(WHITE, ROOK));
        assert_eq!(W_QUEEN, make_piece(WHITE, QUEEN));
        assert_eq!(W_KING, make_piece(WHITE, KING));
        assert_eq!(B_PAWN, make_piece(BLACK, PAWN));
        assert_eq!(B_KNIGHT, make_piece(BLACK, KNIGHT));
        assert_eq!(B_BISHOP, make_piece(BLACK, BISHOP));
        assert_eq!(B_ROOK, make_piece(BLACK, ROOK));
        assert_eq!(B_QUEEN, make_piece(BLACK, QUEEN));
        assert_eq!(B_KING, make_piece(BLACK, KING));
    }

    #[test]
    fn test_type_of_piece() {
        assert_eq!(PAWN, type_of_piece(W_PAWN));
        assert_eq!(PAWN, type_of_piece(B_PAWN));

        assert_eq!(KNIGHT, type_of_piece(W_KNIGHT));
        assert_eq!(KNIGHT, type_of_piece(B_KNIGHT));

        assert_eq!(BISHOP, type_of_piece(W_BISHOP));
        assert_eq!(BISHOP, type_of_piece(B_BISHOP));

        assert_eq!(ROOK, type_of_piece(W_ROOK));
        assert_eq!(ROOK, type_of_piece(B_ROOK));

        assert_eq!(QUEEN, type_of_piece(W_QUEEN));
        assert_eq!(QUEEN, type_of_piece(B_QUEEN));

        assert_eq!(KING, type_of_piece(W_KING));
        assert_eq!(KING, type_of_piece(B_KING));
    }

    #[test]
    fn test_color_of() {
        assert_eq!(WHITE, color_of(W_PAWN));
        assert_eq!(BLACK, color_of(B_PAWN));

        assert_eq!(WHITE, color_of(W_KNIGHT));
        assert_eq!(BLACK, color_of(B_KNIGHT));

        assert_eq!(WHITE, color_of(W_BISHOP));
        assert_eq!(BLACK, color_of(B_BISHOP));

        assert_eq!(WHITE, color_of(W_ROOK));
        assert_eq!(BLACK, color_of(B_ROOK));

        assert_eq!(WHITE, color_of(W_QUEEN));
        assert_eq!(BLACK, color_of(B_QUEEN));

        assert_eq!(WHITE, color_of(W_KING));
        assert_eq!(BLACK, color_of(B_KING));
    }

    #[test]
    fn test_is_square_ok() {
        assert_eq!(false, is_square_ok(Square(SQ_A1.0-1)));
        assert_eq!(true, is_square_ok(SQ_A1));
        assert_eq!(true, is_square_ok(SQ_A2));
        assert_eq!(true, is_square_ok(SQ_A3));
        assert_eq!(true, is_square_ok(SQ_A4));
        assert_eq!(true, is_square_ok(SQ_A5));
        assert_eq!(true, is_square_ok(SQ_A6));
        assert_eq!(true, is_square_ok(SQ_A7));
        assert_eq!(true, is_square_ok(SQ_A8));

        assert_eq!(true, is_square_ok(SQ_H1));
        assert_eq!(true, is_square_ok(SQ_H2));
        assert_eq!(true, is_square_ok(SQ_H3));
        assert_eq!(true, is_square_ok(SQ_H4));
        assert_eq!(true, is_square_ok(SQ_H5));
        assert_eq!(true, is_square_ok(SQ_H6));
        assert_eq!(true, is_square_ok(SQ_H7));
        assert_eq!(true, is_square_ok(SQ_H8));
        assert_eq!(false, is_square_ok(Square(SQ_H8.0+1)));
    }

    #[test]
    fn test_file_of() {
        for &f in FILES {
            for &r in RANKS {
                assert_eq!(f, file_of(make_square(f, r)));
            }
        }
    }

    #[test]
    fn test_rank_of() {
        for &f in FILES {
            for &r in RANKS {
                assert_eq!(r, rank_of(make_square(f, r)));
            }
        }
    }
    #[test]
    fn test_relative_square() {
        for &f in FILES {
            assert_eq!(make_square(f, RANK_1), relative_square(WHITE, make_square(f, RANK_1)));
            assert_eq!(make_square(f, RANK_8), relative_square(BLACK, make_square(f, RANK_1)));
            assert_eq!(make_square(f, RANK_2), relative_square(WHITE, make_square(f, RANK_2)));
            assert_eq!(make_square(f, RANK_7), relative_square(BLACK, make_square(f, RANK_2)));
            assert_eq!(make_square(f, RANK_3), relative_square(WHITE, make_square(f, RANK_3)));
            assert_eq!(make_square(f, RANK_6), relative_square(BLACK, make_square(f, RANK_3)));
            assert_eq!(make_square(f, RANK_4), relative_square(WHITE, make_square(f, RANK_4)));
            assert_eq!(make_square(f, RANK_5), relative_square(BLACK, make_square(f, RANK_4)));
            assert_eq!(make_square(f, RANK_5), relative_square(WHITE, make_square(f, RANK_5)));
            assert_eq!(make_square(f, RANK_4), relative_square(BLACK, make_square(f, RANK_5)));
            assert_eq!(make_square(f, RANK_6), relative_square(WHITE, make_square(f, RANK_6)));
            assert_eq!(make_square(f, RANK_3), relative_square(BLACK, make_square(f, RANK_6)));
            assert_eq!(make_square(f, RANK_7), relative_square(WHITE, make_square(f, RANK_7)));
            assert_eq!(make_square(f, RANK_2), relative_square(BLACK, make_square(f, RANK_7)));
            assert_eq!(make_square(f, RANK_8), relative_square(WHITE, make_square(f, RANK_8)));
            assert_eq!(make_square(f, RANK_1), relative_square(BLACK, make_square(f, RANK_8)));
        }
    }
    #[test]
    fn test_relative_rank() {
        assert_eq!(RANK_1, relative_rank(WHITE, RANK_1));
        assert_eq!(RANK_8, relative_rank(BLACK, RANK_1));
        assert_eq!(RANK_2, relative_rank(WHITE, RANK_2));
        assert_eq!(RANK_7, relative_rank(BLACK, RANK_2));
        assert_eq!(RANK_3, relative_rank(WHITE, RANK_3));
        assert_eq!(RANK_6, relative_rank(BLACK, RANK_3));
        assert_eq!(RANK_4, relative_rank(WHITE, RANK_4));
        assert_eq!(RANK_5, relative_rank(BLACK, RANK_4));
        assert_eq!(RANK_5, relative_rank(WHITE, RANK_5));
        assert_eq!(RANK_4, relative_rank(BLACK, RANK_5));
        assert_eq!(RANK_6, relative_rank(WHITE, RANK_6));
        assert_eq!(RANK_3, relative_rank(BLACK, RANK_6));
        assert_eq!(RANK_7, relative_rank(WHITE, RANK_7));
        assert_eq!(RANK_2, relative_rank(BLACK, RANK_7));
        assert_eq!(RANK_8, relative_rank(WHITE, RANK_8));
        assert_eq!(RANK_1, relative_rank(BLACK, RANK_8));
    }
    #[test]
    fn test_relative_rank_from_square() {
        for &f in FILES {
            assert_eq!(RANK_1, relative_rank_from_square(WHITE, make_square(f, RANK_1)));
            assert_eq!(RANK_8, relative_rank_from_square(BLACK, make_square(f, RANK_1)));
            assert_eq!(RANK_2, relative_rank_from_square(WHITE, make_square(f, RANK_2)));
            assert_eq!(RANK_7, relative_rank_from_square(BLACK, make_square(f, RANK_2)));
            assert_eq!(RANK_3, relative_rank_from_square(WHITE, make_square(f, RANK_3)));
            assert_eq!(RANK_6, relative_rank_from_square(BLACK, make_square(f, RANK_3)));
            assert_eq!(RANK_4, relative_rank_from_square(WHITE, make_square(f, RANK_4)));
            assert_eq!(RANK_5, relative_rank_from_square(BLACK, make_square(f, RANK_4)));
            assert_eq!(RANK_5, relative_rank_from_square(WHITE, make_square(f, RANK_5)));
            assert_eq!(RANK_4, relative_rank_from_square(BLACK, make_square(f, RANK_5)));
            assert_eq!(RANK_6, relative_rank_from_square(WHITE, make_square(f, RANK_6)));
            assert_eq!(RANK_3, relative_rank_from_square(BLACK, make_square(f, RANK_6)));
            assert_eq!(RANK_7, relative_rank_from_square(WHITE, make_square(f, RANK_7)));
            assert_eq!(RANK_2, relative_rank_from_square(BLACK, make_square(f, RANK_7)));
            assert_eq!(RANK_8, relative_rank_from_square(WHITE, make_square(f, RANK_8)));
            assert_eq!(RANK_1, relative_rank_from_square(BLACK, make_square(f, RANK_8)));
        }
    }
    #[test]
    fn test_opposite_colors() {
        for &f in FILES {
            assert_eq!(true, opposite_colors(make_square(f, RANK_1), make_square(f, RANK_8)));
            assert_eq!(true, opposite_colors(make_square(f, RANK_2), make_square(f, RANK_7)));
            assert_eq!(true, opposite_colors(make_square(f, RANK_3), make_square(f, RANK_6)));
            assert_eq!(true, opposite_colors(make_square(f, RANK_4), make_square(f, RANK_5)));

            assert_eq!(false, opposite_colors(make_square(f, RANK_1), make_square(f, RANK_5)));
            assert_eq!(false, opposite_colors(make_square(f, RANK_2), make_square(f, RANK_6)));
            assert_eq!(false, opposite_colors(make_square(f, RANK_3), make_square(f, RANK_7)));
            assert_eq!(false, opposite_colors(make_square(f, RANK_4), make_square(f, RANK_8)));
        }
    }
    #[test]
    fn test_pawn_push() {
        assert_eq!(NORTH, pawn_push(WHITE));
        assert_eq!(SOUTH, pawn_push(BLACK));
    }
    #[test]
    fn test_from_square() {
        assert_eq!(SQ_A1, from_square(make_move_simple(SQ_A1, SQ_A2)));
        assert_eq!(SQ_B3, from_square(make_move_simple(SQ_B3, SQ_B4)));
        assert_eq!(SQ_A3, from_square(make_move_simple(SQ_A3, SQ_A4)));
        assert_eq!(SQ_A2, from_square(make_move_simple(SQ_A2, SQ_A3)));
    }
    #[test]
    fn test_to_square() {
        assert_eq!(SQ_A2, to_square(make_move_simple(SQ_A1, SQ_A2)));
        assert_eq!(SQ_A3, to_square(make_move_simple(SQ_A2, SQ_A3)));
    }
    #[test]
    fn test_is_move_ok() {
        assert_eq!(true, is_move_ok(make_move_simple(SQ_A1, SQ_A2)));
        assert_eq!(true, is_move_ok(make_move_simple(SQ_A2, SQ_A3)));
        assert_eq!(false, is_move_ok(MOVE_NULL));
        assert_eq!(false, is_move_ok(MOVE_NONE));
    }
}
