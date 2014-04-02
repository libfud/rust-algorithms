#[crate_id = "merge_insertion"];
#[crate_type = "bin"];

//! Merge sort with insertion sort.

use std::num::from_uint;

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
    let args = std::os::args();
    let mut min_size = 20; // magic numbers woo (this is a default size)
    
    if args.len() < 2 {
        println("I need a filename.");
        return;
    }
    if args.len() > 2 {
        min_size = match from_str::<int>(args[2].to_owned()) {
            Some(num) => num,
            _         => 20
        }
    }
    println!("Using {} as the minimum length for the arrays.",min_size);
    let pathname = args[1].to_owned();
    let array = common::utils::int_array_from_file(pathname);
    let sorted_array = merge_ins_sort(array.clone(),min_size);
    for &elem in sorted_array.iter() {
        println!("{}",elem);
    }
}
