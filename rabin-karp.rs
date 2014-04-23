#![crate_id = "Rabin-Karp"]
#![crate_type = "bin"]

extern crate getopts;
use getopts::{reqopt, optflag, getopts, OptGroup};
use std::os;
use std::hash::hash;
use common::utils::array_from_file;

pub mod common { pub mod utils; }

pub fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-i, --input\tInput");
    println!("-k, --key\tKey");
    println!("-h, --help\tUsage");
}

/// Takes an array of strings and successively iterates through each string.
/// On each iteration, it compares the hash of a substring of equal length
/// to the key to the hash of the key, and terminates either when a match
/// is found or if all possibilities are exhausted. Returns a boolean
/// to indicate if it was found, the index (uint) of the array holding
/// the string in which it was found, and the index (uint) of that
/// string at which the matching substring begins.
pub fn rubin_karp(array: Vec<~str>, key: &str) -> (bool, uint, uint) {
    let mut index = 0u;
    let mut found = false;
    let mut str_index = 0u;

    if array.len() < 1 || key.len() < 1 { return (found, index, str_index) }

    let keyhash = hash(&key);
    
    loop {
        if index >= array.len() { break }
        let string_i = array.as_slice()[index].clone();
        str_index = 0;

        loop { 
            let right_index = str_index + key.len();
            if right_index > string_i.len() { break }
            if hash(&string_i.slice(str_index, right_index)) == keyhash {
                found = true
            }
            if found == true { break }
            else { 
                str_index += 1;
            }
        }

        if found == true { break }
        else { index += 1 }
    }

    return (found, index, str_index)
}

fn main() {
    let args = os::args();

    let program = args[0].clone();

    let opts = [
        reqopt("i", "input", "input file aka haystack", "FILENAME"),
        reqopt("k", "key", "key aka needle", "KEY VALUE"),
        optflag("h", "help", "print help menu")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };

    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    }

    let input_filename = match matches.opt_str("i") {
        Some(string)    => string,
        _               => ~"Invalid"
    };
    let array = array_from_file(input_filename);

    let key = match matches.opt_str("k") {
        Some(string)    => string,
        _               => ~"Invalid"
    };

    let (found, i, j) = rubin_karp(array, key);

    println!("{} {} {}", found, i, j);

}
