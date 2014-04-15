#![crate_id = "boyer_moore"]
#![crate_type = "bin"]

extern crate getopts;
extern crate collections;
use getopts::{reqopt, optflag,getopts,OptGroup};
use collections::HashMap;
use std::os;
use common::utils::{array_from_file};

pub mod common { 
    pub mod utils;
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-i ,\t--input\t\t Input file");
    println!("-k ,\t--key\t\t Key value");
}

fn boyer_moore_search(array: &[~str], key: &str) -> (bool, uint, uint) {
    if array.len() < 1 || key.len() < 1 { return (false, 0u, 0u) }
    let bad_table = bad_char_table_gen(key);
    let mut match_pos: uint;
    let mut found: bool;
    let mut i = 0u;
    let mut keylen = 0u;

    for _ in key.chars() {
        keylen += 1;
    }

    loop {
        match_pos = 0;
        found = false;
        if i >= array.len() { break }
        let string_i = array[i].clone();
        let str_len = string_i.len() - 1;

        loop {
            let right_i = match_pos + key.len();
            if right_i  > str_len { break }
            let (found_inner,jump) = reverse_search(
                string_i.slice(match_pos, right_i), key, keylen,
                bad_table.clone());
            if found_inner == true {
                found = true;
                break;
            }
            match_pos += jump;
        }

        if found == true { break }
        i += 1;
    }

    return (found, i, match_pos);
}

fn reverse_search(stringa: &str,key: &str, keylen: uint,
    table: HashMap<char,uint>) -> (bool, uint) {
    let mut i = keylen;
    let mut j = 1u;
    let mut found = false;
    for (chara, charb) in key.chars_rev().zip(stringa.chars_rev()) {
        if chara != charb {
            j  = match table.find(&charb) {
                Some(jump_value)    => jump_value.clone(),
                _                   => j
            }; 
            break;
        }
        i -= 1;
    }
    if i == 0  { found = true }
    if j < i + 1 { return (found, j) }
    return (found, i + 1) 
}

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

