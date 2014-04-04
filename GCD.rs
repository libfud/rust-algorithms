use common::utils::string_getter;

pub mod common { pub mod utils; }

fn GCD(x: int, y: int) -> int {
    let mut divisor: int;
    let mut gcd;
    if x < y { divisor = x }
    else { divisor = y }
    loop {
        if x%divisor != 0 || y%divisor != 0 { 
            divisor -= 1;
        }
        else { gcd = divisor; break }
    }
    return gcd;
}

fn main() {
    let (mut x, mut y): (int, int);
    loop {
        x = match from_str::<int>(string_getter("What is the value of the first"
        +" number?")) {
            Some(num) => num,
            _         => 0,
        };
        if x > 1 { break }
        else { println!("No, a number greater than 1."); }
    } 
    loop {
        y = match from_str::<int>(string_getter("What is the next number?")) {
            Some(num) => num,
            _         => 0,
        };
        if y > 1 { break }
        else { println!("No, give me something greater than 1."); }
   }
   let gcd = GCD(x, y);
   println!("The GCD of {} and {} is {}",x,y,gcd);
}
