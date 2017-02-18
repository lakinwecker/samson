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
// Parsers for the FEN specification
//------------------------------------------------------------------------------

use super::super::types::*;
use nom::*;


///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum FEN {
    Drop(Piece),
    Skip(u8),
    NextRank,
    Move(Color),
    Castling,
    WhiteCastleA,
    WhiteCastleB,
    WhiteCastleC,
    WhiteCastleD,
    WhiteCastleE,
    WhiteCastleF,
    WhiteCastleG,
    WhiteCastleH,
    BlackCastleA,
    BlackCastleB,
    BlackCastleC,
    BlackCastleD,
    BlackCastleE,
    BlackCastleF,
    BlackCastleG,
    BlackCastleH,
    NoCastling,
    EnPassantTargetSquare(Square),
    HalfMoveClock(u16),
    FullMoveNumber(u16),
    Error(char)
}

///-----------------------------------------------------------------------------
named!(fen_char<FEN>, 
    map!(
	one_of!("rnbkqp/RNBKQP12345678"),
	|c: char| {
	    match c {
		'r' => FEN::Drop(B_ROOK),
		'n' => FEN::Drop(B_KNIGHT),
		'b' => FEN::Drop(B_BISHOP),
		'q' => FEN::Drop(B_QUEEN),
		'k' => FEN::Drop(B_KING),
		'p' => FEN::Drop(B_PAWN),
		'R' => FEN::Drop(W_ROOK),
		'N' => FEN::Drop(W_KNIGHT),
		'B' => FEN::Drop(W_BISHOP),
		'Q' => FEN::Drop(W_QUEEN),
		'K' => FEN::Drop(W_KING),
		'P' => FEN::Drop(W_PAWN),
		'1' => FEN::Skip(1),
		'2' => FEN::Skip(2),
		'3' => FEN::Skip(3),
		'4' => FEN::Skip(4),
		'5' => FEN::Skip(5),
		'6' => FEN::Skip(6),
		'7' => FEN::Skip(7),
		'8' => FEN::Skip(8),
		'/' => FEN::NextRank,
		_ => FEN::Error(c) // This should never happen because of above.
	    }
	}
    )
);

///-----------------------------------------------------------------------------
named!(pub piece_placement<&[u8], Vec<FEN> >, many0!(fen_char));

///-----------------------------------------------------------------------------
named!(pub color_to_move<&[u8], FEN >,
    map!(one_of!("wb-"), |c: char| { match c { 
	'w' => FEN::Move(WHITE),
	'b' => FEN::Move(BLACK),
	'-' => FEN::Move(NO_COLOR),
	_ => FEN::Error(c) // This should never happen because of above.
    }})
);

///-----------------------------------------------------------------------------
named!(pub castling_rights<&[u8], Vec<FEN> >,
    many0!(
	map!(one_of!("-KQkqABCEDFGHabcdefgh"), |c: char| { match c { 
	    'k' => FEN::WhiteCastleH,
	    'q' => FEN::WhiteCastleA,
	    'a' => FEN::WhiteCastleA,
	    'b' => FEN::WhiteCastleB,
	    'c' => FEN::WhiteCastleC,
	    'd' => FEN::WhiteCastleD,
	    'e' => FEN::WhiteCastleE,
	    'f' => FEN::WhiteCastleF,
	    'g' => FEN::WhiteCastleG,
	    'h' => FEN::WhiteCastleH,
	    'K' => FEN::BlackCastleH,
	    'Q' => FEN::BlackCastleA,
	    'A' => FEN::BlackCastleA,
	    'B' => FEN::BlackCastleB,
	    'C' => FEN::BlackCastleC,
	    'D' => FEN::BlackCastleD,
	    'E' => FEN::BlackCastleE,
	    'F' => FEN::BlackCastleF,
	    'G' => FEN::BlackCastleG,
	    'H' => FEN::BlackCastleH,
	    '-' => FEN::NoCastling,
	    _ => FEN::Error(c) // This should never happen because of above.
	}})
    )
);

#[cfg(test)]
mod tests {

    use super::super::super::*;
    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_parse_starting_position() {
	// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
	let fen = &b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"[..];
	let fen_actions = vec![
	    FEN::Drop(B_ROOK), FEN::Drop(B_KNIGHT), FEN::Drop(B_BISHOP), FEN::Drop(B_QUEEN),
	    FEN::Drop(B_KING), FEN::Drop(B_BISHOP), FEN::Drop(B_KNIGHT), FEN::Drop(B_ROOK),
	    FEN::NextRank,
	    FEN::Drop(B_PAWN), FEN::Drop(B_PAWN), FEN::Drop(B_PAWN), FEN::Drop(B_PAWN),
	    FEN::Drop(B_PAWN), FEN::Drop(B_PAWN), FEN::Drop(B_PAWN), FEN::Drop(B_PAWN),
	    FEN::NextRank,
	    FEN::Skip(8),
	    FEN::NextRank,
	    FEN::Skip(8),
	    FEN::NextRank,
	    FEN::Skip(8),
	    FEN::NextRank,
	    FEN::Skip(8),
	    FEN::NextRank,
	    FEN::Drop(W_PAWN), FEN::Drop(W_PAWN), FEN::Drop(W_PAWN), FEN::Drop(W_PAWN),
	    FEN::Drop(W_PAWN), FEN::Drop(W_PAWN), FEN::Drop(W_PAWN), FEN::Drop(W_PAWN),
	    FEN::NextRank,
	    FEN::Drop(W_ROOK), FEN::Drop(W_KNIGHT), FEN::Drop(W_BISHOP), FEN::Drop(W_QUEEN),
	    FEN::Drop(W_KING), FEN::Drop(W_BISHOP), FEN::Drop(W_KNIGHT), FEN::Drop(W_ROOK)
	];
	match piece_placement(fen) {
	    Done(_, actions) => assert_eq!(actions, fen_actions),
	    Incomplete(_) => assert_eq!(false, true, "Error parsing base fen"),
	    Error(_) => assert_eq!(false, true, "Error parsing base fen") 
	}
    }
    #[test]
    fn test_color_to_move() {
	let fen = &b"w"[..];
	assert_eq!(Done(&b""[..], FEN::Move(WHITE)), color_to_move(fen));
    }
    #[test]
    fn test_castling_rights() {
	let fen = &b"KQkq"[..];
	let expected = vec![
	    FEN::BlackCastleH, FEN::BlackCastleA, FEN::WhiteCastleH, FEN::WhiteCastleA
	];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
	let fen = &b"Kq"[..];
	let expected = vec![FEN::BlackCastleH, FEN::WhiteCastleA];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
	let fen = &b"Qk"[..];
	let expected = vec![FEN::BlackCastleA, FEN::WhiteCastleH];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
	let fen = &b"-"[..];
	let expected = vec![FEN::NoCastling];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));

	let fen = &b"HAha"[..];
	let expected = vec![
	    FEN::BlackCastleH, FEN::BlackCastleA, FEN::WhiteCastleH, FEN::WhiteCastleA
	];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
    }
}
