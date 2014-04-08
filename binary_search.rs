#![crate_id = "binary_search"]
#![crate_type = "bin"]

extern crate getopts;
use getopts::{reqopt,optflag,getopts,OptGroup};
use std::os;
use common::utils::int_array_from_file;
use common::search::binary_search;

pub mod common {
    pub mod utils;
    pub mod search;
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-i ,\t--input\t\t Input file");
    println!("-k ,\t--key\t\t Key value");
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
        Ok(m)   =>  { m }
        Err(f)  =>  { fail!(f.to_err_msg()) }
    };
    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    }

    let input_filename = match matches.opt_str("i") {
        Some(string) => string,
        _            => ~"INVALID"
    };
    if input_filename == ~"INVALID" {
        println!("{}", input_filename);
        return;
    }
    let array = int_array_from_file(input_filename);

    let key_string = match matches.opt_str("k") {
        Some(string) => string,
        _            => ~"none"
    };

    let key = match from_str::<int>(key_string) {
        Some(num) => num,
        _         => 0,
    };

    let (found, index) = binary_search(array.clone(), key.clone(), 0, array.len() - 1);
    println!("{} {}", found, index);
}

