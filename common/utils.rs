//!  Commonly used functions 

use std::io;
use std::io::File;
use std::io::buffered::BufferedReader;
use std::rand::{task_rng, Rng};

/// Takes a yes or no answer in string form and returns a boolean value.
pub fn answer_to_bool(string_orig: ~str) -> bool {
    let string = string_orig;
    match string.trim_left() {
        "y" | "Y" => return true,
        _ => return false
    };
}

/// Returns a string in response to a question.
pub fn string_getter(question: &str) -> ~str {
    println!("{}",question);
    let mut reader = BufferedReader::new(io::stdin());
    let mut string = reader.read_line().unwrap_or(~"No");
    string = string.slice_to(string.len() - 1).trim_left().to_owned();
    if string == ~"" { string = ~"is invalid" }
    println!("You input {}",string);
    return string;
}

/// Takes user input and returns an integer.
pub fn number_getter(question: &str) -> int {
    let number = string_getter(question);
    let num = from_str::<int>(number);
    match num {
        Some(num) => return num,
        None => return 0
    }
}

/// Takes user input and returns a floating point number.
pub fn float_getter(question: &str) -> f64 {
    let number = string_getter(question);
    let num = from_str::<f64>(number);
    match num {
        Some(num) => return num,
        None      => return 0.0
   }
}

/// Geneerates an array with the number of elements specified by size.
/// Upper bound limits the size of the numbers generated to itself
/// times the number of elements requested. For example, if you
/// request 10 elements and specify an upper bound of 2, you will get
/// 10 numbers ranging in size from 1 to 20.
pub fn array_gen(size: int, upper_bound: int) -> ~[int] {
    println!("Generating {} random numbers", size);
    let mut array =  ~[];
    let mut i = 0;
    while i <= size {
        let mut rng = task_rng();
        let y: int = rng.gen_range(1,upper_bound*size);
        array.push(y);
        i += 1;
    }
    return array;
}

/// Facilitates getting data from files in the form of an array
/// of strings, each string consisting of one line from the file.
pub fn array_from_file(strpath: ~str) -> ~[~str] {
    let path = Path::new(strpath);
    let mut file = BufferedReader::new(File::open(&path));
    let lines: ~[~str] = file.lines().collect();
    
    return lines
}

/// Uses array from file to return an array of integers. This
/// needs some more work.
pub fn int_array_from_file(strpath: ~str) -> ~[int] {
    let lines = array_from_file(strpath);
    let size = lines.len();
    let mut i = 0;
    let mut array: ~[int] = ~[];
    while i < size {
        let numstr = &lines[i];
        let num = from_str::<int>(numstr.slice_to(numstr.len() - 1));
        let number: int = match num {
            Some(num) => num,
            None => 0
        };
        array.push(number);
        i+=1;
    }
    return array;
}

/// Uses array_from_file and parse_string_to_chars and parse_string_to_float
/// to return two vectors from a file: one of floating point numbers, and
/// another of owned strings. 
pub fn float_array_from_file(strpath: ~str) -> (~[f64],~[~str]) {
    let lines = array_from_file(strpath);
    let size = lines.len();
    let mut i = 0;
    let mut float_array: ~[f64] = ~[];
    let mut string_array: ~[~str] = ~[];
    while i < size {
        let numstr = lines[i].trim_left().to_owned();
        let (number, rest_of_string) = parse_string_to_float(numstr);
        float_array.push(number);
        string_array.push(rest_of_string);
        i+=1;
    }
    return (float_array, string_array);
}

/// Takes an owned string and returns a floating point numbger
/// and an owned string, if possible.
pub fn parse_string_to_float(string_orig: ~str) -> (f64, ~str) {
    let string = string_orig.trim_left().to_owned();
    let mut float_chars = parse_string_to_chars(string);
    let mut float_string: ~str = ~"0";
    let mut decimal_flag = false;
    loop {
        let number_bool = match float_chars[0] {
            '0'..'9'|'.' => true,
            _ => false
        };
        if  float_chars[0] == '.' {
            if decimal_flag == false {
                decimal_flag = true;
                float_string.push_char(float_chars[0]);
                float_chars.shift();
            }
            else { break }
        }
        else if number_bool == true {
            float_string.push_char(float_chars[0]);
            float_chars.shift();
        }
        else { break }
    }
    let float_number = match from_str::<f64>(float_string) {
        Some(num) => num,
        None => 0.0
    };
    let rest_of_string = parse_chars_to_string(float_chars);
    return (float_number, rest_of_string);
}

/// Turns an owned string into a vector of chars.
pub fn parse_string_to_chars(string: ~str) -> ~[char] {
    let mut char_string: ~[char] = ~[];
    for char_elem in string.chars() {
        char_string.push(char_elem);
    }
    return char_string;
}

/// Turns an owned vector of chars into an owned string.
pub fn parse_chars_to_string(char_string: ~[char]) -> ~str {
    let mut string: ~str = ~"";
    for &elem in char_string.iter() {
        string.push_char(elem);
    }
    return string;
}
