pub mod common { pub mod utils; }

fn main() {
    let x = 100000;
    let pathname = ~"common/numbers.txt";
    let mut array = common::utils::array_from_file(pathname,x);
    for &elem in array.iter(){
        println!("{}",elem);
    } // To see how they look unsorted.
    let mut i = 0;
    while i < (x-1) {  // The outer loop will iterate through each element in the array.
        let mut j = i; //j will be used for the inner loop
        let mut Min_index = j; //Min_index will be the index of the smallest number
        while j < x-1 {
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
    println!("\n\n\nThe sorted numbers:");
    for &elem in array.iter(){
        println!("{}",elem);
    }
}
