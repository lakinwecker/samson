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
pub enum Source {
    None,
    Rank(Rank),
    File(File),
    Square(Square)
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Promotion {
    None,
    PieceType(PieceType),
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Check {
    None,
    Check,
    Checkmate
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum MoveAnnotation {
    None,
    Strong,
    Brilliant,
    Mistake,
    Blunder,
    Interesting,
    Dubious
}

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Node {
    Move(PieceType, Source, MoveOrCapture, Square, Promotion, Check, MoveAnnotation),
    CastleKingSide(Check, MoveAnnotation),
    CastleQueenSide(Check, MoveAnnotation),
    NullMove(Check, MoveAnnotation),
    InvalidMove
}

///-----------------------------------------------------------------------------
named!(pub san_piece<PieceType>, 
    map!(
        one_of!("PNBRQKpnbrqk"),
        |c: char| {
            match c {
                'P' => PAWN,
                'N' => KNIGHT,
                'B' => BISHOP,
                'R' => ROOK,
                'Q' => QUEEN,
                'K' => KING,
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
                'a' | 'A' => FILE_A,
                'b' | 'B' => FILE_B,
                'c' | 'C' => FILE_C,
                'd' | 'D' => FILE_D,
                'e' | 'E' => FILE_E,
                'f' | 'F' => FILE_F,
                'g' | 'G' => FILE_G,
                'h' | 'H' => FILE_H,
                _ => FILE_NB // This should never happen because of above.
            }
        }
    )
);

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
named!(pub san_promotion<char>, one_of!("="));

///-----------------------------------------------------------------------------
named!(pub san_check<Check>,
    map!(
        one_of!("+#"),
        |c: char| {
            match c {
                '+' => Check::Check,
                '#' => Check::Checkmate,
                _ => Check::None
            }
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_move_annotation<MoveAnnotation>,
    map!(
        alt_complete!(tag!("!!") | tag!("??") | tag!("?!") | tag!("!?") | tag!("!") | tag!("?")),
        |suffix: &[u8]| {
            match suffix {
                b"!!" => MoveAnnotation::Brilliant,
                b"!" => MoveAnnotation::Strong,
                b"??" => MoveAnnotation::Blunder,
                b"?" => MoveAnnotation::Mistake,
                b"!?" => MoveAnnotation::Interesting,
                b"?!" => MoveAnnotation::Dubious,
                _ => MoveAnnotation::None
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
named!(whitespace<char>, one_of!(" \r\n\t"));

///-----------------------------------------------------------------------------
named!(pub san_pawn_move_bare<Node>, 
    map!(
        do_parse!(
            square: complete!(san_square) >>
            whitespace >>
            (square)
        ),
        |square| {
            Node::Move(
                PAWN,
                Source::None,
                MoveOrCapture::Move,
                square,
                Promotion::None,
                Check::None, MoveAnnotation::None
            )
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_pawn_capture_bare<Node>, 
    map!(
        do_parse!(
            file: complete!(san_file) >>
            capture: complete!(san_capture) >>
            square: complete!(san_square) >>
            whitespace >>
            (file, capture, square)
        ),
        |(file, capture, square)| {
            Node::Move(
                PAWN,
                Source::File(file),
                capture,
                square,
                Promotion::None,
                Check::None, MoveAnnotation::None
            )
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_pawn_move<Node>, 
    map!(
        do_parse!(
            square: complete!(san_square) >>
            promotion: opt!(complete!(san_promotion)) >>
            promotion_piece: opt!(complete!(san_piece)) >>
            check: opt!(complete!(san_check)) >>
            annotation: opt!(complete!(san_move_annotation)) >>
            (square, promotion, promotion_piece, check, annotation)
        ),
        |(square, promotion, promotion_piece, check, annotation)| {
            let check = if let Some(x) = check { x } else { Check::None };
            let annotation = if let Some(x) = annotation { x } else { MoveAnnotation::None };
            let promotion = match (promotion, promotion_piece) {
                (Some(_), Some(promotion_piece)) => Promotion::PieceType(promotion_piece),
                _ => Promotion::None
            };
            Node::Move(PAWN, Source::None, MoveOrCapture::Move, square, promotion, check, annotation)
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_pawn_capture<Node>, 
    map!(
        do_parse!(
            file: complete!(san_file) >>
            capture: complete!(san_capture) >>
            square: complete!(san_square) >>
            promotion: opt!(complete!(san_promotion)) >>
            promotion_piece: opt!(complete!(san_piece)) >>
            check: opt!(complete!(san_check)) >>
            annotation: opt!(complete!(san_move_annotation)) >>
            (file, capture, square, promotion, promotion_piece, check, annotation)
        ),
        |(file, capture, square, promotion, promotion_piece, check, annotation)| {
            let source = Source::File(file);
            let check = if let Some(x) = check { x } else { Check::None };
            let annotation = if let Some(x) = annotation { x } else { MoveAnnotation::None };
            let promotion = match (promotion, promotion_piece) {
                (Some(_), Some(promotion_piece)) => Promotion::PieceType(promotion_piece),
                _ => Promotion::None
            };
            Node::Move(PAWN, source, capture, square, promotion, check, annotation)
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_move_bare<Node>, 
    map!(
        do_parse!(
            piece: complete!(san_piece) >>
            square: complete!(san_square) >>
            whitespace >>
            (piece, square)
        ),
        |(piece, square)| {
            Node::Move(piece, Source::None, MoveOrCapture::Move, square, Promotion::None, Check::None, MoveAnnotation::None)
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_piece_move<Node>, 
    map!(
        do_parse!(
            piece: complete!(san_piece) >>
            file: opt!(complete!(san_file)) >>
            rank: opt!(complete!(san_rank)) >>
            capture: opt!(complete!(san_capture)) >>
            square: opt!(complete!(san_square)) >>
            check: opt!(complete!(san_check)) >>
            annotation: opt!(complete!(san_move_annotation)) >>
            (piece, file, rank, capture, square, check, annotation)
        ),
        |(piece, file, rank, capture, square, check, annotation)| {
            let capture = if let Some(x) = capture { x } else { MoveOrCapture::Move };
            let check = if let Some(x) = check { x } else { Check::None };
            let annotation = if let Some(x) = annotation { x } else { MoveAnnotation::None };
            match (file, rank, square) {
                (Some(f), Some(r), None) => {
                    Node::Move(piece, Source::None, capture, make_square(f, r), Promotion::None, check, annotation)
                },
                (None, None, Some(square)) => {
                    Node::Move(piece, Source::None, capture, square, Promotion::None, check, annotation)
                },
                (Some(f), None, Some(square)) => {
                    Node::Move(piece, Source::File(f), capture, square, Promotion::None, check, annotation)
                },
                (None, Some(r), Some(square)) => {
                    Node::Move(piece, Source::Rank(r), capture, square, Promotion::None, check, annotation)
                },
                (Some(f), Some(r), Some(square)) => {
                    Node::Move(piece, Source::Square(make_square(f, r)), capture, square, Promotion::None, check, annotation)
                },
                _ => Node::InvalidMove
            }
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_explicit_move<Node>, 
    alt_complete!(
        san_pawn_move_bare |
        san_piece_move_bare |
        san_pawn_capture_bare |
        san_pawn_move |
        san_piece_move |
        san_pawn_capture
    )
);

///-----------------------------------------------------------------------------
named!(pub san_null_move<Node>,
   map!(
        do_parse!(
            alt_complete!(tag!("--") | tag!("Z0") | tag!("z0")) >>
            check: opt!(complete!(san_check)) >>
            annotation: opt!(complete!(san_move_annotation)) >>
            (check, annotation)
        ),
        |(check, annotation)| {
            let check = if let Some(x) = check { x } else { Check::None };
            let annotation = if let Some(x) = annotation { x } else { MoveAnnotation::None };
            Node::NullMove(check, annotation)
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_castle_king_side<Node>,
   map!(
        do_parse!(
            tag!("O-O") >>
            check: opt!(complete!(san_check)) >>
            annotation: opt!(complete!(san_move_annotation)) >>
            (check, annotation)
        ),
        |(check, annotation)| {
            let check = if let Some(x) = check { x } else { Check::None };
            let annotation = if let Some(x) = annotation { x } else { MoveAnnotation::None };
            Node::CastleKingSide(check, annotation)
        }
    )
);

///-----------------------------------------------------------------------------
named!(pub san_castle_queen_side<Node>,
   map!(
        do_parse!(
            tag!("O-O-O") >>
            check: opt!(complete!(san_check)) >>
            annotation: opt!(complete!(san_move_annotation)) >>
            (check, annotation)
        ),
        |(check, annotation)| {
            let check = if let Some(x) = check { x } else { Check::None };
            let annotation = if let Some(x) = annotation { x } else { MoveAnnotation::None };
            Node::CastleQueenSide(check, annotation)
        }
    )
);


///-----------------------------------------------------------------------------
named!(pub san_move<Node>, alt_complete!(
    san_explicit_move |
    san_castle_queen_side |
    san_castle_king_side |
    san_null_move
));

#[cfg(test)]
mod tests {

    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_san_piece() {
        assert_eq!(Done(&b""[..], PAWN), san_piece(&b"P"[..]));
        assert_eq!(Done(&b""[..], KNIGHT), san_piece(&b"N"[..]));
        assert_eq!(Done(&b""[..], BISHOP), san_piece(&b"B"[..]));
        assert_eq!(Done(&b""[..], ROOK), san_piece(&b"R"[..]));
        assert_eq!(Done(&b""[..], QUEEN), san_piece(&b"Q"[..]));
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
        assert_eq!(Done(&b""[..], SQ_E4), san_square(&b"e4"[..]));
    }
    #[test]
    fn test_san_pawn_capture() {
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::File(FILE_B), MoveOrCapture::Capture, SQ_C1, Promotion::PieceType(ROOK), Check::Check, MoveAnnotation::None)), san_pawn_capture(&b"bxc1=R+"[..]));
    }
    #[test]
    fn test_san_move_parsing() {
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::None, MoveOrCapture::Move, SQ_E4, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"e4"[..]));
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::None, MoveOrCapture::Move, SQ_D4, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"d4"[..]));
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::None, MoveOrCapture::Move, SQ_C4, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"c4"[..]));

        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::File(FILE_D), MoveOrCapture::Capture, SQ_E4, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"dxe4"[..]));
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::File(FILE_E), MoveOrCapture::Capture, SQ_D4, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"exd4"[..]));
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::File(FILE_D), MoveOrCapture::Capture, SQ_C4, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"dxc4"[..]));

        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::None, MoveOrCapture::Move, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Nf3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(BISHOP, Source::None, MoveOrCapture::Move, SQ_B5, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Bb5"[..]));
        assert_eq!(Done(&b""[..], Node::Move(QUEEN, Source::None, MoveOrCapture::Move, SQ_D8, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Qd8"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::None, MoveOrCapture::Move, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Rd1"[..]));

        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::None, MoveOrCapture::Capture, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Nxf3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(BISHOP, Source::None, MoveOrCapture::Capture, SQ_B5, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Bxb5"[..]));
        assert_eq!(Done(&b""[..], Node::Move(QUEEN, Source::None, MoveOrCapture::Capture, SQ_D8, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Qxd8"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::None, MoveOrCapture::Capture, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Rxd1"[..]));

        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::File(FILE_E), MoveOrCapture::Move, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Nef3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::File(FILE_E), MoveOrCapture::Move, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Red1"[..]));

        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::File(FILE_E), MoveOrCapture::Capture, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Nexf3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::File(FILE_E), MoveOrCapture::Capture, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Rexd1"[..]));

        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::Rank(RANK_2), MoveOrCapture::Move, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"N2f3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::Rank(RANK_3), MoveOrCapture::Move, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"R3d1"[..]));

        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::Rank(RANK_1), MoveOrCapture::Capture, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"N1xf3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::Rank(RANK_6), MoveOrCapture::Capture, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"R6xd1"[..]));
        
        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::Square(SQ_F1), MoveOrCapture::Move, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Nf1f3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::Square(SQ_D3), MoveOrCapture::Move, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Rd3d1"[..]));

        assert_eq!(Done(&b""[..], Node::Move(KNIGHT, Source::Square(SQ_F1), MoveOrCapture::Capture, SQ_F3, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Nf1xf3"[..]));
        assert_eq!(Done(&b""[..], Node::Move(ROOK, Source::Square(SQ_D3), MoveOrCapture::Capture, SQ_D1, Promotion::None, Check::None, MoveAnnotation::None)), san_move(&b"Rd3xd1"[..]));

        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::None, MoveOrCapture::Move, SQ_E8, Promotion::PieceType(QUEEN), Check::None, MoveAnnotation::None)), san_move(&b"e8=Q"[..]));
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::File(FILE_F), MoveOrCapture::Capture, SQ_E8, Promotion::PieceType(KNIGHT), Check::None, MoveAnnotation::None)), san_move(&b"fxe8=N"[..]));

        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::None, MoveOrCapture::Move, SQ_E8, Promotion::PieceType(QUEEN), Check::Check, MoveAnnotation::None)), san_move(&b"e8=Q+"[..]));
        assert_eq!(Done(&b""[..], Node::Move(PAWN, Source::File(FILE_F), MoveOrCapture::Capture, SQ_E8, Promotion::PieceType(KNIGHT), Check::Checkmate, MoveAnnotation::None)), san_move(&b"fxe8=N#"[..]));

        assert_eq!(Done(&b""[..], Node::Move(QUEEN, Source::Square(SQ_A6), MoveOrCapture::Capture, SQ_B7, Promotion::None, Check::Checkmate, MoveAnnotation::None)), san_move(&b"Qa6xb7#"[..]));

        assert_eq!(Done(&b""[..], Node::Move(QUEEN, Source::Square(SQ_A6), MoveOrCapture::Capture, SQ_B7, Promotion::None, Check::Checkmate, MoveAnnotation::Brilliant)), san_move(&b"Qa6xb7#!!"[..]));

        assert_eq!(Done(&b""[..], Node::CastleKingSide(Check::None, MoveAnnotation::None)), san_move(&b"O-O"[..]));
        assert_eq!(Done(&b""[..], Node::CastleQueenSide(Check::None, MoveAnnotation::None)), san_move(&b"O-O-O"[..]));

        assert_eq!(Done(&b""[..], Node::CastleKingSide(Check::Checkmate, MoveAnnotation::Brilliant)), san_move(&b"O-O#!!"[..]));
        assert_eq!(Done(&b""[..], Node::CastleQueenSide(Check::Checkmate, MoveAnnotation::Brilliant)), san_move(&b"O-O-O#!!"[..]));

        assert_eq!(Done(&b""[..], Node::NullMove(Check::Checkmate, MoveAnnotation::Brilliant)), san_move(&b"--#!!"[..]));
        assert_eq!(Done(&b""[..], Node::NullMove(Check::Checkmate, MoveAnnotation::Brilliant)), san_move(&b"Z0#!!"[..]));
        assert_eq!(Done(&b""[..], Node::NullMove(Check::Checkmate, MoveAnnotation::Brilliant)), san_move(&b"z0#!!"[..]));
    }
}
