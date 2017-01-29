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

use super::types::*;

//------------------------------------------------------------------------------
// UCI related parsers
named!(pub file <File>, chain!(
    f: one_of!("abcdefghABCDEFGH0"),
    || { 
        match f {
            'a' | 'A' => FILE_A,
            'b' | 'B' => FILE_B,
            'c' | 'C' => FILE_C,
            'd' | 'D' => FILE_D,
            'e' | 'E' => FILE_E,
            'f' | 'F' => FILE_F,
            'g' | 'G' => FILE_G,
            'h' | 'H' => FILE_H,
            '0' => FILE_NB,
            _ => FILE_NB
        }
    }
));
named!(pub rank <Rank>, chain!(
    r: one_of!("123456780"),
    || { 
        match r {
            '1' => RANK_1,
            '2' => RANK_2,
            '3' => RANK_3,
            '4' => RANK_4,
            '5' => RANK_5,
            '6' => RANK_6,
            '7' => RANK_7,
            '8' => RANK_8,
            '0' => RANK_NB,
            _ => RANK_NB
        }
    }
));
named!(pub piece <PieceType>, complete!(chain!(
    p: one_of!("pnbrqkPNBRQK"),
    || { 
        match p {
            'p' | 'P' => PAWN,
            'n' | 'N' => KNIGHT,
            'b' | 'B' => BISHOP,
            'r' | 'R' => ROOK,
            'q' | 'Q' => QUEEN,
            'k' | 'K' => KING,
            _ => NO_PIECE_TYPE
        }
    }
)));

named!(pub sq <Square>, chain!(f: File ~ r: Rank, || {
    match (f, r) {
        (FILE_NB, _) | (_, RANK_NB) => SQUARE_NB,
        _ => make_square(f, r)
    }
}));

named!(pub uci <Move>, chain!(
    from: sq ~
    to: sq ~
    promotion: piece? ,
    || {
        match (from, to) {
            (SQUARE_NB, _) | (_, SQUARE_NB) => MOVE_NULL,
            _ => match promotion {
                Some(p) => make_move_with_promotion(from, to, p),
                None => make_move(from, to)
            }
        }
    }
));

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::*;
    use nom::IResult::*;

    #[test]
    fn test_parse_square() {
        assert_eq!(Done(&[][..], SQ_A1), sq(b"a1"));
        assert_eq!(Done(&[][..], SQ_A1), sq(b"A1"));
        assert_eq!(Done(&[][..], SQ_B1), sq(b"b1"));
        assert_eq!(Done(&[][..], SQ_B1), sq(b"B1"));
        assert_eq!(Done(&[][..], SQ_C1), sq(b"c1"));
        assert_eq!(Done(&[][..], SQ_C1), sq(b"C1"));
        assert_eq!(Done(&[][..], SQ_D1), sq(b"d1"));
        assert_eq!(Done(&[][..], SQ_D1), sq(b"D1"));
        assert_eq!(Done(&[][..], SQ_E1), sq(b"e1"));
        assert_eq!(Done(&[][..], SQ_E1), sq(b"E1"));
        assert_eq!(Done(&[][..], SQ_F1), sq(b"f1"));
        assert_eq!(Done(&[][..], SQ_F1), sq(b"F1"));
        assert_eq!(Done(&[][..], SQ_G1), sq(b"g1"));
        assert_eq!(Done(&[][..], SQ_G1), sq(b"G1"));
        assert_eq!(Done(&[][..], SQ_H1), sq(b"h1"));
        assert_eq!(Done(&[][..], SQ_H1), sq(b"H1"));

        assert_eq!(Done(&[][..], SQ_A2), sq(b"a2"));
        assert_eq!(Done(&[][..], SQ_A2), sq(b"A2"));
        assert_eq!(Done(&[][..], SQ_B2), sq(b"b2"));
        assert_eq!(Done(&[][..], SQ_B2), sq(b"B2"));
        assert_eq!(Done(&[][..], SQ_C2), sq(b"c2"));
        assert_eq!(Done(&[][..], SQ_C2), sq(b"C2"));
        assert_eq!(Done(&[][..], SQ_D2), sq(b"d2"));
        assert_eq!(Done(&[][..], SQ_D2), sq(b"D2"));
        assert_eq!(Done(&[][..], SQ_E2), sq(b"e2"));
        assert_eq!(Done(&[][..], SQ_E2), sq(b"E2"));
        assert_eq!(Done(&[][..], SQ_F2), sq(b"f2"));
        assert_eq!(Done(&[][..], SQ_F2), sq(b"F2"));
        assert_eq!(Done(&[][..], SQ_G2), sq(b"g2"));
        assert_eq!(Done(&[][..], SQ_G2), sq(b"G2"));
        assert_eq!(Done(&[][..], SQ_H2), sq(b"h2"));
        assert_eq!(Done(&[][..], SQ_H2), sq(b"H2"));
    }

    #[test]
    fn test_parse_piece() {
        assert_eq!(Done(&[][..], PAWN), piece(b"p"));
        assert_eq!(Done(&[][..], ROOK), piece(b"r"));
        assert_eq!(Done(&[][..], KNIGHT), piece(b"n"));
        assert_eq!(Done(&[][..], BISHOP), piece(b"b"));
        assert_eq!(Done(&[][..], QUEEN), piece(b"q"));
        assert_eq!(Done(&[][..], KING), piece(b"k"));

        assert_eq!(Done(&[][..], PAWN), piece(b"P"));
        assert_eq!(Done(&[][..], ROOK), piece(b"R"));
        assert_eq!(Done(&[][..], KNIGHT), piece(b"N"));
        assert_eq!(Done(&[][..], BISHOP), piece(b"B"));
        assert_eq!(Done(&[][..], QUEEN), piece(b"Q"));
        assert_eq!(Done(&[][..], KING), piece(b"K"));
    }

    #[test]
    fn test_parse_uci() {
        assert_eq!(Done(&[][..], make_move(SQ_E2, SQ_E4, PAWN)), uci(b"e2e4p"));
        assert_eq!(Done(&[][..], MOVE_NULL), uci(b"0000"));
    }
}
