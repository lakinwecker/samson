// This file is part of the samson library.
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
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//------------------------------------------------------------------------------
// Parsers for the SAN specification
//------------------------------------------------------------------------------

use super::super::types::*;

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum MoveOrCapture { Move, Capture }

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum SAN {
    PieceToSquare(PieceType, MoveOrCapture, Square),
    PieceRankToSquare(PieceType, Rank, MoveOrCapture, Square),
    PieceFileToSquare(PieceType, File, MoveOrCapture, Square),
    PieceSquareToSquare(PieceType, Square, MoveOrCapture, Square),
    PromotionToSquare(MoveOrCapture, Square, PieceType),
    Error(char)
}

///-----------------------------------------------------------------------------
named!(pub san_piece<PieceType>, 
    map!(
	one_of!("PNBRQKpnbrqk"),
	|c: char| {
	    match c {
		'p'| 'P' => PAWN,
		'n'| 'N' => KNIGHT,
		'b'| 'B' => BISHOP,
		'r'| 'R' => ROOK,
		'q'| 'Q' => QUEEN,
		'k'| 'K' => KING,
		_ => PIECE_TYPE_NB // This should never happen because of above.
	    }
	}
    )
);
///-----------------------------------------------------------------------------
named!(pub san_file<File>, 
    map!(
	one_of!("abcdefghABCDEFGH"),
	|c: char| {
	    match c {
		'a'| 'A' => FILE_A,
		'b'| 'B' => FILE_B,
		'c'| 'C' => FILE_C,
		'd'| 'D' => FILE_D,
		'e'| 'E' => FILE_E,
		'f'| 'F' => FILE_F,
		'g'| 'G' => FILE_G,
		'h'| 'H' => FILE_H,
		_ => FILE_NB // This should never happen because of above.
	    }
	}
    )
);

///-----------------------------------------------------------------------------
named!(pub san_capture<MoveOrCapture>,
    map!(
	one_of!("x"),
	|c: char| {
	    match c {
		'x' => MoveOrCapture::Capture,
		_ => MoveOrCapture::Move
	    }
	}
    )
);

///-----------------------------------------------------------------------------
named!(pub san_promotion, one_of!("="));

///-----------------------------------------------------------------------------
named!(pub san_rank<Rank>, 
    map!(
	one_of!("12345678"),
	|c: char| {
	    match c {
		'1' => RANK_1,
		'2' => RANK_2,
		'3' => RANK_3,
		'4' => RANK_4,
		'5' => RANK_5,
		'6' => RANK_6,
		'7' => RANK_7,
		'8' => RANK_8,
		_ => RANK_NB // This should never happen because of above.
	    }
	}
    )
);

///-----------------------------------------------------------------------------
named!(pub san_square<Square>, 
    do_parse!(
	file: san_file >>
	rank: san_rank >>
	(make_square(file, rank))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_pawn_to_square<SAN>, 
    do_parse!(
	square: san_square >>
	(SAN::PieceToSquare(PAWN, MoveOrCapture::Move, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_pawn_captures_square<SAN>, 
    do_parse!(
	file: san_file >>
	capture: san_capture >>
	square: san_square >>
	(SAN::PieceFileToSquare(PAWN, file, capture, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_to_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	square: san_square >>
	(SAN::PieceToSquare(piece, MoveOrCapture::Move, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_captures_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	capture: san_capture >>
	square: san_square >>
	(SAN::PieceToSquare(piece, capture, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_file_to_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	file: san_file >>
	square: san_square >>
	(SAN::PieceFileToSquare(piece, file, MoveOrCapture::Move, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_file_captures_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	file: san_file >>
	capture: san_capture >>
	square: san_square >>
	(SAN::PieceFileToSquare(piece, file, capture, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_rank_to_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	rank: san_rank >>
	square: san_square >>
	(SAN::PieceRankToSquare(piece, rank, MoveOrCapture::Move, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_rank_captures_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	rank: san_rank >>
	capture: san_capture >>
	square: san_square >>
	(SAN::PieceRankToSquare(piece, rank, capture, square))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_square_to_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	square1: san_square >>
	square2: san_square >>
	(SAN::PieceSquareToSquare(piece, square1, MoveOrCapture::Move, square2))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_square_captures_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	square1: san_square >>
	capture: san_capture >>
	square2: san_square >>
	(SAN::PieceSquareToSquare(piece, square1, capture, square2))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_promotion_to_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	square1: san_square >>
	square2: san_square >>
	(SAN::PieceSquareToSquare(piece, square1, MoveOrCapture::Move, square2))
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_square_captures_square<SAN>, 
    do_parse!(
	piece: san_piece >>
	square1: san_square >>
	capture: san_capture >>
	square2: san_square >>
	(SAN::PieceSquareToSquare(piece, square1, capture, square2))
    )
);

#[cfg(test)]
mod tests {

    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_san_piece() {
	assert_eq!(Done(&b""[..], PAWN), san_piece(&b"p"[..]));
	assert_eq!(Done(&b""[..], PAWN), san_piece(&b"P"[..]));
	assert_eq!(Done(&b""[..], KNIGHT), san_piece(&b"n"[..]));
	assert_eq!(Done(&b""[..], KNIGHT), san_piece(&b"N"[..]));
	assert_eq!(Done(&b""[..], BISHOP), san_piece(&b"b"[..]));
	assert_eq!(Done(&b""[..], BISHOP), san_piece(&b"B"[..]));
	assert_eq!(Done(&b""[..], ROOK), san_piece(&b"r"[..]));
	assert_eq!(Done(&b""[..], ROOK), san_piece(&b"R"[..]));
	assert_eq!(Done(&b""[..], QUEEN), san_piece(&b"q"[..]));
	assert_eq!(Done(&b""[..], QUEEN), san_piece(&b"Q"[..]));
	assert_eq!(Done(&b""[..], KING), san_piece(&b"k"[..]));
	assert_eq!(Done(&b""[..], KING), san_piece(&b"K"[..]));
    }

    #[test]
    fn test_san_file() {
	assert_eq!(Done(&b""[..], FILE_A), san_file(&b"a"[..]));
	assert_eq!(Done(&b""[..], FILE_A), san_file(&b"A"[..]));
	assert_eq!(Done(&b""[..], FILE_B), san_file(&b"b"[..]));
	assert_eq!(Done(&b""[..], FILE_B), san_file(&b"B"[..]));
	assert_eq!(Done(&b""[..], FILE_C), san_file(&b"c"[..]));
	assert_eq!(Done(&b""[..], FILE_C), san_file(&b"C"[..]));
	assert_eq!(Done(&b""[..], FILE_D), san_file(&b"d"[..]));
	assert_eq!(Done(&b""[..], FILE_D), san_file(&b"D"[..]));
	assert_eq!(Done(&b""[..], FILE_E), san_file(&b"e"[..]));
	assert_eq!(Done(&b""[..], FILE_E), san_file(&b"E"[..]));
	assert_eq!(Done(&b""[..], FILE_F), san_file(&b"f"[..]));
	assert_eq!(Done(&b""[..], FILE_F), san_file(&b"F"[..]));
	assert_eq!(Done(&b""[..], FILE_G), san_file(&b"g"[..]));
	assert_eq!(Done(&b""[..], FILE_G), san_file(&b"G"[..]));
	assert_eq!(Done(&b""[..], FILE_H), san_file(&b"h"[..]));
	assert_eq!(Done(&b""[..], FILE_H), san_file(&b"H"[..]));
    }
    #[test]
    fn test_san_rank() {
	assert_eq!(Done(&b""[..], RANK_1), san_rank(&b"1"[..]));
	assert_eq!(Done(&b""[..], RANK_2), san_rank(&b"2"[..]));
	assert_eq!(Done(&b""[..], RANK_3), san_rank(&b"3"[..]));
	assert_eq!(Done(&b""[..], RANK_4), san_rank(&b"4"[..]));
	assert_eq!(Done(&b""[..], RANK_5), san_rank(&b"5"[..]));
	assert_eq!(Done(&b""[..], RANK_6), san_rank(&b"6"[..]));
	assert_eq!(Done(&b""[..], RANK_7), san_rank(&b"7"[..]));
	assert_eq!(Done(&b""[..], RANK_8), san_rank(&b"8"[..]));
    }
    #[test]
    fn test_san_square() {
	assert_eq!(Done(&b""[..], SQ_A1), san_square(&b"a1"[..]));
	assert_eq!(Done(&b""[..], SQ_A1), san_square(&b"A1"[..]));
	assert_eq!(Done(&b""[..], SQ_A8), san_square(&b"a8"[..]));
	assert_eq!(Done(&b""[..], SQ_A8), san_square(&b"A8"[..]));
	assert_eq!(Done(&b""[..], SQ_H1), san_square(&b"h1"[..]));
	assert_eq!(Done(&b""[..], SQ_H1), san_square(&b"H1"[..]));
	assert_eq!(Done(&b""[..], SQ_H8), san_square(&b"h8"[..]));
	assert_eq!(Done(&b""[..], SQ_H8), san_square(&b"H8"[..]));
    }
    #[test]
    fn test_san_pawn_to_square() {
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(PAWN, MoveOrCapture::Move, SQ_E4)), san_pawn_to_square(&b"e4"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(PAWN, MoveOrCapture::Move, SQ_D4)), san_pawn_to_square(&b"d4"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(PAWN, MoveOrCapture::Move, SQ_C4)), san_pawn_to_square(&b"c4"[..]));
    }
    #[test]
    fn test_san_pawn_capture_square() {
	assert_eq!(Done(&b""[..], SAN::PieceFileToSquare(PAWN, FILE_D, MoveOrCapture::Capture, SQ_E4)), san_pawn_captures_square(&b"dxe4"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceFileToSquare(PAWN, FILE_E, MoveOrCapture::Capture, SQ_D4)), san_pawn_captures_square(&b"exd4"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceFileToSquare(PAWN, FILE_D, MoveOrCapture::Capture, SQ_C4)), san_pawn_captures_square(&b"dxc4"[..]));
    }
    #[test]
    fn test_san_piece_to_square() {
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(KNIGHT, MoveOrCapture::Move, SQ_F3)), san_piece_to_square(&b"Nf3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(BISHOP, MoveOrCapture::Move, SQ_B5)), san_piece_to_square(&b"Bb5"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(QUEEN, MoveOrCapture::Move, SQ_D8)), san_piece_to_square(&b"Qd8"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(ROOK, MoveOrCapture::Move, SQ_D1)), san_piece_to_square(&b"Rd1"[..]));
    }
    #[test]
    fn test_san_piece_captures_square() {
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(KNIGHT, MoveOrCapture::Capture, SQ_F3)), san_piece_captures_square(&b"Nxf3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(BISHOP, MoveOrCapture::Capture, SQ_B5)), san_piece_captures_square(&b"Bxb5"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(QUEEN, MoveOrCapture::Capture, SQ_D8)), san_piece_captures_square(&b"Qxd8"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceToSquare(ROOK, MoveOrCapture::Capture, SQ_D1)), san_piece_captures_square(&b"Rxd1"[..]));
    }
    #[test]
    fn test_san_piece_file_to_square() {
	assert_eq!(Done(&b""[..], SAN::PieceFileToSquare(KNIGHT, FILE_E, MoveOrCapture::Move, SQ_F3)), san_piece_file_to_square(&b"Nef3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceFileToSquare(ROOK, FILE_E, MoveOrCapture::Move, SQ_D1)), san_piece_file_to_square(&b"Red1"[..]));
    }
    #[test]
    fn test_san_piece_file_captures_square() {
	assert_eq!(Done(&b""[..], SAN::PieceFileToSquare(KNIGHT, FILE_E, MoveOrCapture::Capture, SQ_F3)), san_piece_file_captures_square(&b"Nexf3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceFileToSquare(ROOK, FILE_E, MoveOrCapture::Capture, SQ_D1)), san_piece_file_captures_square(&b"Rexd1"[..]));
    }

    #[test]
    fn test_san_piece_rank_to_square() {
	assert_eq!(Done(&b""[..], SAN::PieceRankToSquare(KNIGHT, RANK_2, MoveOrCapture::Move, SQ_F3)), san_piece_rank_to_square(&b"N2f3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceRankToSquare(ROOK, RANK_3, MoveOrCapture::Move, SQ_D1)), san_piece_rank_to_square(&b"R3d1"[..]));
    }
    #[test]
    fn test_san_piece_rank_captures_square() {
	assert_eq!(Done(&b""[..], SAN::PieceRankToSquare(KNIGHT, RANK_1, MoveOrCapture::Capture, SQ_F3)), san_piece_rank_captures_square(&b"N1xf3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceRankToSquare(ROOK, RANK_6, MoveOrCapture::Capture, SQ_D1)), san_piece_rank_captures_square(&b"R6xd1"[..]));
    }
    #[test]
    fn test_san_piece_square_to_square() {
	assert_eq!(Done(&b""[..], SAN::PieceSquareToSquare(KNIGHT, SQ_F1, MoveOrCapture::Move, SQ_F3)), san_piece_square_to_square(&b"Nf1f3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceSquareToSquare(ROOK, SQ_D3, MoveOrCapture::Move, SQ_D1)), san_piece_square_to_square(&b"Rd3d1"[..]));
    }
    #[test]
    fn test_san_piece_square_captures_square() {
	assert_eq!(Done(&b""[..], SAN::PieceSquareToSquare(KNIGHT, SQ_F1, MoveOrCapture::Capture, SQ_F3)), san_piece_square_captures_square(&b"Nf1xf3"[..]));
	assert_eq!(Done(&b""[..], SAN::PieceSquareToSquare(ROOK, SQ_D3, MoveOrCapture::Capture, SQ_D1)), san_piece_square_captures_square(&b"Rd3xd1"[..]));
    }
}
