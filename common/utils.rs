//!  Commonly used functions 

extern crate rand;
use std::io;
use std::io::{File, BufferedReader, Open, Read};

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
    let mut reader = io::stdin();
    let mut string = reader.read_line().ok().unwrap_or(~"invalid");
    if string == ~"" { string = ~"is invalid" }
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

/// Generates an array with the number of elements specified by size.
/// Upper bound limits the size of the numbers generated to itself
/// times the number of elements requested. For example, if you
/// request 10 elements and specify an upper bound of 2, you will get
/// 10 numbers ranging in size from 1 to 20.
/*pub fn array_gen(size: uint, upper_bound: uint) -> Vec<int> {
    let range = Range::new(1, size * upper_bound);
    let mut rng = task_rng();

    range(0, size).map(|_| range.ind_sample(&mut rng)).collect()
}*/

/// Facilitates getting data from files in the form of an array
/// of strings, each string consisting of one line from the file.
pub fn array_from_file(strpath: &str) -> ~[~str] {
    let path = Path::new(strpath);
    let mut lines: ~[~str] = ~[];

    let file = match File::open_mode(&path, Open, Read) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

    let mut reader = BufferedReader::new(file);

    for aline in reader.lines() {
        lines.push(aline.unwrap());
    }

    return lines;
}

/// Uses array from file to return an array of integers. This
/// needs some more work.
pub fn int_array_from_file(strpath: &str) -> ~[int] {
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
pub fn float_array_from_file(strpath: &str) -> (~[f64],~[~str]) {
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
        if float_chars.len() == 0 { break }
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
    if char_string.len() > 0 {
        for &elem in char_string.iter() {
            string.push_char(elem);
        }
    }
    return string;
}

/// Parses a string in date format into a tuple of three integers for
/// day, month and year

pub fn parse_date(string_orig: ~str) -> (uint, uint, uint) {
    let date_string = string_orig.trim_left().to_owned();
    let mut date_chars = parse_string_to_chars(date_string);
    let mut date = [0, 0, 0]; // an array to represent the day, month, and year.
    
    let mut date_index = 0;

    let mut format_invalid = 0; //ensuring valid format
    for &elem in date_chars.iter() {
        format_invalid += match elem {
            '0'..'9' => 0,
            _        => 1
        };
    }
    if format_invalid > 2 {
        println!("Invalid!");
        return (0,0,0);
    }
    let mut temp_array: ~[char] = ~[];
    while date_chars.len() > 0 { 
        let date_scope_flag = match date_chars[0] {
            '0'..'9'=> false,
            _       => true
        };
        if date_scope_flag == false {
            let temp_date_char = match date_chars.shift() {
                Some(c) => c,
                _       => '0',
            };
            temp_array.push(temp_date_char);
        }
        else {
            date[date_index] = match from_str::<uint>(parse_chars_to_string(temp_array.clone())) {
                Some(num) => num,
                _         => 0
            };
            temp_array = ~[];
            if date_chars.len() > 0 { date_chars.shift(); }
            date_index += 1;
        }
    }
    date[2] = match from_str::<uint>(parse_chars_to_string(temp_array)) {
        Some(num) => num,
        _         => 0
    };
    return (date[0], date[1], date[2]);
}
