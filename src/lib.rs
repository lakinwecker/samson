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
// A pure rust chess library.
//
// Shamelessly patterned after the amazing Stockfish
//------------------------------------------------------------------------------

#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate regex;

#[macro_use]
extern crate nom;

pub mod types;
pub mod parser;

