#![crate_id = "insertion_sort"]
#![crate_type = "bin"]
//!Insertion sort

pub mod common { 
    pub mod utils;
    pub mod sort;
}

fn main() {
    let pathname = ~"common/numbers.txt";
    let array = common::utils::int_array_from_file(pathname);

/*    let mut i = 0;
    while i < array.len() {
        let val = array[i];
        let mut j = i - 1;
        while j < array.len() && array[j] > val {
            array[j+1] = array[j];
            j -= 1;
        }
        array[j+1] = val;
        i+=1;
    }
*/

    let sorted_array = common::sort::insertion_sort(array);
    println!("{}", sorted_array[sorted_array.len() - 1]);
}
