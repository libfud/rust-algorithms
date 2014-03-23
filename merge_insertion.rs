#[crate_id = "merge_insertion"];

//! Merge sort with insertion sort.

use std::num::from_uint;
use common::utils::{string_getter, number_getter};

pub mod common {
    pub mod utils;
    pub mod insertion;
    pub mod merge;
}

fn merge_ins_sort(array: ~[int], min_size: int) -> ~[int] {
//! Takes an array of integers and recursively splits it until it holds
//! the number of elements specified by minimum size. At that point, each
//! array is sorted by insertion sort, and then merged.

   let length = match from_uint::<int>(array.len()) {
        Some(num) => num,
        None => 1
    };
    if length <= min_size {
        return common::insertion::ins_sort(array.clone(),length);
    }

    let middle = array.len() /2 ;
    let mut left = array.slice(0, middle).to_owned();
    let mut right = array.slice(middle, array.len()).to_owned();

    left = merge_ins_sort(left, min_size);
    right = merge_ins_sort(right, min_size);

    common::merge::merge(left, right)
}


fn main() {
//!Requests a minimum size of the subarray for insertion sort, and the name of the
//!file holding the unsorted values.
    let min_size = number_getter("What is the minimum size of the subarrays?");
    let pathname = string_getter("What is the name of the file with unsorted values?");
    let array = common::utils::int_array_from_file(pathname);
    let sorted_array = merge_ins_sort(array.clone(),min_size);
    for &elem in sorted_array.iter() {
        println!("{}",elem);
    }
}
