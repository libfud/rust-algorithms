#![crate_id = "Base_converter"]
#![crate_type = "bin"]

//!Converts numbers from one base to another. Handles conversion to and from
//!binary, octal, decimal, hex or any other base from 2 to 256.

extern crate collections;
extern crate getopts;

use std::os;
use collections::hashmap::HashMap;
use getopts::{reqopt, getopts, optopt, Matches};
//b, o, x and n are reserved, other characters are omitted due to
//similarity/readability issues
static char_array : &'static [char] = &'static [ 
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
    'f', 'g', 'h', 'i', 'j', 'k', 'm', 'p', 'w', 'z', 
    'あ', 'い', 'え', 'か', 'き', 'く', 'け', 'こ', 'し', 'す',
    'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'ぬ', 'ね',
    'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'み', 'む', 'も', 'や',
    'ゆ', 'よ', 'り', 'る', 'れ', 'わ', 'を', '一', '二', '三',
    '四', '五', '八', '九', '十', '子', '女', '口', '下', '日',
    '水', '何', '田', '円', '万', '全', '又', '心', '夕', '上',
    '公', '仏', '私', '刀', 'ツ', '少', '土', '止', '正', '本',
    '大', '王', '中', 'ス', '区', '川', '天', '弓', '今', '父',
    '也', '亡', '光', '猫', '犬', '具'
];

static bad_format_str: &'static str = 
    "Badly formatted number or bad value for base. Returning 0 in decimal.";

/// Check that an input number is properly formatted for its base
pub fn check_sanity(number: &str, base_m: uint, base_n: uint) -> bool {
    //FIXME incorporate more checks, this just checks if it has a prefix or not
    if base_m != 10 {
        if number.slice_to(1) != "0" {
            return false;
        }
        match number.slice(1,2) {
            "b"|"o"|"x"|"n" => {}, // do nothing, it's ok
            _               => {
                return false;
            }
        }
    }
    if base_n < 2 || base_n > 256 || base_m < 2 || base_m > 256 {
        return false
    }

    return true;
}

pub fn conv_string_to_int(number: &str, base_m: uint, is_positive: bool,
    char_table: HashMap<char, uint>) -> (uint, char) {
    
    let mut total_val = 0;
    let mut place_val = 1;
    for c in number.chars_rev() {
        match char_table.find(&c) {
            Some(value) => {
                total_val += match is_positive {
                    true    => place_val * *value,
                    _       => place_val * (base_m - 1 - *value)
                };
                place_val *= base_m;
            },
            None        => {
                if c != ' ' { return (total_val, c) }
            }
        }
    }
    if is_positive == false {
        total_val += 1; //probably some modulus reason here for this
    }
    return (total_val, '0') // 0 cannot fail as all bases have it
}

/// Takes a string formatted by the user to represent a number in a base that
/// the user discloses as an argument, and returns a string formatted for
/// another base that the user specifies. The first string is converted
/// to an unsigned integer based on size_m, and that unsigned integer is then
/// converted to a string. An additional formatting option is present as 
/// word_length, which by default is 8. For any non-decimal base, spaces
/// separate these "words." For decimal, this character is instead a comma.
/// Non decimal numbers will have leading zeros prepended to them to make
/// them the corrent length for the word.
pub fn base_m_to_base_n_uint(number: &str, base_m: uint, base_n: uint,
    word_length: uint) -> ~str {

    if check_sanity(number, base_m, base_n) == false {
        println!("{}", bad_format_str);
        return "0".to_owned();
    }

    let (pretty_char, prefix, word_length, base_n_prefix) = 
        pretty(base_m, base_n, word_length);

    //counter is used in determining how many 0s to prepend to the leading
    //"word"
    let mut counter = 0;

    let mut character_table = HashMap::new();
    
    for i in range(0, base_m) {
        character_table.insert(char_array[i], i);
    }

    let (mut total_val, bad_char) = conv_string_to_int(
        number.slice_from(prefix), base_m, true, character_table
    );
    if bad_char!= '0' { return bad_char_fail(bad_char) };

    let mut reverse_base_n_str = StrBuf::new();

    loop {
        if total_val == 0 { break } //still works for an original input of 0
        if counter == word_length {
            reverse_base_n_str.push_char(pretty_char);
            counter = 0;
        }
        reverse_base_n_str.push_char(char_array[total_val % base_n]);
        //because the modulus of input number by the new base can't
        //be more than the new base, and the array holds 256 characters,
        //which is the highest allowed base, we don't have to worry
        //about out of bounds access.
        total_val /= base_n;
        counter += 1;
    }

    let leading_zeros = "0".repeat(word_length - counter);
    let base_n_str: ~str = reverse_base_n_str.to_str().chars_rev().collect();
    //reverse the reversed string by collecting the chars from a reverse
    //iteration

    if base_n == 10 { return base_n_str }
    //if you specify a pretty value of 64 no input number can generate
    //spaces. Unfortunately, this means that you'll have many leading
    //zeros unless you have a pretty large number.

    return base_n_prefix + leading_zeros + base_n_str
}

pub fn base_m_to_base_n_signed_int(number: &str, base_m: uint, base_n: uint,
    word_length: uint) -> ~str {

    let mut neg_prefix = 0;

    if number.slice(2,3) != "1" {
        return base_m_to_base_n_uint(number, base_m, base_n, word_length)
    } else if number.slice_to(1) == "-" {
        neg_prefix = 1;
    }

    if check_sanity(number.slice_from(neg_prefix), base_m, base_n) == false {
        println!("{}", bad_format_str);
        return "0".to_owned();
    }

    let (pretty_char, mut prefix, word_length, base_n_prefix) =
        pretty(base_m,base_n,word_length);
    prefix += 1; //Doesn't matter if the negativity is specified by a '-' or
                 //by a leading 1; the "real" part of the number starts after
                 //3 characters. This also applies to decimal, in which case
                 //we change the prefix from 0 to 1

    let mut counter = 0;
    
    let mut character_table = HashMap::new();

    for i in range(0,base_m) {
        character_table.insert(char_array[i], i);
    }

    let (mut total_val, bad_char) = conv_string_to_int(
        number.slice_from(prefix), base_m, false, character_table
    );
    if bad_char != '0' { return bad_char_fail(bad_char) }

    let mut rev_base_n_str = StrBuf::new();

    loop {
        if total_val == 0 { break }
        if counter == word_length {
            rev_base_n_str.push_char(pretty_char);
            counter = 0;
        }
        rev_base_n_str.push_char(
            char_array[total_val % base_n]
        );
        total_val /= base_n;
        counter += 1;
    }
    
    let mut base_n_str: ~str = rev_base_n_str.to_str().chars_rev().collect();

    let neg_pre_str = match base_n {
        10  => "-",
        _   => "1",
    };

    let mut leading_chars= match word_length - counter {
        0   => char_array[base_n-1].to_str().repeat(word_length - 1) + " ",
        _   => char_array[base_n-1].to_str().repeat(word_length - counter - 1)
    };

    if base_n == 10 {
        leading_chars = "".to_owned();
    }

    if  base_n_str.len() == 0 {
        base_n_str = "0".to_owned();
    }

    return base_n_prefix + neg_pre_str + leading_chars + base_n_str;
}

pub fn pretty(base_m: uint, base_n: uint, mut word_length: uint) ->
    (char, uint, uint, ~str) {

    let mut pretty_char = ' ';
    if base_n == 10 {
        pretty_char = ',';
        word_length = 3;
    }
    let mut prefix = 0;
    if base_m != 10 { prefix = 2 };
    let base_n_prefix = match base_n {
        2   => "0b",
        8   => "0o",
        10  => "",
        16  => "0x",
        _   => "0n",
    };

    (pretty_char, prefix, word_length, base_n_prefix.to_owned())
}

pub fn uint_from_opt(param: &str, default_val: &str, matches: &Matches)
    -> uint {
    let mut number;
    match from_str::<uint>(match matches.opt_str(param) {
            Some(string)    => string,
            _               => default_val.to_owned(),
        }) {
        Some(num)   => { number = num },
        _           => { number = 0 },
    }
    return number
}

fn bad_char_fail(c: char) -> ~str {
    println!("{}", bad_format_str);
    println!("Offending char was {}", c);
    return "0".to_owned();
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

    let base_m = uint_from_opt("f", "10", &matches);
    let base_n = uint_from_opt("t", "16", &matches);

    if base_n < 2 || base_n > 256 || base_m < 2 || base_m > 256 { 
        println!("Hey! What exactly are you trying to pull here?");
        return
    }

    let mut word_length = 8;
    if matches.opt_present("p") { 
        word_length = uint_from_opt("p", "8", &matches);
    }

    let mut input_num;
    match matches.opt_str("i") {
        Some(string)    => { input_num = string }
        _               => { 
            println!("No");
            return
        }
    }

    //I could check to see if base_n and base_m are equal here, however,
    //if you want to reformat the number with a different "word" length,
    //doing that check and returning the input number would disallow that

    let rebased = base_m_to_base_n_uint(
        input_num, base_m, base_n, word_length
    );
    let other_base = base_m_to_base_n_signed_int(
        input_num, base_m, base_n, word_length
    );

    println!("{}", rebased);
    println!("{}", other_base);
}
