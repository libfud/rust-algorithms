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

pub fn binary_search<T: Ord >(array: &[T], key: T, min_index_orig: uint, max_index_orig: uint) -> (bool, uint) {
    let mut found = false;
    let mut min_index = min_index_orig;
    let mut max_index = max_index_orig;
    let mut mid_index;

    if array.len() < 1 { return (found, 0); }
    
    loop {
        mid_index = (max_index + min_index) / 2;

        if array[mid_index] == key {
            found = true;
            break;
        } else if array[mid_index] < key {
            min_index = mid_index + 1;
        } else {
            max_index = mid_index;
        }
        if max_index <= min_index { break }
    }
    
    return (found, mid_index);
}

pub fn naive_string_search(key: ~str, str_array: ~[~str]) -> (bool, uint, uint) {
    let mut i: uint = 0;
    let mut found = false;
    let mut index: uint;

    loop {
        index = 0;
        if i >= str_array.len() { break }
        let mut string = str_array[i].clone();
        loop {
            if key.len() > string.len() { break }
            if key[0] == string[0] {
                found = compare_strings(key.clone(), string.clone());
            }
            if found == true { break }
            string.shift_char();
            index += 1;
        }
        if found == true { break }
        i += 1;
    }

    return (found, i, index);
}

pub fn compare_strings(stringa: ~str, stringb: ~str) -> bool {
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
