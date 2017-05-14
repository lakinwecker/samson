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

///-------------------------------------------------------------------------------------------------
/// Parsers for byte-order-markers sometimes found in utf-8/utf-16 files.
///-------------------------------------------------------------------------------------------------

use super::super::types::*;
use nom::*;

///-------------------------------------------------------------------------------------------------
named!(pub utf_8_bom, tag!("\u{feff}"));

#[cfg(test)]
mod tests {

    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_utf_8_bom_efbbbf() {
        assert_eq!(Done(&b""[..], &"\u{feff}".as_bytes()[..]), utf_8_bom(&"\u{feff}".as_bytes()));
        assert_eq!(Done(&b""[..], &[239u8, 187u8, 191u8][..]), utf_8_bom(&"\u{feff}".as_bytes()));
        assert_eq!(Done(&b""[..], &[0xEFu8, 0xBBu8, 0xBFu8][..]), utf_8_bom(&"\u{feff}".as_bytes()));
    }
}
