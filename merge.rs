#[crate_id = "merge_sort"];
#[crate_type = "bin"];

//! An implementation of merge sort.

use std::vec;

pub mod common{ pub mod utils; }

///Merge_sort will recursively split an array into two subarrays until
/// each has a length of 1 element.
fn merge_sort(array: ~[int]) -> ~[int] {
    let length = array.len();
    if length <= 1 {
        return array.to_owned();
    }

    let middle = length / 2;
    let mut left = array.slice(0, middle).to_owned();
    let mut right = array.slice(middle, length).to_owned();

    left = merge_sort(left);
    right = merge_sort(right);

    merge(left, right)
}

///merge will reinsert the elements into the correct order, going from
/// the smallest size and working its way back up to the original size
/// of the array
fn merge(left_orig: ~[int], right_orig: ~[int]) -> ~[int] {
    let mut left = left_orig.clone();
    let mut right = right_orig.clone();
    let mut result = vec::from_elem(0, left[0].clone());

    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                result.push(left.shift());
            }
            else {
                result.push(right.shift());
            }
        }
        else if left.len() > 0 { //this is done if the right vector is empty
            result.push(left.shift());
        }
        else { //this is done if the  left vector is empty
            result.push(right.shift());
        }
    }

    return result;
}


fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        println("I need a filename.");
        return;
    }
    let pathname = args[1].to_owned();
    let unsorted_array = common::utils::int_array_from_file(pathname);
    let sorted_array = merge_sort(unsorted_array.clone());
    for &elem in sorted_array.iter() {
        println!("{}",elem);
    }
}
