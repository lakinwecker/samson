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

use super::super::types;
use super::super::game;
use nom::*;

named!(pub string_token, delimited!(char!('"'), escaped!(is_not!("\\\""), '\\', one_of!("\"\\")), char!('"')));
named!(pub integer_token, call!(digit));
named!(pub period_token, tag!("."));
named!(pub asterisk_token, tag!("*"));
named!(pub open_bracket_token, tag!("["));
named!(pub close_bracket_token, tag!("]"));
named!(pub nag_token, preceded!(char!('$'), integer_token));
named!(pub symbol, preceded!(char!('$'), integer_token));

#[cfg(test)]
mod tests {

    use super::super::super::*;
    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(Done(&b""[..], &b"aaaaaaa"[..]), string_token(b"\"aaaaaaa\""));
        assert_eq!(Done(&b""[..], &b"aaaaaaa \\\" aaaaaaa"[..]), string_token(b"\"aaaaaaa \\\" aaaaaaa\""));
    }

    #[test]
    fn test_integer_token() {
        assert_eq!(Done(&b""[..], &b"111"[..]), integer_token(b"111"));
        assert_eq!(Done(&b""[..], &b"311"[..]), integer_token(b"311"));
        assert_eq!(Done(&b"ef"[..], &b"111"[..]), integer_token(b"111ef"));
        assert_eq!(Done(&b"ef"[..], &b"311"[..]), integer_token(b"311ef"));
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
        assert_eq!(Done(&b""[..], &b"4"[..]), nag_token(b"$4"));
        assert_eq!(Done(&b"ef"[..], &b"4"[..]), nag_token(b"$4ef"));
    }
}
