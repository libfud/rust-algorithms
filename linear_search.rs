#[crate_id = "linear_search"];
#[crate_type = "bin"];

//!An implementation of a linear search.

pub mod common {
    pub mod utils;
}

fn main() {
    let mut array_size = common::utils::number_getter("What is the number of "+
    "elements in the array?");
    if array_size < 1 {
        println!("Using 10.");
        array_size = 10;
    }
    let mut upper_bound = common::utils::number_getter("Input a factor to "+
    "determine the largest number possible in the array.");
    if upper_bound < 1 { upper_bound = 10; }
    let array = common::utils::array_gen(array_size,upper_bound);

    loop {
        let z = common::utils::number_getter("Input a number to search for "
        +"between 1 and the factor times the number of elements in the array.");
        if (z > 0 && z < (10*array_size+1)) { y = z; break; }
    }

    println!("{}",array[array_size-1]);
    
    let mut i = 0;
    while i < array_size-1 {
        if y == array[i] { break }
        println!("{}",array[i]);
        i += 1;
    }
    
    if array[i]== y {
        println!("Array index {} holds value {} equal to the supplied value of {}", i, array[i], y);
    }
    else { println("Sorry, didn't find it."); }
}
