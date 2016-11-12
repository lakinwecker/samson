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

use super::*;

//------------------------------------------------------------------------------
// UCI related parsers
named!(pub file <usize>, chain!(
    f: one_of!("abcdefghABCDEFGH"),
    || { 
        match f {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 6,
            'h' | 'H' => 7,
            _ => 10 // NOTE: this  will never occur provided the above parser isn't buggy. :P
        }
    }
));
named!(pub rank <usize>, chain!(
    r: one_of!("12345678"),
    || { 
        match r {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => 10 // NOTE: this  will never occur provided the above parser isn't buggy. :P
        }
    }
));
named!(pub piece <PieceType>, chain!(
    p: one_of!("pnbrqkPNBRQK"),
    || { 
        match p {
            'p' | 'P' => PieceType::Pawn,
            'n' | 'N' => PieceType::Knight,
            'b' | 'B' => PieceType::Bishop,
            'r' | 'R' => PieceType::Rook,
            'q' | 'Q' => PieceType::Queen,
            'k' | 'K' => PieceType::King,
            _ => PieceType::Null // NOTE: this  will never occur provided the above parser isn't buggy. :P
        }
    }
));

named!(pub sq <usize>, chain!(f: file ~ r: rank, || { square(f, r) }));
named!(pub uci <Move>, chain!(
    from: sq ~
    to: sq ~
    promotion: piece? ,
    || {
        match promotion {
            Some(p) => Move::new_with_promotion(from, to, p),
            None => Move::new(from, to)
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
        assert_eq!(Done(&[][..], A1), sq(b"a1"));
        assert_eq!(Done(&[][..], A1), sq(b"A1"));
        assert_eq!(Done(&[][..], B1), sq(b"b1"));
        assert_eq!(Done(&[][..], B1), sq(b"B1"));
        assert_eq!(Done(&[][..], C1), sq(b"c1"));
        assert_eq!(Done(&[][..], C1), sq(b"C1"));
        assert_eq!(Done(&[][..], D1), sq(b"d1"));
        assert_eq!(Done(&[][..], D1), sq(b"D1"));
        assert_eq!(Done(&[][..], E1), sq(b"e1"));
        assert_eq!(Done(&[][..], E1), sq(b"E1"));
        assert_eq!(Done(&[][..], F1), sq(b"f1"));
        assert_eq!(Done(&[][..], F1), sq(b"F1"));
        assert_eq!(Done(&[][..], G1), sq(b"g1"));
        assert_eq!(Done(&[][..], G1), sq(b"G1"));
        assert_eq!(Done(&[][..], H1), sq(b"h1"));
        assert_eq!(Done(&[][..], H1), sq(b"H1"));

        assert_eq!(Done(&[][..], A2), sq(b"a2"));
        assert_eq!(Done(&[][..], A2), sq(b"A2"));
        assert_eq!(Done(&[][..], B2), sq(b"b2"));
        assert_eq!(Done(&[][..], B2), sq(b"B2"));
        assert_eq!(Done(&[][..], C2), sq(b"c2"));
        assert_eq!(Done(&[][..], C2), sq(b"C2"));
        assert_eq!(Done(&[][..], D2), sq(b"d2"));
        assert_eq!(Done(&[][..], D2), sq(b"D2"));
        assert_eq!(Done(&[][..], E2), sq(b"e2"));
        assert_eq!(Done(&[][..], E2), sq(b"E2"));
        assert_eq!(Done(&[][..], F2), sq(b"f2"));
        assert_eq!(Done(&[][..], F2), sq(b"F2"));
        assert_eq!(Done(&[][..], G2), sq(b"g2"));
        assert_eq!(Done(&[][..], G2), sq(b"G2"));
        assert_eq!(Done(&[][..], H2), sq(b"h2"));
        assert_eq!(Done(&[][..], H2), sq(b"H2"));
    }

    #[test]
    fn test_parse_piece() {
        assert_eq!(Done(&[][..], PieceType::Pawn), piece(b"p"));
        assert_eq!(Done(&[][..], PieceType::Rook), piece(b"r"));
        assert_eq!(Done(&[][..], PieceType::Knight), piece(b"n"));
        assert_eq!(Done(&[][..], PieceType::Bishop), piece(b"b"));
        assert_eq!(Done(&[][..], PieceType::Queen), piece(b"q"));
        assert_eq!(Done(&[][..], PieceType::King), piece(b"k"));

        assert_eq!(Done(&[][..], PieceType::Pawn), piece(b"P"));
        assert_eq!(Done(&[][..], PieceType::Rook), piece(b"R"));
        assert_eq!(Done(&[][..], PieceType::Knight), piece(b"N"));
        assert_eq!(Done(&[][..], PieceType::Bishop), piece(b"B"));
        assert_eq!(Done(&[][..], PieceType::Queen), piece(b"Q"));
        assert_eq!(Done(&[][..], PieceType::King), piece(b"K"));
    }

    #[test]
    fn test_parse_uci() {
        assert_eq!(Done(&[][..], Move::new(E2, E4)), uci(b"e2e4"));
        assert_eq!(Done(&[][..], Move::new_with_promotion(E2, E4, PieceType::Pawn)), uci(b"e2e4p"));
    }
}
