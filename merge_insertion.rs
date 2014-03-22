use std::num::from_uint;

pub mod common {
    pub mod utils;
    pub mod insertion;
    pub mod merge;
}

fn merge_ins_sort(array: ~[int], min_size: int) -> ~[int] {
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
    let array_size = 100000;
    let min_size = 150;
    let pathname = ~"common/numbers.txt";
    let array = common::utils::int_array_from_file(pathname, array_size);
    let sorted_array = merge_ins_sort(array.clone(),min_size);
    for &elem in sorted_array.iter() {
        println!("{}",elem);
    }
}
