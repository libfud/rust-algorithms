/* Takes a numeric input and finds if there are two values in a sorted
array whose sum is equal to the input. */

pub mod common {
    pub mod utils;              //stuff to get numeric input and create arrays
    pub mod binary_search;      //binary_search returns -1 if nothing is found
}

fn augend_getter(array: ~[int], summed_number: int, array_size: int) -> int {
    let mut augend_index: int = -1;         //guarantees a returned value in case
    let mut augend = summed_number;         
    loop {
        if augend < 0 { break } 
        // if 0 is the sum, two elements holding 0 add to 0, but
        //negative numbers can never add to be >= 0
        augend_index = common::binary_search::binary_search(array.clone(), augend, 0, array_size);
        if augend_index < 0 {
            augend -= 1;                    
            //the augend wasn't found, try to find one with a value one lower
        }
        else if augend_index > 0 { break } //the augend was found
    }
    return augend_index;
}

fn sum_search(array: ~[int], summed_number: int, array_size: int) -> (int, int) {
    let mut augend = summed_number;        
    if augend > array[array_size-1] { augend = array[array_size-1] }
    let mut addend: int; 
    let mut augend_index: int;
    let mut addend_index: int;
    loop {
        augend_index = augend_getter(array.clone(), augend, array_size);
        if augend_index <= 1 || augend < summed_number / 2 {
            augend_index = 1;
            addend_index = 0;
            break
            /* If the augend is less than half the input, then there is no possible way 
            that the addend, which is less than or equal to the augend, plus the augend
            will make a sum equal to the original input. 0 and 1 are generally a failed
            return, but are also the lowest possible elements holding the augend and
            addend valuees*/
            }
        augend = array[augend_index]; 
        /* This helps sets up for the next iteration of the loop
        if the corresponding addend is not found. */
        println!("{} is a possible augend.", augend);
        addend = summed_number - array[augend_index];
        addend_index = common::binary_search::binary_search(array.clone(), addend, 0, augend_index);
        if addend_index < 0 {
            augend -= 1;
            println!("Addend {} not found.",addend);
        }
        if augend_index > 0 && addend_index >= 0 { break } //success
    }
    return (augend_index, addend_index);
}

fn main() {
    let array_size = 100000; 
    let pathname = ~"common/sorted.txt"; 
    //a file with randomly generated values from
    // 1 to 1,000,000 that have been sorted
    let array = common::utils::int_array_from_file(pathname,array_size);
    let mut summed_number: int;
    loop {
        summed_number = common::utils::number_getter("What is the value of the sum?");
        if summed_number > 0 && summed_number <= array[array_size-1] + array[array_size-2]
            && summed_number != array[0] { break }
        else { println("Invalid or impossible sum.") }
    }
    println!("{} is the input.",summed_number);
    let (augend, addend) = sum_search(array.clone(), summed_number, array_size);
    if array[augend] + array[addend] == summed_number { 
        println!("{} and {}, {} + {} = {}", augend, addend, array[augend], array[addend], summed_number);
    }
    else { println!("Fail.") }
}
