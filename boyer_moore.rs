#![crate_id = "boyer_moore"]
#![crate_type = "bin"]

//!Boyer Moore string search, simplified. No good suffix rule here.

extern crate getopts;
extern crate collections;
use getopts::{reqopt, optflag,getopts,OptGroup};
use collections::HashMap;
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
    let bad_table = bad_char_table_gen(key);
    let mut match_pos: uint;
    let mut found: bool;
    let mut i = 0u; //i is the index of the string in the array
    let mut keylen = 0u;

    for _ in key.chars() {
        keylen += 1;
    }
    //There's probably a method to do this better.

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
                string_i.slice(match_pos, right_i), key, keylen,
                bad_table.clone());
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
fn reverse_search(stringa: &str,key: &str, keylen: uint,
    table: HashMap<char,uint>) -> (bool, uint) {
    let mut i = keylen; //don't want to mutate the length of the key
    let mut j = 1u; 
    let mut found = false;
    //reverse iterate through  the key and the substring
    for (chara, charb) in key.chars_rev().zip(stringa.chars_rev()) {
        if chara != charb {
            //j gets assigned the appropriate jump value from the table
            j  = match table.find(&charb) {
                Some(jump_value)    => jump_value.clone(),
                _                   => j
            }; 
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

///generate a hashmap mapping a uint value to each char in the key
///depending on how many instances of the char are in the key and
///the location of the char in the key
fn bad_char_table_gen(key: &str) -> HashMap<char, uint> {
    let mut bad_table = HashMap::new();
    let length = key.len();
    let mut i = 0u;
    for eachc in key.chars()  {
        let jump = length - 1 - i;
        if jump > length || jump == 0 {
            bad_table.insert_or_update_with(eachc, 1, |_k, v| *v = 1);
        } else {
            bad_table.insert_or_update_with(eachc, jump, |_k, v| *v = jump);
        }
        i += 1;
    }
    return bad_table;
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

