#![crate_id = "merge_sort"]
#![crate_type = "bin"]

//! An implementation of merge sort.

pub mod common{ 
    pub mod utils;
    pub mod sort;
}

fn main() {
    let args = std::os::args();
    let mut min_size = 1;
    if args.len() < 2 {
        println!("I need a filename.");
        return;
    }
    if args.len() > 2 {
        min_size = match from_str::<uint>(args[2]) {
            Some(num)   => num,
            _           => 1
        };
    }
    let pathname = args[1].to_owned();
    let unsorted_array = common::utils::int_array_from_file(pathname);
    let sorted_array = common::sort::merge_sort(unsorted_array.clone(), 
        min_size);

    for elem in sorted_array.iter() {
        println!("{}", elem);
    }

}
