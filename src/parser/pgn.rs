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

use super::super::game::*;
use super::super::types::*;
use nom::*;
use parser::san;

use std::str;
use std::str::FromStr;

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
pub enum Node {
    Comment(String),
    Nag(NumericAnnotationGlyph),
    MoveNumber(u64, Periods),
    Move(san::Node),
    StartVariation,
    EndVariation

}

///-----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub tags: Vec<Tag>,
    pub nodes: Vec<Node>,
    pub result: Result
}

named!(pub string_token, delimited!(char!('"'), escaped!(is_not!("\\\""), '\\', one_of!("\"\\")), char!('"')));
named!(pub string_token_as_string<String>, map_res!(map_res!(string_token, str::from_utf8), String::from_str));
named!(pub integer_token<u64>, map_res!(map_res!(ws!(digit), str::from_utf8), FromStr::from_str));
named!(pub period_token, tag!("."));
named!(pub open_bracket_token, tag!("["));
named!(pub close_bracket_token, tag!("]"));
named!(pub open_parenthesis_token, tag!("("));
named!(pub close_parenthesis_token, tag!(")"));
named!(pub nag_token<NumericAnnotationGlyph>,
    map!(preceded!(char!('$'), integer_token), |i| { NumericAnnotationGlyph(i) })
);
named!(pub symbol_token, re_bytes_find!(r"[[:alnum:]]{1}[0-9A-Za-z#=:+_-]*"));
named!(pub symbol_token_as_string<String>, map_res!(map_res!(symbol_token, str::from_utf8), String::from_str));
named!(pub tag_pair<&[u8], Tag>, do_parse!(
    ws!(open_bracket_token) >>
    tag_key: ws!(symbol_token_as_string) >>
    tag_value: ws!(string_token_as_string) >>
    ws!(close_bracket_token) >>
    (Tag{key: TagKey(tag_key), value: TagValue(tag_value)})
));
named!(pub tag_list<&[u8], Vec<Tag> >, many0!(ws!(complete!(tag_pair))));
named!(pub commentary_token, delimited!(char!('{'), is_not!("}"), char!('}')));

named!(pub game_result<Result>,
    alt_complete!(
        map!(ws!(tag!("1-0")), |_| { Result::WhiteWin }) |
        map!(ws!(tag!("0-1")), |_| { Result::BlackWin }) |
        map!(ws!(tag!("1/2-1/2")), |_| { Result::Draw }) |
        map!(ws!(tag!("*")), |_| { Result::Other })
    )
);

named!(pub game_node<Node>,
    alt_complete!(
        map!(ws!(open_parenthesis_token), |_| { Node::StartVariation }) |
        map!(ws!(close_parenthesis_token), |_| { Node::EndVariation }) |
        map!(ws!(nag_token), |x| { Node::Nag(x) }) |
        map!(ws!(commentary_token), |comment| {
            Node::Comment(String::from_str(str::from_utf8(comment).unwrap_or("")).unwrap_or(String::new()))
        }) |
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

named!(pub game<Game>,
    map!(
        do_parse!(
            tags: tag_list >>
            nodes: game_node_list >>
            result: game_result >>
            (tags, nodes, result)
        ),
        |(tags, nodes, result)| { Game{tags: tags, nodes:nodes, result: result} }
    )
);

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
    fn test_parse_string_as_string() {
        assert_eq!(Done(&b""[..], String::from_str("aaaaaaa").unwrap()), string_token_as_string(b"\"aaaaaaa\""));
        assert_eq!(Done(&b""[..], String::from_str("aaaaaaa \\\" aaaaaaa").unwrap()), string_token_as_string(b"\"aaaaaaa \\\" aaaaaaa\""));
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
    fn test_symbol_token_as_string() {
        assert_eq!(Done(&b""[..], String::from("sasd#_+#=:-")), symbol_token_as_string(b"sasd#_+#=:-"));
        assert_eq!(Done(&b"!()~{}[]"[..], String::from("sasd#_+#=:-")), symbol_token_as_string(b"sasd#_+#=:-!()~{}[]"));
    }
    #[test]
    fn test_tag_pair() {
        assert_eq!(Done(&b""[..], Tag{key: TagKey(String::from("Event")), value: TagValue(String::from("?"))}), tag_pair(b"[Event \"?\"]"));
        assert_eq!(Done(&b""[..], Tag{key: TagKey(String::from("Event")), value: TagValue(String::from("Tony Rotella"))}), tag_pair(b"[Event \"Tony Rotella\"]"));
    }
    #[test]
    fn test_tag_list() {
        assert_eq!(
            Done(&b""[..], 
                vec!{
                    Tag{key: TagKey(String::from("Event")), value: TagValue(String::from("Tony Rotella"))},
                    Tag{key: TagKey(String::from("Date")), value: TagValue(String::from("2017.01.01"))},
                }
            ),
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
            Done(&b""[..], Node::Comment(String::from_str("this is a comment").unwrap())),
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
                    Node::Comment(String::from_str("comment").unwrap()),
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
        let result = game(&b"[Event \"?\"]
[Site \"?\"]
[Date \"????.??.??\"]
[Round \"?\"]
[White \"About this Publication\"]
[Black \"?\"]
[Result \"*\"]
[Annotator \"Tony Rotella\"]
[PlyCount \"2\"]
[SourceDate \"2015.03.02\"]

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

                assert_eq!(
                    game.tags[0],
                    Tag{key: TagKey(String::from_str("Event").unwrap()), value: TagValue(String::from_str("?").unwrap())}
                );
                assert_eq!(
                    game.tags[1],
                    Tag{key: TagKey(String::from_str("Site").unwrap()), value: TagValue(String::from_str("?").unwrap())}
                );
                assert_eq!(
                    game.tags[2],
                    Tag{key: TagKey(String::from_str("Date").unwrap()), value: TagValue(String::from_str("????.??.??").unwrap())}
                );
                assert_eq!(
                    game.tags[3],
                    Tag{key: TagKey(String::from_str("Round").unwrap()), value: TagValue(String::from_str("?").unwrap())}
                );
                assert_eq!(
                    game.tags[4],
                    Tag{key: TagKey(String::from_str("White").unwrap()), value: TagValue(String::from_str("About this Publication").unwrap())}
                );
                assert_eq!(
                    game.tags[5],
                    Tag{key: TagKey(String::from_str("Black").unwrap()), value: TagValue(String::from_str("?").unwrap())}
                );
                assert_eq!(
                    game.tags[6],
                    Tag{key: TagKey(String::from_str("Result").unwrap()), value: TagValue(String::from_str("*").unwrap())}
                );
                assert_eq!(
                    game.tags[7],
                    Tag{key: TagKey(String::from_str("Annotator").unwrap()), value: TagValue(String::from_str("Tony Rotella").unwrap())}
                );
                assert_eq!(
                    game.tags[8],
                    Tag{key: TagKey(String::from_str("PlyCount").unwrap()), value: TagValue(String::from_str("2").unwrap())}
                );
                assert_eq!(
                    game.tags[9],
                    Tag{key: TagKey(String::from_str("SourceDate").unwrap()), value: TagValue(String::from_str("2015.03.02").unwrap())}
                );
                assert_eq!(
                    game.nodes[0],
                    Node::Comment(String::from_str("\
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
").unwrap())
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
                    Node::Comment(String::from_str("\
. Tony Rotella is an experienced correspondence player, teacher,
analyst and openings theoretician, from Ohio, USA.").unwrap())
                );
                assert_eq!(game.result, Result::Other);

            },
            _ => assert!(false, "Unable to parse PGN from valid PGN"),
        }
    }
}
