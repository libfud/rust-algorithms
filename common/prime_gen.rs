use std::num::sqrt;

pub fn prime_gen(upper_lim: int) -> ~[int] {
    let mut i: int = 3;
    let mut prime_array = ~[2];
    while i <= upper_lim {
        let mut j = 0;
        let i_flt: f64 = i as f64;
        let sqrt_lim_flt: f64 = sqrt(i_flt);
        let sqrt_lim: int = sqrt_lim_flt as int;
        loop {
            if j >= prime_array.len() || prime_array[j] > sqrt_lim {
                /* This signfies that i is indivisibile by any
                element in the array, and is a prime number. */
                prime_array.push(i);
                break
            }
            if i % prime_array[j] == 0 { break } //non-prime number
            j += 1;
        }
        i+=2;
    }
    return prime_array
}

