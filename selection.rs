#[crate_id = "selection"];
#[crate_type = "bin"];

//!Selection sort implementation.

pub mod common { pub mod utils; }

fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        println("I need a filename.");
        return;
    }
    let pathname = args[1].to_owned();
    let mut array = common::utils::int_array_from_file(pathname);
    let mut i = 0;
    while i < (array.len() -1) {  // The outer loop will iterate through each element in the array.
        let mut j = i; //j will be used for the inner loop
        let mut Min_index = j; //Min_index will be the index of the smallest number
        while j < array.len() -1 {
            j += 1;
            if array[j] < array[Min_index] { Min_index = j; } 
             //gotta ensure that you find the smallest number and update the index.
        }
        if array[Min_index] < array[i] {
            let holder = array[i];
            array[i] = array[Min_index];
            array[Min_index] = holder;
            //reassignments done with a placeholder
        }
        i+=1;
    }
    for &elem in array.iter(){
        println!("{}",elem);
    }
}
