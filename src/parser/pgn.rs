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
use nom::*;

use std::str;
use std::str::FromStr;

named!(pub string_token, delimited!(char!('"'), escaped!(is_not!("\\\""), '\\', one_of!("\"\\")), char!('"')));
named!(pub string_token_as_string<String>, map_res!(map_res!(string_token, str::from_utf8), String::from_str));
named!(pub integer_token<u64>, map_res!(map_res!(ws!(digit), str::from_utf8), FromStr::from_str));
named!(pub period_token, tag!("."));
named!(pub asterisk_token, tag!("*"));
named!(pub open_bracket_token, tag!("["));
named!(pub close_bracket_token, tag!("]"));
named!(pub nag_token<NumericAnnotationGlyph>,
    map!(preceded!(char!('$'), integer_token), |i| { NumericAnnotationGlyph{num: i} })
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
    fn test_asterisk_token() {
        assert_eq!(Done(&b""[..], &b"*"[..]), asterisk_token(b"*"));
        assert_eq!(Done(&b"ef"[..], &b"*"[..]), asterisk_token(b"*ef"));
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
        assert_eq!(Done(&b""[..], NumericAnnotationGlyph{num: 4u64}), nag_token(b"$4"));
        assert_eq!(Done(&b"ef"[..], NumericAnnotationGlyph{num: 4u64}), nag_token(b"$4ef"));
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
}
