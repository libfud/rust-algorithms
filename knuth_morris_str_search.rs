#![crate_id = "kmp_stringsearch"]
#![crate_type = "bin"]

//! An implementation of the Knuth-Morris-Pratt string search algorithm.
//! I think. I didn't read it too carefully, so this might not actually capture
//! the spirit of it, but it works, and it's faster than naive string search.

use common::utils::array_from_file;

pub mod common { pub mod utils; }

///Takes an array of owned strings and a string, and returns the
///index of the string holding the key and the index at which the
///key starts.
fn kmp_find_string(keychars: &str, data: ~[~str]) -> (uint, uint) {

    let mut i = 0;
    let mut found = false;
    let mut true_index: uint = 0;
    let mut index: uint;

    loop {
        if i >= data.len() { break }  // don't go out of bounds
        let mut datastring = data[i].clone();
        true_index = 0; //resets to 0 for the beginning of the string
        loop {
            if keychars.len() > datastring.len() { break }
            //key can't be in something shorter than it
            index = compare_chars(keychars.clone(),datastring.clone());
            if index == keychars.len() { found = true }
            //ie, it iterated successfully through both strings and the key
            //exists in the string
            //need this to return
            if found == true { break }
            if index > 1 {
                datastring = datastring.slice_from(index - 1).to_owned();
                true_index += index - 1;
            }
            //this only executes if the key was not in the string;
            //it cuts off the string up to one character prior to the first
            //nonmatching character. I probably have wrong arithmatic here, but
            //this errs on the side of caution.
            else { 
                datastring.shift_char();
                true_index += 1;
            }
            //this happens if there was an immediate mismatch (index = 0)
        }
        if found == true { break }
        i += 1;
        //advance to the next string
    }

    return (i, true_index);
}

///Compare two arrays of characters. Returns the index at which it stops
///comparing them, which will always be equal to or less than the length
///of the first string.
fn compare_chars(stringa: &str, stringb: &str) ->  uint {
    let mut i = 0;

    loop {
        if stringb.len() < stringa.len() {
            break;
        }
        if i == stringa.len() { break }
        if stringa[i] != stringb[i] {
            break;
        }
        i += 1;
    }
    return  i;
}

fn main() {
    let args = std::os::args();

    if args.len() < 3 {
        print!("I need two filenames, one for the text to be searched, and ");
        println!("one for the key, in that order.");
        return;
    }

    let textpath = args[1].to_owned();
    let textarray = array_from_file(textpath);

    let searchpath = args[2].to_owned();
    let searcharray = array_from_file(searchpath);
    let searchstring = searcharray[0].slice_to(searcharray[0].len() - 1).to_owned();
    
    let (textindex, stringindex) = kmp_find_string(searchstring, textarray.clone());

    if textindex >= textarray.len() {
        println!("Not found.");
        return;
    } else {
        println!("{} {}", textindex, stringindex);
    }
}

