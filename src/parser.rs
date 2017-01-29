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
// Parsers for chess related functionality, including UCI, PGN, etc.
//
// Shamelessly patterned after the amazing python-chess library by Niklas Fiekas
//------------------------------------------------------------------------------

use super::types;

//------------------------------------------------------------------------------
// UCI related parsers
named!(pub file <types::File>, chain!(
    f: one_of!("abcdefghABCDEFGH0"),
    || { 
        match f {
            'a' | 'A' => types::FILE_A,
            'b' | 'B' => types::FILE_B,
            'c' | 'C' => types::FILE_C,
            'd' | 'D' => types::FILE_D,
            'e' | 'E' => types::FILE_E,
            'f' | 'F' => types::FILE_F,
            'g' | 'G' => types::FILE_G,
            'h' | 'H' => types::FILE_H,
            '0' => types::FILE_NB,
            _ => types::FILE_NB
        }
    }
));
named!(pub rank <types::Rank>, chain!(
    r: one_of!("123456780"),
    || { 
        match r {
            '1' => types::RANK_1,
            '2' => types::RANK_2,
            '3' => types::RANK_3,
            '4' => types::RANK_4,
            '5' => types::RANK_5,
            '6' => types::RANK_6,
            '7' => types::RANK_7,
            '8' => types::RANK_8,
            '0' => types::RANK_NB,
            _ => types::RANK_NB
        }
    }
));
named!(pub piece <types::PieceType>, complete!(chain!(
    p: one_of!("pnbrqkPNBRQK"),
    || { 
        match p {
            'p' | 'P' => types::PAWN,
            'n' | 'N' => types::KNIGHT,
            'b' | 'B' => types::BISHOP,
            'r' | 'R' => types::ROOK,
            'q' | 'Q' => types::QUEEN,
            'k' | 'K' => types::KING,
            _ => types::NO_PIECE_TYPE
        }
    }
)));

named!(pub square <types::Square>, chain!(f: file ~ r: rank, || {
    match (f, r) {
        (types::FILE_NB, _) | (_, types::RANK_NB) => types::SQUARE_NB,
        _ => types::make_square(f, r)
    }
}));

named!(pub uci <types::Move>, chain!(
    from: square ~
    to: square ~
    promotion: piece? ,
    || {
        match (from, to) {
            (types::SQUARE_NB, _) | (_, types::SQUARE_NB) => types::MOVE_NULL,
            _ => match promotion {
                Some(p) => types::make_move_with_promotion(from, to, p),
                None => types::make_move_simple(from, to)
            }
        }
    }
));

#[cfg(test)]
mod tests {

    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_square() {
        assert_eq!(Done(&[][..], types::SQ_A1), square(b"a1"));
        assert_eq!(Done(&[][..], types::SQ_A1), square(b"A1"));
        assert_eq!(Done(&[][..], types::SQ_B1), square(b"b1"));
        assert_eq!(Done(&[][..], types::SQ_B1), square(b"B1"));
        assert_eq!(Done(&[][..], types::SQ_C1), square(b"c1"));
        assert_eq!(Done(&[][..], types::SQ_C1), square(b"C1"));
        assert_eq!(Done(&[][..], types::SQ_D1), square(b"d1"));
        assert_eq!(Done(&[][..], types::SQ_D1), square(b"D1"));
        assert_eq!(Done(&[][..], types::SQ_E1), square(b"e1"));
        assert_eq!(Done(&[][..], types::SQ_E1), square(b"E1"));
        assert_eq!(Done(&[][..], types::SQ_F1), square(b"f1"));
        assert_eq!(Done(&[][..], types::SQ_F1), square(b"F1"));
        assert_eq!(Done(&[][..], types::SQ_G1), square(b"g1"));
        assert_eq!(Done(&[][..], types::SQ_G1), square(b"G1"));
        assert_eq!(Done(&[][..], types::SQ_H1), square(b"h1"));
        assert_eq!(Done(&[][..], types::SQ_H1), square(b"H1"));

        assert_eq!(Done(&[][..], types::SQ_A2), square(b"a2"));
        assert_eq!(Done(&[][..], types::SQ_A2), square(b"A2"));
        assert_eq!(Done(&[][..], types::SQ_B2), square(b"b2"));
        assert_eq!(Done(&[][..], types::SQ_B2), square(b"B2"));
        assert_eq!(Done(&[][..], types::SQ_C2), square(b"c2"));
        assert_eq!(Done(&[][..], types::SQ_C2), square(b"C2"));
        assert_eq!(Done(&[][..], types::SQ_D2), square(b"d2"));
        assert_eq!(Done(&[][..], types::SQ_D2), square(b"D2"));
        assert_eq!(Done(&[][..], types::SQ_E2), square(b"e2"));
        assert_eq!(Done(&[][..], types::SQ_E2), square(b"E2"));
        assert_eq!(Done(&[][..], types::SQ_F2), square(b"f2"));
        assert_eq!(Done(&[][..], types::SQ_F2), square(b"F2"));
        assert_eq!(Done(&[][..], types::SQ_G2), square(b"g2"));
        assert_eq!(Done(&[][..], types::SQ_G2), square(b"G2"));
        assert_eq!(Done(&[][..], types::SQ_H2), square(b"h2"));
        assert_eq!(Done(&[][..], types::SQ_H2), square(b"H2"));
    }

    #[test]
    fn test_piece() {
        assert_eq!(Done(&[][..], types::PAWN), piece(b"p"));
        assert_eq!(Done(&[][..], types::ROOK), piece(b"r"));
        assert_eq!(Done(&[][..], types::KNIGHT), piece(b"n"));
        assert_eq!(Done(&[][..], types::BISHOP), piece(b"b"));
        assert_eq!(Done(&[][..], types::QUEEN), piece(b"q"));
        assert_eq!(Done(&[][..], types::KING), piece(b"k"));

        assert_eq!(Done(&[][..], types::PAWN), piece(b"P"));
        assert_eq!(Done(&[][..], types::ROOK), piece(b"R"));
        assert_eq!(Done(&[][..], types::KNIGHT), piece(b"N"));
        assert_eq!(Done(&[][..], types::BISHOP), piece(b"B"));
        assert_eq!(Done(&[][..], types::QUEEN), piece(b"Q"));
        assert_eq!(Done(&[][..], types::KING), piece(b"K"));
    }

    #[test]
    fn test_uci() {
        assert_eq!(Done(&[][..], types::make_move_with_promotion(types::SQ_E2, types::SQ_E4, types::KNIGHT)), uci(b"e2e4n"));
        assert_eq!(Done(&[][..], types::MOVE_NULL), uci(b"0000"));
    }
}
