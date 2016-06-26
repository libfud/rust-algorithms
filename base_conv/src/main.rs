//!Converts numbers from one base to another. Handles conversion to and from
//!binary, octal, decimal, hex or any other base from 2 to 256.

extern crate getopts;

//use std::str::Graphemes;
use std::str::FromStr;
use std::env;
use std::iter::repeat;
use std::collections::HashMap;
use getopts::{Options, Matches};

//b, o, x and n are reserved, other characters are omitted due to
//similarity/readability issues
static CHAR_ARRAY : [char; 256] = [ 
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

static MAX_SIZE: usize = !0;

static BAD_FORMAT_STR: &'static str = 
    "Badly formatted number or bad value for base. Returning 0 in decimal.";

/// Check that an input number is properly formatted for its base
pub fn check_sanity(number: &str, base_m: usize, base_n: usize) -> bool {
    if base_m != 10 {
        if &number[..1] != "0" {
            return false;
        }
        match &number[1..2] {
            "b"|"o"|"x"|"n" => {}, // do nothing, it's ok
            _               => {
                return false;
            }
        }
    }
    if base_n < 2 || base_n > 256 || base_m < 2 || base_m > 256 {
        return false
    }

    let prefix = match base_m {
        10  => 0,
        _   => 2,
    };
    let max_binary = std::mem::size_of::<usize>() * 8;
    let mut max_possible = 8; //256^8 == 2^64
    for x in 1..max_binary {
        if base_m.pow(x as u32) >= MAX_SIZE / base_m {
            max_possible = x;
            break;
        }
    };
    println!("Maximum length of base is {}", max_possible);
    let total_len: usize = number[prefix..].chars().fold(0, |acc, c| {
        match c {
            ' '|'_'|',' => { acc + 0 },
            _           => { acc + 1 }
        }
    });
    println!("Total len: {}", total_len);

    if total_len > max_possible {
        false
    } else {
        true
    }
}

pub fn conv_string_to_int(number: &str, base_m: usize, is_positive: bool,
                          char_table: HashMap<char, usize>)
                          -> (usize, char) {
    
    let mut total_val = 0;
    let mut place_val = 1;
    for c in number.chars().rev() {
        match char_table.get(&c) {
            Some(value) => {
                total_val = match is_positive {
                    true    => total_val + place_val * *value,
                    _       => total_val + place_val * (base_m - 1 - *value)
                };
                place_val *= base_m;
            },
            None        => {
                if c != ' ' && c != ',' && c!= '_' { return (total_val, c) }
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
pub fn base_m_to_base_n_usize(number: &str,base_m: usize, base_n: usize,
                              word_length: usize) -> String {

    if check_sanity(number, base_m, base_n) == false {
        println!("{}", BAD_FORMAT_STR);
        return "0".to_owned();
    }

    let (pretty_char, prefix, word_length, base_n_prefix) = 
        pretty(base_m, base_n, word_length);

    //counter is used in determining how many 0s to prepend to the leading
    //"word"
    let mut counter = 0;

    let mut character_table = HashMap::new();
    
    for i in 0.. base_m {
        character_table.insert(CHAR_ARRAY[i], i);
    }

    let (mut total_val, bad_char) = conv_string_to_int(
        &number[prefix..], base_m, true, character_table
    );
    if bad_char!= '0' { return bad_char_fail(bad_char) };

    let mut reverse_base_n_str = String::new();

    loop {
        if total_val == 0 { break } //still works for an original input of 0
        if counter == word_length {
            reverse_base_n_str.push(pretty_char);
            counter = 0;
        }
        reverse_base_n_str.push(CHAR_ARRAY[total_val % base_n]);
        //because the modulus of input number by the new base can't
        //be more than the new base, and the array holds 256 characters,
        //which is the highest allowed base, we don't have to worry
        //about out of bounds access.
        total_val = total_val / base_n;
        counter += 1;
    }

    let leading_zeros: String = repeat("0").take(word_length - counter).collect();
    let base_n_str: String = reverse_base_n_str.to_string().chars().rev().collect();
    //reverse the reversed string by collecting the chars from a reverse
    //iteration

    if base_n == 10 {
        return base_n_str
    }
    //if you specify a pretty value of 64 no input number can generate
    //spaces. Unfortunately, this means that you'll have many leading
    //zeros unless you have a pretty large number.

    return base_n_prefix + &leading_zeros + &base_n_str
}

pub fn pretty(base_m: usize, base_n: usize, mut word_length: usize) ->
    (char, usize, usize, String) {

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

pub fn usize_from_opt(param: &str, default_val: &str, matches: &Matches) -> usize {
    let number;
    let num_str = match matches.opt_str(param) {
        Some(string)    => string,
        _               => default_val.to_string()
    };
    match usize::from_str(&num_str) {
        Ok(num)   => { number = num },
        _         => { number = 0 },
    }
    return number
}

fn bad_char_fail(c: char) -> String {
    println!("{}", BAD_FORMAT_STR);
    println!("Offending char was {}", c);
    return "0".to_owned();
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt("f", "from", "original base", "BASE VALUE");
    opts.reqopt("t", "to", "new base", "BASE VALUE");
    opts.reqopt("i", "input-number", "original number", "FORMATTED NUMBER");
    opts.optopt("p", "pretty", "word length", "VALUE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => { m },
        Err(f)  => { panic!(f.to_string())}
    };

    let base_m = usize_from_opt("f", "10", &matches);
    let base_n = usize_from_opt("t", "16", &matches);

    if base_n < 2 || base_n > 256 || base_m < 2 || base_m > 256 { 
        println!("Hey! What exactly are you trying to pull here?");
        return
    }

    let mut word_length = 8;
    if matches.opt_present("p") { 
        word_length = usize_from_opt("p", "8", &matches);
    }

    let input_num;
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

    let rebased = base_m_to_base_n_usize(&input_num, base_m, base_n, word_length);

    println!("{}", rebased);
}
