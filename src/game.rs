// samson - An engine focused on teaching humans.
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
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// A game representation
#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub key: String,
    pub value: String
}
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub tags: Vec<Tag>
}
#[derive(Clone, Debug, PartialEq)]
pub struct NumericAnnotationGlyph {
    pub num: u64
}
