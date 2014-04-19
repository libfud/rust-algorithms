#![crate_id = "fibonacci"]
#![crate_type = "bin"]

//! Returns the nth element of the fibonacci sequence.

pub mod common { pub mod utils; }

///Creates an array with elements that are from the fibonacci
/// sequence. Takes an argument for how many elements to generate.
pub fn fib ( nth_elem: uint ) -> ~[u64] {
    assert!(nth_elem >= 2);
    let mut array = ~[0, 1];
    let mut current_elem = 2;       //the third element 
    while current_elem <= nth_elem {
        array.push(array[current_elem - 2] + array[current_elem - 1]);
        current_elem += 1;
    }
    return array
}

pub fn main() {
    
    let args = std::os::args();

    let mut nth_elem: uint = 10; //Dummy for no arguments.
    
    if args.len() > 1 {
        nth_elem = match from_str::<uint>(args[1].to_owned()) {
            Some(num) => num,
            _         => 10
        };
    } else {
        println!("Generating {} numbers in the fibonacci sequence.",nth_elem);
    }
    if nth_elem > 93 {
        println!("{} is too big, using 93.",nth_elem);
        nth_elem = 93;
    }

    let array = fib(nth_elem);
    let fib_nth = array[nth_elem];
    println!("The {}th number in the fibonacci sequence is {}.",nth_elem,fib_nth);
}
