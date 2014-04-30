#![crate_id = "Real_Base_Conv"]
#![crate_type = "bin"]

//! Certified failure!

extern crate getopts;
extern crate collections;

use std::os;
use getopts::{reqopt, getopts, Matches};
use collections::ringbuf::RingBuf;
use collections::deque::Deque;
use collections::hashmap::HashMap;

static char_array : &'static [char] = &'static [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F'
];

static uint_max_size: uint = !0;

static bad_char_fail : &'static str =
    "Badly fromatted number. Did you use the right base?";

pub fn num_str_to_ringbuf(numstr: &str, base_m: uint) -> RingBuf<uint> {

    let mut bignum_buf : RingBuf<uint> = RingBuf::new();
    let mut char_table = HashMap::new();
    for i in range(0, base_m) {
        char_table.insert(char_array[i], i);
    }

    let mut current_word = 0;
    let mut place_val = 1;
    for c in numstr.chars_rev() {
        match char_table.find(&c) {
            Some(value) => {
                if uint_max_size - (place_val * *value) >= current_word {
                    current_word += place_val * *value;
                    place_val *= base_m;
                    if current_word == uint_max_size {
                        bignum_buf.push_front(current_word);
                        current_word = 0;
                        place_val = 1;
                    }
                } else {
                    bignum_buf.push_front(current_word);
                    current_word = *value;
                    place_val = base_m;
                }
            },
            None        =>{
                if c != ' ' && c != ',' {
                    println!("{}", bad_char_fail);
                    return bignum_buf
                }
            }
        }
    }
    if current_word > 0 {
        bignum_buf.push_front(current_word);
    }

    bignum_buf
}

pub fn ringbuf_num_to_vec_str(bignum_buf: RingBuf<uint>,base_n: uint) ->
    Vec<StrBuf> {

    let mut current_val: uint;
    let mut rev_str_vec : Vec<StrBuf> = Vec::new();

    for word in bignum_buf.rev_iter() {
        let mut rev_num_str = StrBuf::new();
        current_val = *word;
        loop {
            if current_val == 0 { break }
            rev_num_str.push_char(char_array[current_val % base_n]);
            current_val /= base_n;
        }
        rev_str_vec.push(rev_num_str);
    }

    rev_str_vec
}

pub fn add_bignums(bignum1: &StrBuf, bignum2: &StrBuf, base_n: uint, 
    char_table: &HashMap<char, uint>) -> StrBuf {

    let mut sumstr = StrBuf::new();
    let mut carry = 0;
    for (a, b) in bignum1.to_str().chars().zip(bignum2.to_str().chars()) {
        let val1 = match char_table.find(&a) {
            Some(val)   => *val,
            None        => 0, //I'm tired of matching like this.
        };
        let val2 = match char_table.find(&b) {
            Some(val)   => *val,
            None        => 0, //So very, very tired.
        };
        let partial_sum = val1 + val2 + carry;
        sumstr.push_char(char_array[partial_sum % base_n]);
        carry = partial_sum / base_n;
   }

   sumstr.push_char(char_array[carry % base_n]);

   sumstr
}

pub fn mult_bignum(bignum1:&StrBuf, factor: uint, base_n: uint,
    char_table: &HashMap<char, uint>) -> StrBuf {
    
    let mut product: StrBuf = "0".repeat(bignum1.len()).into_strbuf();

    for _ in range(1, factor) {
        product = add_bignums(&product, bignum1, base_n, char_table);
    }

    product
}

pub fn base_change_nat(numstr: &str, base_m: uint, base_n: uint) -> ~str {
    let bignum_buf = num_str_to_ringbuf(numstr, base_m);
    let rev_str_vec = ringbuf_num_to_vec_str(bignum_buf, base_n);

    let mut char_table = HashMap::new();
    for i in range(0, base_n) {
        char_table.insert(char_array[i as uint], i as uint);
    }

    let mut rev_str_buf = rev_str_vec.as_slice()[0].clone();
    let mut i = 1;

    if rev_str_vec.len() > 1 {
    loop {
        if i == rev_str_vec.len() { break }
        for c in rev_str_vec.as_slice()[i].to_str().chars() {
            let multiplier = match char_table.find(&c) {
                Some(val)   => *val,
                None        => 1
            };
            if multiplier> 0 {
                rev_str_buf = mult_bignum(&rev_str_buf, multiplier, base_n, 
                    &char_table);
            }
        }
        i += 1;
    } 
    }

    let return_str: ~str = rev_str_buf.to_str().chars_rev().collect();

    return_str
}

pub fn uint_from_opt(param: &str, def_val: &str, matches: &Matches) -> uint {
    let mut number;
    match from_str::<uint>(match matches.opt_str(param) {
        Some(string)    => string,
        _               => def_val.to_owned(),
    })
    {
        Some(num)   => { number = num },
        _           => { number = 0 },
    }

    number
}

fn main() {

    let args = os::args();
    let opts = [
        reqopt("f", "from", "original base", "BASE VAL"),
        reqopt("t", "to", "new base", "BASE VAL"),
        reqopt("i", "input-number", "original number", "FORMATTED NUM"),
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => { m },
        Err(f)  => { fail!(f.to_err_msg()) }
    };
    let base_m = uint_from_opt("f", "2", &matches);
    let base_n = uint_from_opt("t", "10", &matches);

    if base_n < 2 || base_n > 16 || base_m < 2 || base_m > 16 {
        println!("Bad base value.");
        return
    }

    let mut input_num;
    match matches.opt_str("i") {
        Some(string)    => { input_num = string }
        _               => {
            println!("Bad number");
            return
        }
    }

    let rebased = base_change_nat(input_num, base_m, base_n);
    println!("{}", rebased);
}
