#![crate_id = "boyer_moore"]
#![crate_type = "bin"]

extern crate getopts;
use getopts::{reqopt, optflag,getopts,OptGroup};
use std::os;
use common::utils::array_from_file;

pub mod common { 
    pub mod utils;
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-i ,\t--input\t\t Input file");
    println!("-k ,\t--key\t\t Key value");
}

fn boyer_moore_search(array: &[~str], key: &str) -> (bool, uint, uint) {
    if array.len() < 1 { return (false, 0u, 0u) }
    let mut match_pos: uint;
    let mut found: bool;
    let mut i = 0u;

    loop {
        match_pos = 0;
        found = false;
        if i >= array.len() { break }
        let string_i = array[i].clone();
        let str_len = string_i.len() - 1;
        loop {
            let mut jump = 1u;
            let right_index = match_pos + key.len();
            if right_index  >= str_len { break }
            if string_i[match_pos] == key[0] {
                jump = reverse_search(string_i.slice(match_pos,
                    match_pos+key.len()),key);
            }
            if jump == 0 { 
                found = true;
                break;
            } else { 
                match_pos += key.len() - jump;
            }
        }
        if found == true { break }
        i += 1;
    }

    return (found, i, match_pos);
}

fn reverse_search(string_i: &str, key: &str) -> uint {
    let mut i = key.len() - 1;
    while i < key.len() {
        if string_i[i] != key[i] { break }
        i -= 1;
    }
    if i > key.len() { i = 0 }
    return i
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

