#![crate_id = "linear_search"]
#![crate_type = "bin"]

//!An implementation of a linear search.

use common::utils::string_getter;

pub mod common {
    pub mod utils;
}

fn main() {
    let array_size = match from_str::<uint>(string_getter("What is the number of "+
    "elements in the array?")) {
        Some(num) => num,
        _         => 10, //default size
    };
    let factor = match from_str::<uint>(string_getter("What is the factor by "+
    "which you will use for the range?")) {
        Some(num) => num,
        _         => 2, //default factor
    };

    let guess = match from_str::<uint>(string_getter("What is the number" +
    " you are looking for?")) {
        Some(num) => num,
        _         => 1,
    };

    let array = common::utils::array_gen(array_size, factor);

    let mut i = 0;

    loop {
        if i == array_size - 1 { break }
        if array[i] == guess { break }
        i += 1;
    }

    if array[i] == guess {
        println!("Array element {} holds {}, which equals {}",
            i, array[i], guess);
    }
    else { println!("Did not find it."); }
}
