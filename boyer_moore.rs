#![crate_id = "boyer_moore"]
#![crate_type = "bin"]

//!Boyer Moore string search, simplified. No good suffix rule here.

extern crate getopts;
use getopts::{reqopt, optflag,getopts,OptGroup};
use std::os;
use common::utils::array_from_file;
use common::search::boyer_moore;

pub mod common {
    pub mod utils;
    pub mod search;
}

pub fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-i ,\t--input\t\t Input file");
    println!("-k ,\t--key\t\t Key value");
}

pub fn main() {
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

    let (found, index, true_index) = boyer_moore(array, key);

    println!("{} {} {}", found, index, true_index);

}

