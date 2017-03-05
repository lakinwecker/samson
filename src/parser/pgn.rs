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
// Parsers for the PGN import specification.
//------------------------------------------------------------------------------

use super::super::types::*;
use nom::*;
use parser::san;

use std::str;
use std::str::FromStr;

///-----------------------------------------------------------------------------
/// There are 7 tags that must be present with each game:
/// 1. Event
/// 2. Site
/// 3. Date
/// 4. Round
/// 5. White
/// 6. Black
/// 7. Result

///-----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub enum Tag<'a> {
    Event(&'a [u8]),
    Site(&'a [u8]),
    Date(&'a [u8]),
    Round(&'a [u8]),
    White(&'a [u8]),
    Black(&'a [u8]),
    Result(&'a [u8]),
    Other(&'a [u8], &'a [u8])
}

///-----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct NumericAnnotationGlyph(pub u64);

///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Periods {
    None,
    One,
    Three,
    Other
}

///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Result {
    WhiteWin,
    BlackWin,
    Draw,
    Other
}

///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Node<'a> {
    EscapeComment(&'a [u8]),
    Comment(&'a [u8]),
    Nag(NumericAnnotationGlyph),
    MoveNumber(u64, Periods),
    Move(san::Node),
    StartVariation,
    EndVariation,
    Variation(Vec<Node<'a>>)

}

///-----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Game<'a> {
    pub tags: Vec<Tag<'a>>,
    pub nodes: Vec<Node<'a>>,
    pub result: Result
}

named!(pub string_token, delimited!(char!('"'), escaped!(is_not!("\\\""), '\\', one_of!("\"\\")), char!('"')));
named!(pub integer_token<u64>, map_res!(map_res!(ws!(digit), str::from_utf8), FromStr::from_str));
named!(pub period_token, tag!("."));
named!(pub open_bracket_token, tag!("["));
named!(pub close_bracket_token, tag!("]"));
named!(pub open_parenthesis_token, tag!("("));
named!(pub close_parenthesis_token, tag!(")"));
named!(pub escape_comment, preceded!(tag!("%"), is_not!("\n")));
named!(pub nag_token<NumericAnnotationGlyph>,
    map!(preceded!(char!('$'), integer_token), |i| { NumericAnnotationGlyph(i) })
);
named!(pub symbol_token, re_bytes_find!(r"[[:alnum:]]{1}[0-9A-Za-z#=:+_-]*"));
// TODO: this is verbose and gross, but I can't figure out a better way to do it
// without the help of the internet.
named!(pub event_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            ws!(tag!("Event")) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (value)
        ),
        |value|{ Tag::Event(value) }
    )
);
named!(pub site_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            ws!(tag!("Site")) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (value)
        ),
        |value|{ Tag::Site(value) }
    )
);
named!(pub date_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            ws!(tag!("Date")) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (value)
        ),
        |value|{ Tag::Date(value) }
    )
);
named!(pub round_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            ws!(tag!("Round")) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (value)
        ),
        |value|{ Tag::Round(value) }
    )
);
named!(pub white_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            ws!(tag!("White")) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (value)
        ),
        |value|{ Tag::White(value) }
    )
);
named!(pub black_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            ws!(tag!("Black")) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (value)
        ),
        |value|{ Tag::Black(value) }
    )
);
named!(pub result_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            ws!(tag!("Result")) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (value)
        ),
        |value|{ Tag::Result(value) }
    )
);
named!(pub other_tag<Tag>, 
    map!(
        do_parse!(
            ws!(open_bracket_token) >>
            key: ws!(symbol_token) >>
            value: ws!(string_token) >>
            ws!(close_bracket_token) >>
            (key, value)
        ),
        |(key, value)| { Tag::Other(key, value) }
    )
);
named!(pub tag_pair<Tag>, 
    alt_complete!(event_tag | site_tag | date_tag | round_tag | white_tag | black_tag | result_tag | other_tag)
);
named!(pub tag_list<Vec<Tag> >, many0!(ws!(tag_pair)));
named!(pub commentary_token, delimited!(char!('{'), is_not!("}"), char!('}')));

named!(pub game_result<Result>,
    map!(
        alt_complete!(
            ws!(tag!("*")) |
            ws!(tag!("0-1")) |
            ws!(tag!("1/2-1/2")) |
            ws!(tag!("1-0"))
        ),
        |value: &[u8]| {
            println!("{:?}", value);
            match value {
                b"1-0" => Result::WhiteWin,
                b"0-1" => Result::BlackWin,
                b"1/2-1/2" => Result::Draw,
                _ => Result::Other
            }
        }
    )
);

named!(pub game_node<Node>,
    alt_complete!(
        map!(ws!(open_parenthesis_token), |_| { Node::StartVariation }) |
        map!(ws!(close_parenthesis_token), |_| { Node::EndVariation }) |
        map!(ws!(nag_token), |n| { Node::Nag(n) }) |
        map!(ws!(commentary_token), |c| { Node::Comment(c) }) |
        map!(
            do_parse!(
                num: ws!(complete!(integer_token)) >>
                periods: opt!(ws!(complete!(many0!(period_token)))) >>
                (num, periods)
            ),
            |(num, periods): (u64, Option<Vec<&[u8]> >)| {
                Node::MoveNumber(
                    num,
                    match periods {
                        Some(x) => {
                            match x.len() {
                                1 => Periods::One,
                                3 => Periods::Three,
                                0 => Periods:: None,
                                _ => Periods::Other
                            }
                        },
                        _ => Periods::None,
                    }
                )
            }
        ) |
        map!(ws!(complete!(san::san_move)), |x| { Node::Move(x) })
    )
);
named!(pub game_node_list<Vec<Node> >, many1!(game_node));

// TODO: find a more elegant way to deal with the silly escape comments.
//       Q: Why does pgn have such ambiguous rules. So can an escape comment
//       appear in the middle of a tag list or set of moves/commentary?
//       What about in a commentary itself? 
//       A: Tide goes in, Tide goes out. You can't explain that.
//
//       Also, we are ignoring the escape comments for now. 
named!(pub game<Game>,
    map!(
        do_parse!(
            many0!(escape_comment) >>
            tags: ws!(tag_list) >>
            many0!(escape_comment) >>
            nodes: ws!(game_node_list) >>
            result: ws!(game_result) >>
            (tags, nodes, result)
        ),
        |(tags, nodes, result)| { Game{tags: tags, nodes:nodes, result: result} }
    )
);
named!(pub pgn<Vec<Game> >, many0!(game));

#[cfg(test)]
mod tests {

    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(Done(&b""[..], &b"aaaaaaa"[..]), string_token(b"\"aaaaaaa\""));
        assert_eq!(Done(&b""[..], &b"aaaaaaa \\\" aaaaaaa"[..]), string_token(b"\"aaaaaaa \\\" aaaaaaa\""));
    }
    #[test]
    fn test_integer_token() {
        assert_eq!(Done(&b""[..], 111u64), integer_token(b"111"));
        assert_eq!(Done(&b""[..], 311u64), integer_token(b"311"));
        assert_eq!(Done(&b"ef"[..], 111u64), integer_token(b"111ef"));
        assert_eq!(Done(&b"ef"[..], 311u64), integer_token(b"311ef"));
    }

    #[test]
    fn test_period_token() {
        assert_eq!(Done(&b""[..], &b"."[..]), period_token(b"."));
        assert_eq!(Done(&b"ef"[..], &b"."[..]), period_token(b".ef"));
    }

    #[test]
    fn test_open_bracket_token() {
        assert_eq!(Done(&b""[..], &b"["[..]), open_bracket_token(b"["));
        assert_eq!(Done(&b"ef"[..], &b"["[..]), open_bracket_token(b"[ef"));
    }
    #[test]
    fn test_close_bracket_token() {
        assert_eq!(Done(&b""[..], &b"]"[..]), close_bracket_token(b"]"));
        assert_eq!(Done(&b"ef"[..], &b"]"[..]), close_bracket_token(b"]ef"));
    }
    #[test]
    fn test_nag_token() {
        assert_eq!(Done(&b""[..], NumericAnnotationGlyph(4u64)), nag_token(b"$4"));
        assert_eq!(Done(&b"ef"[..], NumericAnnotationGlyph(4u64)), nag_token(b"$4ef"));
    }
    #[test]
    fn test_symbol_token() {
        assert_eq!(Done(&b""[..], &b"sasd#_+#=:-"[..]), symbol_token(b"sasd#_+#=:-"));
        assert_eq!(Done(&b"!()~{}[]"[..], &b"sasd#_+#=:-"[..]), symbol_token(b"sasd#_+#=:-!()~{}[]"));
    }
    #[test]
    fn test_tag_pair() {
        assert_eq!(Done(&b""[..], Tag::Event(&b"?"[..])), tag_pair(b"[Event \"?\"]"));
        assert_eq!(Done(&b""[..], Tag::Event(&b"Tony Rotella"[..])), tag_pair(b"[Event \"Tony Rotella\"]"));
    }
    #[test]
    fn test_tag_list() {
        assert_eq!(
            Done(&b""[..], vec![Tag::Event(&b"Tony Rotella"[..]), Tag::Date(&b"2017.01.01"[..])]),
            tag_list(b"[Event \"Tony Rotella\"]\n[Date \"2017.01.01\"]")
        );
    }
    #[test]
    fn test_commentary() {
        assert_eq!(Done(&b""[..], &b"this is a comment"[..]), commentary_token(b"{this is a comment}"));
        assert_eq!(Done(&b""[..], &b"this is a\n comment"[..]), commentary_token(b"{this is a\n comment}"));
    }
    #[test]
    fn test_game_result() {
        assert_eq!(Done(&b""[..], Result::WhiteWin), game_result(b"1-0"));
        assert_eq!(Done(&b""[..], Result::BlackWin), game_result(b"0-1"));
        assert_eq!(Done(&b""[..], Result::Draw), game_result(b"1/2-1/2"));
        assert_eq!(Done(&b""[..], Result::Other), game_result(b"*"));

        assert_eq!(Done(&b""[..], Result::WhiteWin), game_result(b" 1-0 "));
        assert_eq!(Done(&b""[..], Result::BlackWin), game_result(b" 0-1 "));
        assert_eq!(Done(&b""[..], Result::Draw), game_result(b" 1/2-1/2 "));
        assert_eq!(Done(&b""[..], Result::Other), game_result(b" * "));
    }
    #[test]
    fn test_game_node() {
        assert_eq!(Done(&b""[..], Node::StartVariation), game_node(b"("));
        assert_eq!(Done(&b""[..], Node::EndVariation), game_node(b")"));
        assert_eq!(Done(&b""[..], Node::Nag(NumericAnnotationGlyph(1))), game_node(b"$1"));
        assert_eq!(Done(&b""[..], Node::MoveNumber(1, Periods::None)), game_node(b"1"));
        assert_eq!(Done(&b""[..], Node::MoveNumber(2, Periods::One)), game_node(b"2."));
        assert_eq!(Done(&b""[..], Node::MoveNumber(3, Periods::Other)), game_node(b"3...."));
        assert_eq!(Done(&b""[..], Node::MoveNumber(4, Periods::Three)), game_node(b"4..."));
        assert_eq!(
            Done(&b""[..], Node::Comment(&b"this is a comment"[..])),
            game_node(b"{this is a comment}")
        );
        assert_eq!(
            Done(&b""[..], Node::Move(san::Node::Move(
                        KNIGHT,
                        san::Source::None,
                        san::MoveOrCapture::Capture, SQ_F3,
                        san::Promotion::None,
                        san::Check::None,
                        san::MoveAnnotation::None
                    )
                )
            ),
            game_node(&b"Nxf3"[..])
        );
    }
    #[test]
    fn test_game_node_list() {
        let nxf3 = san::Node::Move(
            KNIGHT,
            san::Source::None,
            san::MoveOrCapture::Capture, SQ_F3,
            san::Promotion::None,
            san::Check::None,
            san::MoveAnnotation::None
        );
        assert_eq!(
            Done(&b""[..], 
                vec![
                    Node::StartVariation,
                    Node::Comment(&b"comment"[..]),
                    Node::MoveNumber(1, Periods::Three),
                    Node::Move(nxf3),
                    Node::Nag(NumericAnnotationGlyph(3)),
                    Node::EndVariation,
                ]
            ),
            game_node_list(&b"( {comment} 1...Nxf3 $3 )"[..])
        );
    }
    #[test]
    fn test_game() {
        let e4 = san::Node::Move(
            PAWN,
            san::Source::None,
            san::MoveOrCapture::Move,
            SQ_E4,
            san::Promotion::None,
            san::Check::None,
            san::MoveAnnotation::None
        );
        let c5 = san::Node::Move(
            PAWN,
            san::Source::None,
            san::MoveOrCapture::Move,
            SQ_C5,
            san::Promotion::None,
            san::Check::None,
            san::MoveAnnotation::None
        );
        let result = game(&b"% BOOKTITLE = The Killer Sicilian: Fighting 1 e4 with the Kalashnikov
[Event \"?\"]
[Site \"?\"]
[Date \"????.??.??\"]
[Round \"?\"]
[White \"About this Publication\"]
[Black \"?\"]
[Result \"*\"]
[Annotator \"Tony Rotella\"]
[PlyCount \"2\"]
[SourceDate \"2015.03.02\"]

% This should be ignored for now

{Are you searching for a new weapon against 1 e4? Look no further - choose the
Killer Sicilian! --- In this book, opening expert Tony Rotella presents a
Sicilian repertoire for Black, the backbone of which consists of the
Kalashnikov Variation. The Kalashnikov is an ideal choice for those looking to
take up the Sicilian. Black follows an easy-to-learn system of development,
with clear strategic aims. What's more, in many lines Black can choose between
aggressive and positional options. It's no coincidence that the Kalashnikov
has attracted such attacking talents as World Championship candidate Teimour
Radjabov and multi-time US Champion Alexander Shabalov. --- Rotella critically
examines the main lines and lucidly explains the key positional and tactical
ideas for both sides. He also shows what Black should do against White's
various Anti-Sicilian options. Read this book and unleash the Killer Sicilian!
} 1. e4 c5 {. Tony Rotella is an experienced correspondence player, teacher,
analyst and openings theoretician, from Ohio, USA.} *
"[..]);
        match result {
            Done(_, game) => {

                assert_eq!(game.tags[0], Tag::Event(&b"?"[..]));
                assert_eq!(game.tags[1], Tag::Site(&b"?"[..]));
                assert_eq!(game.tags[2], Tag::Date(&b"????.??.??"[..]));
                assert_eq!(game.tags[3], Tag::Round(&b"?"[..]));
                assert_eq!(game.tags[4], Tag::White(&b"About this Publication"[..]));
                assert_eq!(game.tags[5], Tag::Black(&b"?"[..]));
                assert_eq!(game.tags[6], Tag::Result(&b"*"[..]));
                assert_eq!(game.tags[7], Tag::Other(&b"Annotator"[..], &b"Tony Rotella"[..]));
                assert_eq!(game.tags[8], Tag::Other(&b"PlyCount"[..], &b"2"[..]));
                assert_eq!(game.tags[9], Tag::Other(&b"SourceDate"[..], &b"2015.03.02"[..]));
                assert_eq!(
                    game.nodes[0],
                    Node::Comment(&b"\
Are you searching for a new weapon against 1 e4? Look no further - choose the
Killer Sicilian! --- In this book, opening expert Tony Rotella presents a
Sicilian repertoire for Black, the backbone of which consists of the
Kalashnikov Variation. The Kalashnikov is an ideal choice for those looking to
take up the Sicilian. Black follows an easy-to-learn system of development,
with clear strategic aims. What's more, in many lines Black can choose between
aggressive and positional options. It's no coincidence that the Kalashnikov
has attracted such attacking talents as World Championship candidate Teimour
Radjabov and multi-time US Champion Alexander Shabalov. --- Rotella critically
examines the main lines and lucidly explains the key positional and tactical
ideas for both sides. He also shows what Black should do against White's
various Anti-Sicilian options. Read this book and unleash the Killer Sicilian!
"[..])
                );
                assert_eq!(
                    game.nodes[1],
                    Node::MoveNumber(1, Periods::One)
                );
                assert_eq!(
                    game.nodes[2],
                    Node::Move(e4)
                );
                assert_eq!(
                    game.nodes[3],
                    Node::Move(c5)
                );
                assert_eq!(
                    game.nodes[4],
                    Node::Comment(&b"\
. Tony Rotella is an experienced correspondence player, teacher,
analyst and openings theoretician, from Ohio, USA."[..])
                );
                assert_eq!(game.result, Result::Other);

            },
            _ => assert!(false, "Unable to parse PGN from valid PGN"),
        }
    }

    #[test]
    fn test_game_2() {
        let result = game(&b"[Event \"London\"]
[Site \"?\"]
[Date \"1834.??.??\"]
[Round \"?\"]
[White \"McDonnell, A.\"]
[Black \"De La Bourdonnais, L.\"]
[Result \"0-1\"]
[ECO \"B32\"]
[Annotator \"Tony Rotella\"]
[PlyCount \"74\"]
[EventDate \"1834.??.??\"]
[Source \"Everyman Chess\"]
[SourceDate \"2015.02.28\"]

1. e4 c5 2. Nf3 Nc6 3. d4 cxd4 4. Nxd4 e5 5. Nxc6 bxc6 6. Bc4 Nf6 7. Bg5 Be7 8.
Qe2 d5 9. Bxf6 Bxf6 10. Bb3 O-O 11. O-O a5 12. exd5 cxd5 13. Rd1 d4 14. c4 Qb6
15. Bc2 Bb7 16. Nd2 Rae8 17. Ne4 Bd8 18. c5 Qc6 19. f3 Be7 20. Rac1 f5 21. Qc4+
Kh8 22. Ba4 Qh6 23. Bxe8 fxe4 24. c6 exf3 25. Rc2 Qe3+ 26. Kh1 Bc8 27. Bd7 f2
28. Rf1 d3 29. Rc3 Bxd7 30. cxd7 e4 31. Qc8 Bd8 32. Qc4 Qe1 33. Rc1 d2 34. Qc5
Rg8 35. Rd1 e3 36. Qc3 Qxd1 37. Rxd1 e2 {. One of the most fantastic positions
in all of chess history, produced (roughly, as White's 5th isn't exactly the
main focus of the work you just purchased) by the very subject of this book by
two of the best players in the world over a century and a half ago.} 1-0
"[..]);
        match result {
            Done(_, game) => {

                assert_eq!(game.tags[0], Tag::Event(&b"London"[..]));
                assert_eq!(game.tags[1], Tag::Site(&b"?"[..]));
                assert_eq!(game.tags[2], Tag::Date(&b"1834.??.??"[..]));
                assert_eq!(game.tags[3], Tag::Round(&b"?"[..]));
                assert_eq!(game.tags[4], Tag::White(&b"McDonnell, A."[..]));
                assert_eq!(game.tags[5], Tag::Black(&b"De La Bourdonnais, L."[..]));
                assert_eq!(game.tags[6], Tag::Result(&b"0-1"[..]));
                assert_eq!(game.tags[7], Tag::Other(&b"ECO"[..], &b"B32"[..]));
                assert_eq!(game.tags[8], Tag::Other(&b"Annotator"[..], &b"Tony Rotella"[..]));
                assert_eq!(game.tags[9], Tag::Other(&b"PlyCount"[..], &b"74"[..]));
                assert_eq!(game.tags[10], Tag::Other(&b"EventDate"[..], &b"1834.??.??"[..]));
                assert_eq!(game.tags[11], Tag::Other(&b"Source"[..], &b"Everyman Chess"[..]));
                assert_eq!(game.tags[12], Tag::Other(&b"SourceDate"[..], &b"2015.02.28"[..]));
            },
            Error(e) => {
                println!("Error!: {:?}", e);
                assert!(false);
            },
            Incomplete(_) => {
                println!("Incomplete!");
                assert!(false);
            }
        }
    }
}
