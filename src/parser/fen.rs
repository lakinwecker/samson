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
    WhiteCastleKingSide,
    WhiteCastleQueenSide,
    BlackCastleKingSide,
    BlackCastleQueenSide,
    EnPassantTargetSquare(Square),
    HalfMoveClock(u16),
    FullMoveNumber(u16),
    Error(char)
}

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
named!(pub piece_placement<&[u8], Vec<FEN> >,
    fold_many0!(fen_char,
	Vec::new(),
	|mut acc: Vec<_>, item| {
	    acc.push(item);
	    acc
	}
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
}
