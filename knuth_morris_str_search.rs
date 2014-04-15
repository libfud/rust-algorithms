#![crate_id = "kmp_stringsearch"]
#![crate_type = "bin"]

//! An implementation of the Knuth-Morris-Pratt string search algorithm.
//! I think. I didn't read it too carefully, so this might not actually capture
//! the spirit of it, but it works, and it's faster than naive string search.

extern crate getopts;
use getopts::{reqopt, getopts};
use std::os;
use common::utils::array_from_file;

pub mod common { pub mod utils; }

///Takes an array of owned strings and a string, and returns the
///index of the string holding the key and the index at which the
///key starts.
fn kmp_find_string(keychars: &str, data: ~[~str]) -> (bool, uint, uint) {

    let mut i = 0;
    let mut found = false;
    let mut true_index: uint = 0;
    let mut index: uint;

    loop {
        if i >= data.len() { break }  // don't go out of bounds
        let mut datastring = data[i].clone();
        true_index = 0; //resets to 0 for the beginning of the string
        loop {
            if keychars.len() > datastring.len() { break }
            //key can't be in something shorter than it
            index = compare_chars(keychars.clone(),datastring.clone());
            if index == keychars.len() { found = true }
            //ie, it iterated successfully through both strings and the key
            //exists in the string
            //need this to return
            if found == true { break }
            if index > 2 {
                datastring = datastring.slice_from(index - 2).to_owned();
                true_index += index - 2;
            }
            //this only executes if the key was not in the string;
            //it cuts off the string up to one character prior to the first
            //nonmatching character. I probably have wrong arithmatic here, but
            //this errs on the side of caution.
            else { 
                datastring.shift_char();
                true_index += 1;
            }
            //this happens if there was an immediate mismatch (index = 0)
        }
        if found == true { break }
        i += 1;
        //advance to the next string
    }

    return (found, i, true_index);
}

///Compare two arrays of characters. Returns the index at which it stops
///comparing them, which will always be equal to or less than the length
///of the first string.
fn compare_chars(stringa: &str, stringb: &str) ->  uint {
    let mut i = 0;

    loop {
        if stringb.len() < stringa.len() {
            break;
        }
        if i == stringa.len() { break }
        if stringa[i] != stringb[i] {
            break;
        }
        i += 1;
    }
    return  i;
}

fn main() {
    let args = os::args();

    if args.len() < 3 {
        print!("I need two filenames, one for the text to be searched, and ");
        println!("one for the key, in that order.");
        return;
    }

    let opts = ~[
        reqopt("i", "input", "input file name", "FILENAME"),
        reqopt("k", "key", "key", "KEY VALUE"),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => { m }
        Err(f)  => { fail!(f.to_err_msg()) }
    };

    let input_filename = match matches.opt_str("i") {
        Some(string) => string,
        _            => ~"Invalid"
    }; 
    let textarray = array_from_file(input_filename);

    let searchstring = match matches.opt_str("k") {
        Some(string)    => string,
        _               => ~"invalid"
    };
    
    let (found, textindex, stringindex) = kmp_find_string(searchstring, textarray.clone());

    if textindex >= textarray.len() {
        println!("Not found.");
        return;
    } else {
        println!("{} {} {}",found,  textindex, stringindex);
    }
}

