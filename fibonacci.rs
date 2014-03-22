/*Returns the nth element of the fibonacci sequence.*/

pub mod common { pub mod utils; }

fn fib ( nth_elem: int ) -> ~[f64] {
    assert!(nth_elem >= 2);
    let mut array = ~[0.0, 1.0];
    let mut current_elem = 2;       //the third element 
    while current_elem <= nth_elem {
        array.push(array[current_elem - 2] + array[current_elem - 1]);
        current_elem += 1;
    }
    return array
}

fn main() {
    let mut nth_elem: int;
    loop {
        nth_elem = common::utils::number_getter("Input the desired number of the fibonacci sequence");
        if nth_elem >= 2 { break }
    }
    let array = fib(nth_elem);
    let fib_nth = array[nth_elem];
    let fib_penult = array[nth_elem-1];
    println!("The {}th number in the fibonacci sequence is {}.",nth_elem,fib_nth);
    println!("{} / {} = {}, and {} / {} = {}.",fib_nth,fib_penult,fib_nth/fib_penult,
        fib_penult,fib_nth,fib_penult/fib_nth);
}
