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
pub enum GameTermination {
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
#[derive(Clone, Debug)]
pub struct Game {
    metadata: Vec<Tag>,
    nodes: Vec<Node>,
    termination: GameTermination
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

named!(pub parse_termination<GameTermination>,
    alt_complete!(
        map!(ws!(tag!("1-0")), |_| { GameTermination::WhiteWin }) |
        map!(ws!(tag!("0-1")), |_| { GameTermination::BlackWin }) |
        map!(ws!(tag!("1/2-1/2")), |_| { GameTermination::Draw }) |
        map!(ws!(tag!("*")), |_| { GameTermination::Other })
    )
);

named!(pub parse_node<Node>,
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
named!(pub node_list<Vec<Node> >, many1!(parse_node));

/*
named!(pub parse_game<Game>,
    map!(
        do_parse!(
            tags: tag_list,
            nodes: nodes_list,
        ),
        || {
        }
    )
);
*/

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
    fn test_game_termination() {
        assert_eq!(Done(&b""[..], GameTermination::WhiteWin), parse_termination(b"1-0"));
        assert_eq!(Done(&b""[..], GameTermination::BlackWin), parse_termination(b"0-1"));
        assert_eq!(Done(&b""[..], GameTermination::Draw), parse_termination(b"1/2-1/2"));
        assert_eq!(Done(&b""[..], GameTermination::Other), parse_termination(b"*"));
    }
    #[test]
    fn test_parse_node() {
        assert_eq!(Done(&b""[..], Node::StartVariation), parse_node(b"("));
        assert_eq!(Done(&b""[..], Node::EndVariation), parse_node(b")"));
        assert_eq!(Done(&b""[..], Node::Nag(NumericAnnotationGlyph(1))), parse_node(b"$1"));
        assert_eq!(Done(&b""[..], Node::MoveNumber(1, Periods::None)), parse_node(b"1"));
        assert_eq!(Done(&b""[..], Node::MoveNumber(2, Periods::One)), parse_node(b"2."));
        assert_eq!(Done(&b""[..], Node::MoveNumber(3, Periods::Other)), parse_node(b"3...."));
        assert_eq!(Done(&b""[..], Node::MoveNumber(4, Periods::Three)), parse_node(b"4..."));
        assert_eq!(
            Done(&b""[..], Node::Comment(String::from_str("this is a comment").unwrap())),
            parse_node(b"{this is a comment}")
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
            parse_node(&b"Nxf3"[..])
        );
    }
    #[test]
    fn test_node_list() {
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
            node_list(&b"( {comment} 1...Nxf3 $3 )"[..])
        );
    }
}
