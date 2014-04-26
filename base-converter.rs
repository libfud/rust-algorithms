#![crate_id = "Base_converter"]
#![crate_type = "bin"]

//!Converts numbers from one base to another. Handles conversion to and from
//!binary, decimal and hexidecimal.

use std::os;

fn decimal_to_base_n(number_dec_orig: uint, base_n: uint) -> ~str {
    if base_n < 2 || base_n > 256 { return ~"Invalid base!" }
    if base_n == 10 { return number_dec_orig.to_str().to_owned(); }
    let mut number_dec = number_dec_orig;
    let mut counter = 0;
    let base_n_prefix = match base_n {
        2   => "0b",
        8   => "0o",
        16  => "0x",
        _   => "0n",
    };

    let base_table = [ 
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

    println!("{}", base_table.len())

    let mut reverse_base_n_str = StrBuf::new();

    loop {
        if number_dec == 0 { break }
        if counter == 8 {
            reverse_base_n_str.push_char(' ');
            counter = 0;
        }
        reverse_base_n_str.push_char(base_table[number_dec % base_n]);
        number_dec /= base_n;
        counter += 1;
    }

    let leading_zeros = "0".repeat(8 - counter);
    let base_n_str: ~str = reverse_base_n_str.to_str().chars_rev().collect();

    return base_n_prefix + leading_zeros + base_n_str
}

fn main() {

    let args = os::args();

    if args.len() < 3 { 
        println!("At least two numbers are required.");
        return;
    }

    let mut number;

    match from_str::<uint>(args[1]) {
        Some(num)    => { number = num },
        _               => { 
            println!("You shouldn't have done such a thing.");
            return
        }
    }

    let mut base;

    match from_str::<uint>(args[2]) {
        Some(num)   => { base = num },
        _           => {
            println!("You shouldn't have done such a thing.");
            return
        }
    }


    println!("{}", decimal_to_base_n(number, base));
}
