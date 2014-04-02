#[crate_id = "stringsearch"];
#[crate_type = "bin"];

//! An implimentation of a straight string search.

use common::utils::array_from_file;
use common::utils::parse_string_to_chars;

pub mod common { pub mod utils; }


///Iterates through an array of strings to see if a key
/// is in any of the strings. On each iteration, the 
/// string is converted to an array of chars and then
/// another loop compares the first char in the array
/// to the first char in the key. If they do not match,
/// the first element from the string is shifted. 
fn find_string(key: ~str, data: ~[~str]) -> bool {
    
    let keychars = parse_string_to_chars(key);
    let mut i = 0;
    let mut found = false;

    loop {
        if i >= data.len() { break }
        let mut datastring = parse_string_to_chars(data[i].to_owned());
        loop {
            if keychars.len() > datastring.len() { break }
            if keychars[0] == datastring[0] {
                found = compare_chars(keychars.clone(), datastring.clone());
            }
            if found == true { break }
            else { datastring.shift(); }
        }
        if found == true { break }
        i += 1;
    }

    return found;
}

/// Compares two arrays of characters. They do not have to be of equal
/// length. However, if the second string is shorter than the first 
/// it will return false.
fn compare_chars(stringa: ~[char], stringb: ~[char]) -> bool {
    let mut i = 0;
    let mut found = true;
    
    loop {
        if stringb.len() < stringa.len() {
            found = false;
            break;
        }
        if i == stringa.len() { break }
        if stringa[i] != stringb[i] {
            found = false;
            break;
        }
        i += 1;
    }
    return found;
}
        

fn main() {
    let args = std::os::args();
    
    if args.len() < 3 {
        println("You must input at least two files; one of the text to be"+
        " searched, and one holding the search string, in that order."+
        "\nFor example, ./searchstring texts/this.txt texts/that.txt");
        return;
    }

    let textpath = args[1].to_owned();
    let textarray = array_from_file(textpath);

    let searchpath = args[2].to_owned();
    let searcharray = array_from_file(searchpath);
    let searchstring = searcharray[0].slice_to(searcharray[0].len() - 1).to_owned();
    //FIXME -- Okay, I still need to work with IO in rust. This is an ugly kludge.

    match find_string(searchstring, textarray) {
        true  => println("Found it."),
        false => println("Did not find it.")
    }
}
