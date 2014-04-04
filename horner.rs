/*Something I don't really understand yet*/

pub mod common { pub mod utils; }

fn main() {
    let mut array: ~[int] = ~[];
    let mut n: int;
    loop {
        n = common::utils::number_getter("How many coefficients will there be?");
        if n != 0 {break}
    }
    let mut x: int;
    loop {
        x = common::utils::number_getter("What will the value of x be?");
        if x != 0 {break}
    }
    let mut y = 0;
    let mut i = 0;
    while i < n {
        i+=1;
        array.push(i);
    }
    assert!(i == n);

    let mut j: uint = i as uint;

    while j > 0 {
        y = array[j-1] + x * y;
        j -= 1;
    }

    println!("y = {}", y);
}
