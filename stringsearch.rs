#![crate_id = "naive_stringsearch"]
#![crate_type = "bin"]

//! An implimentation of a naive string search.

use common::utils::array_from_file;

pub mod common { pub mod utils; }


///find_string takes an owned string for a key and an array of owned strings
/// as the text to be searched, and returns the string's index in the array, 
/// and the index of string at which the first instantiation of the key was
/// found.
fn find_string(keychars: ~str, data: ~[~str]) -> (uint, int) {
    
    let mut i = 0;
    let mut found = false;
    let mut index: int = 0;  //int rather than uint so -1 can show not found

    loop {
        if i >= data.len() { break } //don't try OOB access
        let mut datastring = data[i].clone();
        index = 0;
        loop {
            if keychars.len() > datastring.len() { break } //they key can't
                //be in something shorter than itself
            if keychars[0] == datastring[0] {
                found = compare_chars(keychars.clone(), datastring.clone());
            }
            if found == true { break }
            datastring.shift_char();
            index += 1; //to locate where the key starts in the string
        }
        if found == true { break }
        i += 1;
    }

    if found == false { index = -1 }
    return (i, index);
}

/// Compares two arrays of characters. They do not have to be of equal
/// length. However, if the second string is shorter than the first 
/// it will return false.
fn compare_chars(stringa: ~str, stringb: ~str) -> bool {
    let mut i = 0;
    let mut found = true; //assume true til proven false
    
    loop {
        if stringb.len() < stringa.len() { //a key can't be in something shorter
            //than itself
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
        print!("You must input at least two files; one of the text to be");
        println!(" searched, and one holding the search string, in that order.");
        println!("For example, ./searchstring texts/this.txt texts/that.txt");
        return;
    }

    let textpath = args[1].to_owned();
    let textarray = array_from_file(textpath);

    let searchpath = args[2].to_owned();
    let searcharray = array_from_file(searchpath);
    let searchstring = searcharray[0].slice_to(searcharray[0].len() - 1).to_owned();
    //FIXME -- Okay, I still need to work with IO in rust. This is an ugly kludge.

    let (textindex, stringindex) = find_string(searchstring, textarray.clone());
    if stringindex < 0 { 
        println!("Not found.");
        return;
    }
    else {
        println!("{} {}", textindex, stringindex);
    }
    
}
