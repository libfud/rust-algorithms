#![crate_id = "Base_converter"]
#![crate_type = "bin"]

//!Converts numbers from one base to another. Handles conversion to and from
//!binary, decimal and hexidecimal.

extern crate collections;
extern crate getopts;

use std::os;
use collections::hashmap::HashMap;
use getopts::{reqopt, getopts, optopt};

pub fn base_m_to_base_n(number: &str, base_m: uint,
    base_n: uint, word_length: uint) -> ~str {

    let mut prefix = 0;

    if base_m != 10 { prefix = 2 }

    if base_n < 2 || base_n > 256 { return ~"Invalid base!" }

    let mut counter = 0;

    let base_n_prefix = match base_n {
        2   => "0b",
        8   => "0o",
        10  => "",
        16  => "0x",
        _   => "0n",
    };

    let char_array = [ 
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'L', 'M', 'N', 'P', 'R', 'S', 'T', 'U', 'W', 'X',
    'Y', 'Z', 'α', 'Γ', 'Δ', 'δ', 'ε', 'ζ', 'η', 'λ',
    'μ', 'Ξ', 'ξ', 'π', 'Σ', 'σ', 'φ', 'Ψ', 'Ω', 'ω',
    '&', '@', '€', '₤', '$', 'p', 'f', 'h', '~', '*',
    'Б', 'Д', 'Ж', 'Й', 'Э', 'Ю', 'Я', 'Ѣ', 'ʡ', '?',
    '=', '≺', '≻', '⊗', 'ℵ', '◅', '▻', '⋈', '∴', '↯',
    '↔', '∀', '≅', '∅', '∌', 'ℚ', '∞', '☿', 'Ǣ', 'Ӂ',
    '☿', '♃', '♄', '♅', 'ʡ', '♇', '♈', '♉', '♊', '♋',
    '♌', '♍', '♎', '♏', '♑', '♒', '♓', '℞', '☄', '@',
    'ℜ', 'Ⅎ', '⇅', '⇄', '⇇', '⇈', '⇉', '⇊', '⇖', '⇗',
    '⇘', '⇙', '√', '∛', '∜', '∰', '∽', '≎', '⌘', '⌨',
    '⌶', '☂', '★', '☆', '☚', '☛', '☝', '☟', '☠', '☢',
    '♚', '♛', '♜', '♝', '♞', '♟', '♠', '♡', '♢', '♣',
    '⚐', '⚒', '⚓', '⚕', '⚛', '⚠', '⚡', 'a', 'c', 'e',
    'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p',
    'あ', 'い', 'え', 'か', 'き', 'く', 'け', 'こ', 'し', 'す',
    'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'ぬ', 'ね',
    'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'み', 'む', 'も', 'や',
    'ゆ', 'よ', 'り', 'る', 'れ', 'わ', 'を', '一', '二', '三',
    '四', '五', '八', '九', '十', '子', '女', '口', '下', '日',
    '水', '何', '田', '円', '万', '全', '又', '心', '夕', '上',
    '公', '仏', '私', '刀', 'ツ', '少', '土', '止', '正', '本',
    '大', '王', '中', 'ス', '区', '川', '天', '弓', '今', '父',
    '也', '亡', '光', '猫', '犬', '具'];

    let mut character_table = HashMap::new();

    let mut i : uint = 0;

    for c in char_array.iter() {
        character_table.insert(*c, i);
        i += 1;
    }

    let mut place_val = 1;
    let mut total_val = 0;
    for c in number.slice_from(prefix).chars_rev() {
        match character_table.find(&c) {
            Some(value) => {
                total_val += place_val * *value;
                place_val *= base_m;
            },
            None        => { }
        }
    }

    let mut reverse_base_n_str = StrBuf::new();

    loop {
        if total_val == 0 { break }
        if counter == word_length {
            reverse_base_n_str.push_char(' ');
            counter = 0;
        }
        reverse_base_n_str.push_char(char_array[total_val % base_n]);
        total_val /= base_n;
        counter += 1;
    }

    let leading_zeros = "0".repeat(word_length - counter);
    let base_n_str: ~str = reverse_base_n_str.to_str().chars_rev().collect();

    if base_n == 10 { return base_n_str }

    return base_n_prefix + leading_zeros + base_n_str
}

fn main() {

    let args = os::args();

    let opts = [
        reqopt("f", "from", "original base", "BASE VALUE"),
        reqopt("t", "to", "new base", "BASE VALUE"),
        reqopt("i", "input-number", "original number", "FORMATTED NUMBER"),
        optopt("p", "pretty", "word length", "VALUE")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => { m },
        Err(f)  => { fail!(f.to_err_msg()) }
    };


    if args.len() < 4 { 
        println!("At least two numbers are required.");
        return;
    }

    let mut base_m;
    match from_str::<uint>(match matches.opt_str("f") {
            Some(string)    => string,
            _               => ~"10"
        }) {
        Some(num)   => { base_m = num },
        _           => {
            println!("You shouldn't have done such a thing.");
            return
        }
    }

    let mut base_n;
    match from_str::<uint>(match matches.opt_str("t"){
            Some(string)    => string,
            _               => ~"16"
        }) {
        Some(num)   => { base_n = num },
        _           => {
            println!("You shouldn't have done such a thing.");
            return
        }
    }

    let mut word_length = 8;
    if matches.opt_present("p") {
        match from_str::<uint>(
            match matches.opt_str("p") {
                Some(string)    => string,
                _               => ~"Invalid"
            })  {
                Some(num)   => { word_length = num },
                _           => {
                    println!("Bad pretty value.");
                    return
                }
        }
    }

    let mut input_num;
    match matches.opt_str("i") {
        Some(string)    => { input_num = string }
        _               => { 
            println!("No");
            return
        }
    }

    let rebased = base_m_to_base_n(input_num, base_m, base_n, word_length);

    println!("{}", rebased);

}
