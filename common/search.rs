//!Search functions.

use std::hash::hash;

///generic linear search
///For any type which allows binary comparisons, it iterates through
///the array and returns a boolean  for whether or not the key
///was found, and if true returns the element at which it was found.
pub fn linear_search<T: Eq>(array: &[T], key: T) -> (bool, uint) {
    let mut found = false;

    if array.len() < 1 { return (found, 0); }

    let mut i: uint = 0;

    while i < array.len() {
        if array[i] == key {
            found = true;
            break;
        } else { i += 1; }
    }

    return (found, i);
}

/// Binary search. Takes a generic array whose type implements Ord (binary 
/// comparisons), a key, the index at which to begin, and the index at which 
/// to stop. Returns a bool to indicate if it was found and the index at which
/// it was found.
pub fn binary_search<T: Ord >(array: Vec<T>, key: T, min_index_orig: uint,
    max_index_orig: uint) -> (bool, uint) {
    let mut found = false;
    let mut min_index = min_index_orig;
    let mut max_index = max_index_orig;
    let mut mid_index;

    if array.len() < 1 { return (found, 0); }
    
    loop {
        mid_index = (max_index + min_index) / 2;

        if array.as_slice()[mid_index] == key {
            found = true;
            break;
        } else if array.as_slice()[mid_index] < key {
            min_index = mid_index + 1;
        } else {
            max_index = mid_index;
        }
        
        if max_index <= min_index { break }
    }
    
    return (found, mid_index);
}

/// A brute force string search which scans from left to right. Takes an array
/// of strings and a key, and returns whether or not it was found, the index 
/// of the array holding the string in which it was found, and the index of 
/// the string at which the key begins.
pub fn naive_string_search(key: ~str, str_array: ~[~str]) -> (bool, uint, uint) {
    let mut i: uint = 0;
    let mut found = false;
    let mut index: uint;

    loop {
        index = 0;
        if i >= str_array.len() { break }
        let string = str_array[i].clone().to_owned();
        loop {
            if key.len() > string.len() { break }
            if key[0] == string[index] {
                let test_index= compare_strings(key.clone(),
                    string.slice_from(index).to_owned());
                if test_index == key.len() { found = true }
            }
            if found == true { break }
            index += 1;
        }
        if found == true { break }
        i += 1;
    }

    return (found, i, index);
}

/// Knuth-Morris-Pratt string search. Uses partial matches to advance faster
/// for a given mismatch than does the naive string search. Takes an array 
/// of strings, a key, and returns whether the key was found, and if so,
/// the index of the array holding the string in which it was found,
/// and the index of the string at which the key begins.
pub fn kmp_string_search(key: ~str, str_array: ~[~str]) -> (bool, uint, uint) {
    let mut i: uint = 0;
    let mut found = false;
    let mut true_index: uint;
    let mut index: uint;

    loop {
        true_index = 0;
        if i >= str_array.len() { break }
        let string_i = str_array[i].clone();
        loop {
            if key.len() > string_i.len() { break }
            index = compare_strings(key.clone(), 
                string_i.slice_from(true_index).to_owned());
            if index == key.len() {
                found = true;
                break;
            }
            if index > 2 {
                true_index += index - 2;
            } else {
                true_index += 1;
            }
        }
        if found == true { break }
        else { i += 1 }
    }

    return (found, i, true_index);
}

///Takes an array of strings and a key, returns a boolean to indicate if the 
/// string was found, the index of the string in which it was found,
/// and the index of that string at which it was found.
pub fn boyer_moore(array: &[~str], key: &str) -> (bool, uint, uint) {
    if array.len() < 1 || key.len() < 1 { return (false, 0u, 0u) }

    let mut bad_table = [key.len(), .. 256];
    let mut i = 0;
    for &eachbyte in key.as_bytes().iter() {
        let jump = key.len() - 1 -i;
        if jump > key.len() || jump == 0 {
            bad_table[eachbyte as uint] = 1;
        } else {
            bad_table[eachbyte as uint] = jump;
        }
        i += 1;
    }

    let mut match_pos: uint;
    let mut found: bool;
    let mut i = 0u;

    loop {
        match_pos = 0;
        found = false;
        if i >= array.len() { break }
        let string_i = array[i].clone();
        let str_len = string_i.len() - 1;

        loop {
            let right_i = match_pos + key.len();
            if right_i > str_len { break }
            let (found_inner, jump) = reverse_search(
                string_i.slice(match_pos, right_i), key, bad_table);
            if found_inner == true {
                found = true;
                break;
            }
            match_pos += jump;
        }

        if found == true { break }
        i += 1;
    }

    return (found, i, match_pos)
}

/// Compares two strings. Returns an unsigned integer. If the integer is 
/// less than the length of the shorter string, then there is
/// a mismatch; otherwise they are equal.
pub fn compare_strings(stringa: ~str, stringb: ~str) -> uint {
    let mut i = 0;

    loop {
        if stringb.len() < stringa.len() { break }
        if i == stringa.len() { break }
        if stringa[i] != stringb[i] { break }
        i += 1;
    }
    return i;
}

/// Reverse iterates through the two strings; they key and the string
/// we're searching. Returns whether or not  we found a match and an unsigned
/// integer to advance the slicing point.
pub fn reverse_search(stringa: &str, key: &str, table: [uint, ..256]) 
    -> (bool, uint) {

    let mut i = key.char_len();
    let mut j = 1u;
    let mut found = false;
    
    for (chara, charb) in key.bytes().rev().zip(stringa.bytes().rev()) {
        if chara != charb {
            j = table[charb as uint];
            break;
        }
        i -= 1;
    }

    if i == 0 { found = true }
    if j < i + 1 { return (found, j) }
    else { return (found, i + 1) }
}

/// Takes an array of strings and successively iterates through each string.
/// On each iteration, it compares the hash of a substring of equal length
/// to the key to the hash of the key, and terminates either when a match
/// is found or if all possibilities are exhausted. Returns a boolean
/// to indicate if it was found, the index (uint) of the array holding
/// the string in which it was found, and the index (uint) of that
/// string at which the matching substring begins.
pub fn rubin_karp(array: &[~str], key: &str) -> (bool, uint, uint) {
    let mut index = 0u;
    let mut found = false;
    let mut str_index = 0u;

    if array.len() < 1 || key.len() < 1 { return (found, index, str_index) }

    let keyhash = hash(&key);
    
    loop {
        if index >= array.len() { break }
        let string_i = array[index].clone();
        str_index = 0;

        loop { 
            let right_index = str_index + key.len();
            if right_index > string_i.len() { break }
            if hash(&string_i.slice(str_index, right_index)) == keyhash {
                found = true
            }
            if found == true { break }
            else { 
                str_index += 1;
            }
        }

        if found == true { break }
        else { index += 1 }
    }

    return (found, index, str_index)
}
