/* Decomposes a number into its prime factors.*/

pub mod common { pub mod utils; }

fn decomp(x_orig: int) -> ~[int] {
    let mut x = x_orig;
    let mut factor_array: ~[int] = ~[];
    let mut i = 3;
    loop {
        if x == 1 { break }
        if i > x_orig / 2 { 
            factor_array.push(x);
            break
        }
        if x%2 == 0 { 
            factor_array.push(2);
            x /= 2
        }
        else if x%i == 0 {
            factor_array.push(i);
            x /= i;
        }
        else { i += 2 }
    }
    return factor_array
}

fn main() {
    let mut x: int;
    loop {
        x = common::utils::number_getter("What is the number to decompose?");
        if x > 3 { break }
        else { println("2 and 3 are prime numbers, dummy."); }
    }
    let factor_array = decomp(x);
    for &elem in factor_array.iter() {
        print!("{}  ",elem);
    }
    println("");
}
