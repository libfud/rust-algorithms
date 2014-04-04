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
        x = common::utils::number_getter("What is the value of the first number?");
        if x > 1 { break }
        else { println!("No, a number greater than 1."); }
    } 
    loop {
        y = common::utils::number_getter("What is the value of the next number?");
        if y > 1 { break }
        else { println!("No, give me something greater than 1."); }
   }
   let gcd = GCD(x, y);
   println!("The GCD of {} and {} is {}",x,y,gcd);
}
