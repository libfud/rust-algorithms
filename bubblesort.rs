#[crate_id = "bubblesort"];
#[crate_type = "bin"];

//!An implementation of bubblesort.

use common::utils::int_array_from_file;

pub mod common { pub mod utils; }

fn bubble_sort(array_orig: ~[int]) -> ~[int] {
    let mut array = array_orig.clone();
    let array_size = array.len();
    let mut i = 0;
    while i < array_size {
        let mut j = array_size - 1;
        while j > i {
            if array[j] < array[j - 1] {
               array.swap(j,j-1);
            }
            j -= 1;
        }
        i += 1;
    }
    return array;
}

fn main() {
    
    let args = std::os::args();
    if args.len() < 2 {
        println("I need a filename.");
        return;
    }

    let pathname = args[1].to_owned();
    let array = int_array_from_file(pathname);
    let sorted_array = bubble_sort(array);
    for &elem in sorted_array.iter() {
        println!("{}",elem);
    }
}
