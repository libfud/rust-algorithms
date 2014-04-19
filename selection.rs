#![crate_id = "selection"]
#![crate_type = "bin"]

//!Selection sort implementation.

pub mod common { 
    pub mod utils;
    pub mod sort;
}

fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        println!("I need a filename.");
        return;
    }
    let pathname = args[1].to_owned();
    let array = common::utils::int_array_from_file(pathname);
    /*
    let mut i = 0;
    while i < (array.len() -1) {    // The outer loop will iterate through 
                                    //each element in the array.
        let mut j = i; //j will be used for the inner loop
        let mut min_index = j;  //min_index will be the index of the
                                //smallest number
        while j < array.len() -1 {
            j += 1;
            if array[j] < array[min_index] { min_index = j; } 
             //ensure that you find the smallest number and update the index.
        }
        if array[min_index] < array[i] {
            array.swap(i, min_index);
            //reassignments done with a placeholder
        }
        i+=1;
    }
    */
    let array_sorted = common::sort::selection_sort(array);
    for &elem in array_sorted.iter(){
        println!("{}",elem);
    }
}
