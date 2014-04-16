#![crate_id = "naive_stringsearch"]
#![crate_type = "bin"]

//! An implimentation of a naive string search.

extern crate getopts;
use getopts::{reqopt, getopts};
use std::os;
use common::utils::array_from_file;
pub mod common { pub mod utils; }


///find_string takes an owned string for a key and an array of owned strings
/// as the text to be searched, and returns the string's index in the array, 
/// and the index of string at which the first instantiation of the key was
/// found.
fn find_string(keychars: ~str, data: ~[~str]) -> (bool, uint, uint) {
    
    let mut i = 0;
    let mut found = false;
    let mut index: uint = 0;  

    loop {
        if i >= data.len() { break } //don't try OOB access
        let mut datastring = data[i].clone();
        index = 0;
        loop {
            if keychars.len() > datastring.len() { break } //they key can't
                //be in something shorter than itself
            if keychars[0] == datastring[0] {
                found = compare_chars(keychars.clone(), datastring.clone());
            }
            if found == true { break }
            datastring.shift_char();
            index += 1; //to locate where the key starts in the string
        }
        if found == true { break }
        i += 1;
    }

    return (found, i, index);
}

/// Compares two arrays of characters. They do not have to be of equal
/// length. However, if the second string is shorter than the first 
/// it will return false.
fn compare_chars(stringa: ~str, stringb: ~str) -> bool {
    let mut i = 0;
    let mut found = true; //assume true til proven false
    
    loop {
        if stringb.len() < stringa.len() { //a key can't be in something shorter
            //than itself
            found = false;
            break;
        }
        if i == stringa.len() { break }
        if stringa[i] != stringb[i] {
            found = false;
            break;
        }
        i += 1;
    }
    return found;
}
        

fn main() {
    let args = os::args();

    let opts = ~[
        reqopt("i", "input", "input file name", "FILE NAME"),
        reqopt("k", "key", "key", "KEY VALUE"),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => { m },
        Err(f)  => { fail!(f.to_err_msg()) }
    };

    let textpath = match matches.opt_str("i") {
        Some(string)    => string,
        _               => ~"INVALID"
    };
    let textarray = array_from_file(textpath);

    let searchstring = match matches.opt_str("k") {
        Some(string)    => string,
        _               => ~"invalid"
    };

    let (found, textindex, stringindex) = find_string(searchstring, textarray.clone());
    println!("{} {} {}", found, textindex, stringindex);
}
