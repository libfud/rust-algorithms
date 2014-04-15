#![crate_id = "boyer_moore"]
#![crate_type = "bin"]

//!Boyer Moore string search, simplified. No good suffix rule here.

extern crate getopts;
use getopts::{reqopt, optflag,getopts,OptGroup};
use std::os;
use common::utils::{array_from_file};

pub mod common { pub mod utils; }

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-i ,\t--input\t\t Input file");
    println!("-k ,\t--key\t\t Key value");
}

///Takes an array of strings and a key, returns a boolean to indicate if the
///string was found, the index of the string in which it was found, and the
///index of that string at which it was found.
fn boyer_moore_search(array: &[~str], key: &str) -> (bool, uint, uint) {
    if array.len() < 1 || key.len() < 1 { return (false, 0u, 0u) }

    let mut bad_table = [key.len(), .. 256];
    let mut i = 0;
    for &eachbyte in key.as_bytes().iter() {
        let jump = key.len() - 1 -i;
        if jump > key.len() || jump == 0 {
            bad_table[eachbyte as uint] = 1;
        } else {
            bad_table[eachbyte as uint] = jump;
        }
        i += 1;
    }
    
    let mut match_pos: uint;
    let mut found: bool;
    let mut i = 0u; //i is the index of the string in the array

    loop {
        //match position is the index of the string at which the key is
        //found. Resets to 0 for each new sring in the array.
        match_pos = 0;
        found = false;
        if i >= array.len() { break }
        let string_i = array[i].clone();
        let str_len = string_i.len() - 1; //no out of bounds access

        loop {
            let right_i = match_pos + key.len(); //we're slicing strings
            if right_i  > str_len { break } //no oob
            let (found_inner, jump) = reverse_search(
                string_i.slice(match_pos, right_i), key, bad_table);
            if found_inner == true {
                found = true;
                break;
            }
            match_pos += jump; //advance the index at which we slice
        }

        if found == true { break }
        i += 1;
    }

    return (found, i, match_pos);
}

///Reverse iterates through the two strings; the key and the string
///we're searching. Returns whether or not we found a match (i == 0)
///and an unsigned integer to advance the slicing point.
fn reverse_search(stringa: &str,key: &str, table: [uint, ..256]) -> (bool, uint) {

    let mut i = key.char_len(); //don't want to mutate the length of the key
    let mut j = 1u; 
    let mut found = false;
    //reverse iterate through  the key and the substring
    for (chara, charb) in key.bytes().rev().zip(stringa.bytes().rev()) {
        if chara != charb {
            j = table[charb as uint];
            break;
        }
        i -= 1;
    }
    if i == 0  { found = true }
    //indicates a complete loop through all values in the key and string
    if j < i + 1 { return (found, j) }
    //add one to i because if j got assigned a value then i did not
    //get incremented
    return (found, i + 1) 
    //this happens if the prior comparison was false
}

fn main() {
    let args = os::args();

    let program = args[0].clone();

    let opts = ~[
        reqopt("i", "input", "input file name", "FILENAME"),
        reqopt("k", "key", "key", "KEY VALUE"),
        optflag("h", "help", "print this help message")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => { m }
        Err(f)  => { fail!(f.to_err_msg()) }
    };
    
    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    }

    let input_filename = match matches.opt_str("i") {
        Some(string) => string,
        _            => ~"INVALID"
    };
    let array = array_from_file(input_filename);

    let key = match matches.opt_str("k") {
        Some(string)    => string,
        _               => ~"INVALID"
    };

    let (found, index, true_index) = boyer_moore_search(array, key);

    println!("{} {} {}", found, index, true_index);

}

