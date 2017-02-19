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

pub enum MoveOrCapture { Move, Capture }

pub enum SAN {
    PieceToSquare(PieceType, MoveOrCapture, Square),
    PieceRankToSquare(PieceType, Rank, MoveOrCapture, Square),
    PieceFileToSquare(PieceType, File, MoveOrCapture, Square),
    PieceSquareToSquare(PieceType, Square, MoveOrCapture, Square),
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
}
