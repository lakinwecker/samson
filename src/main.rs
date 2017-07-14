extern crate encoding;
extern crate samson;
extern crate nom;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::vec::Vec;
use std::str;

use samson::parser::pgn::*;
use samson::parser::san::*;
use nom::IResult::*;
use nom::Slice;

use encoding::{Encoding, DecoderTrap};
use encoding::all::{ISO_8859_1, UTF_8};

fn main() {

	//let mut f = File::open("/home/lakin/Personal-Repos/samson/KillerSicilian.pgn").unwrap();
	let mut f = File::open("/home/lakin/Personal-Repos/samson/ORNimzoandBogo.pgn").unwrap();
	//let mut f = File::open("/home/lakin/Personal-Repos/samson/simple.pgn").unwrap();
	//let mut f = File::open("/home/lakin/Downloads/160118 to 170513 Lichess Update.pgn").unwrap();
	//let mut f = File::open("/home/lakin/Personal-Repos/samson/test.pgn").unwrap();
    let mut bom = [0u8; 3];
    let mut is_utf_8 = false;
    f.read_exact(&mut bom).and_then(|_| { 
        if bom == [239u8, 187u8, 191u8] {
            is_utf_8 = true;
        } else {
            is_utf_8 = false;
        }
        Ok(())
    }).unwrap();
    if !is_utf_8 {
        f.seek(SeekFrom::Start(0));
    }
	let mut buf = Vec::with_capacity(1024*1024*128);
    f.read_to_end(&mut buf).unwrap();
    let decoded;
    if !is_utf_8 {
        decoded = ISO_8859_1.decode(&mut buf, DecoderTrap::Strict).unwrap();
    } else {
        decoded = UTF_8.decode(&mut buf, DecoderTrap::Strict).unwrap();
    }
    let bytes = decoded.as_bytes();
    let games = pgn(bytes);
    match games {
        Done(left, games) => {
            println!("Read {:?} games.", games.len());
            println!("Left {:?} bytes.", left.len());
        },
        Error(e) =>  {
            println!("Error!: {:?}", e);
            println!("Error reading games");
        }
        Incomplete(_) => {
            println!("Incomplete!");
            println!("Error reading games");
        }
    }
}
