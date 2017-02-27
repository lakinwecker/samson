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


///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Node {
    Drop(Piece),
    Skip(u8),
    NextRank,
    Move(Color),
    Castle(Color, File),
    NoCastling,
    EnPassantTargetSquare(Square),
    HalfMoveClock(u16),
    FullMoveNumber(u16),
    Error(char)
}

///-----------------------------------------------------------------------------
named!(fen_char<Node>, 
    map!(
	one_of!("rnbkqp/RNBKQP12345678"),
	|c: char| {
	    match c {
		'r' => Node::Drop(B_ROOK),
		'n' => Node::Drop(B_KNIGHT),
		'b' => Node::Drop(B_BISHOP),
		'q' => Node::Drop(B_QUEEN),
		'k' => Node::Drop(B_KING),
		'p' => Node::Drop(B_PAWN),
		'R' => Node::Drop(W_ROOK),
		'N' => Node::Drop(W_KNIGHT),
		'B' => Node::Drop(W_BISHOP),
		'Q' => Node::Drop(W_QUEEN),
		'K' => Node::Drop(W_KING),
		'P' => Node::Drop(W_PAWN),
		'1' => Node::Skip(1),
		'2' => Node::Skip(2),
		'3' => Node::Skip(3),
		'4' => Node::Skip(4),
		'5' => Node::Skip(5),
		'6' => Node::Skip(6),
		'7' => Node::Skip(7),
		'8' => Node::Skip(8),
		'/' => Node::NextRank,
		_ => Node::Error(c) // This should never happen because of above.
	    }
	}
    )
);

///-----------------------------------------------------------------------------
named!(pub piece_placement<&[u8], Vec<Node> >, many0!(fen_char));

///-----------------------------------------------------------------------------
named!(pub color_to_move<&[u8], Node >,
    map!(one_of!("wb-"), |c: char| { match c { 
	'w' => Node::Move(WHITE),
	'b' => Node::Move(BLACK),
	'-' => Node::Move(NO_COLOR),
	_ => Node::Error(c) // This should never happen because of above.
    }})
);

///-----------------------------------------------------------------------------
named!(pub castling_rights<&[u8], Vec<Node> >,
    many0!(
	map!(one_of!("-KQkqABCEDFGHabcdefgh"), |c: char| { match c { 
	    'k' => Node::Castle(WHITE, FILE_H),
	    'q' => Node::Castle(WHITE, FILE_A),
	    'a' => Node::Castle(WHITE, FILE_A),
	    'b' => Node::Castle(WHITE, FILE_B),
	    'c' => Node::Castle(WHITE, FILE_C),
	    'd' => Node::Castle(WHITE, FILE_D),
	    'e' => Node::Castle(WHITE, FILE_E),
	    'f' => Node::Castle(WHITE, FILE_F),
	    'g' => Node::Castle(WHITE, FILE_G),
	    'h' => Node::Castle(WHITE, FILE_H),
	    'K' => Node::Castle(BLACK, FILE_H),
	    'Q' => Node::Castle(BLACK, FILE_A),
	    'A' => Node::Castle(BLACK, FILE_A),
	    'B' => Node::Castle(BLACK, FILE_B),
	    'C' => Node::Castle(BLACK, FILE_C),
	    'D' => Node::Castle(BLACK, FILE_D),
	    'E' => Node::Castle(BLACK, FILE_E),
	    'F' => Node::Castle(BLACK, FILE_F),
	    'G' => Node::Castle(BLACK, FILE_G),
	    'H' => Node::Castle(BLACK, FILE_H),
	    '-' => Node::NoCastling,
	    _ => Node::Error(c) // This should never happen because of above.
	}})
    )
);

#[cfg(test)]
mod tests {

    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_parse_starting_position() {
	// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
	let fen = &b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"[..];
	let fen_actions = vec![
	    Node::Drop(B_ROOK), Node::Drop(B_KNIGHT), Node::Drop(B_BISHOP), Node::Drop(B_QUEEN),
	    Node::Drop(B_KING), Node::Drop(B_BISHOP), Node::Drop(B_KNIGHT), Node::Drop(B_ROOK),
	    Node::NextRank,
	    Node::Drop(B_PAWN), Node::Drop(B_PAWN), Node::Drop(B_PAWN), Node::Drop(B_PAWN),
	    Node::Drop(B_PAWN), Node::Drop(B_PAWN), Node::Drop(B_PAWN), Node::Drop(B_PAWN),
	    Node::NextRank,
	    Node::Skip(8),
	    Node::NextRank,
	    Node::Skip(8),
	    Node::NextRank,
	    Node::Skip(8),
	    Node::NextRank,
	    Node::Skip(8),
	    Node::NextRank,
	    Node::Drop(W_PAWN), Node::Drop(W_PAWN), Node::Drop(W_PAWN), Node::Drop(W_PAWN),
	    Node::Drop(W_PAWN), Node::Drop(W_PAWN), Node::Drop(W_PAWN), Node::Drop(W_PAWN),
	    Node::NextRank,
	    Node::Drop(W_ROOK), Node::Drop(W_KNIGHT), Node::Drop(W_BISHOP), Node::Drop(W_QUEEN),
	    Node::Drop(W_KING), Node::Drop(W_BISHOP), Node::Drop(W_KNIGHT), Node::Drop(W_ROOK)
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
	assert_eq!(Done(&b""[..], Node::Move(WHITE)), color_to_move(fen));
    }
    #[test]
    fn test_castling_rights() {
	let fen = &b"KQkq"[..];
	let expected = vec![
	    Node::Castle(BLACK, FILE_H), Node::Castle(BLACK, FILE_A), Node::Castle(WHITE, FILE_H), Node::Castle(WHITE, FILE_A)
	];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
	let fen = &b"Kq"[..];
	let expected = vec![Node::Castle(BLACK, FILE_H), Node::Castle(WHITE, FILE_A)];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
	let fen = &b"Qk"[..];
	let expected = vec![Node::Castle(BLACK, FILE_A), Node::Castle(WHITE, FILE_H)];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
	let fen = &b"-"[..];
	let expected = vec![Node::NoCastling];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));

	let fen = &b"HAha"[..];
	let expected = vec![
	    Node::Castle(BLACK, FILE_H), Node::Castle(BLACK, FILE_A), Node::Castle(WHITE, FILE_H), Node::Castle(WHITE, FILE_A)
	];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
	let fen = &b"AHah"[..];
	let expected = vec![
	    Node::Castle(BLACK, FILE_A), Node::Castle(BLACK, FILE_H), Node::Castle(WHITE, FILE_A), Node::Castle(WHITE, FILE_H)
	];
	assert_eq!(Done(&b""[..], expected), castling_rights(fen));
    }
}
