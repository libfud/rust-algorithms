/*simple summation tool*/

use std::int;
use std::io;
use std::io::buffered::BufferedReader;

fn main() {

    loop {

        println!("Enter your command.");
    
        let mut reader = BufferedReader::new(io::stdin());
        let input : &str  = reader.read_line().unwrap_or(~"nothing");

        match input {
            "exit\n" => break,
            "sum of i\n" => {
                let x = number_getter();
                println!("The sum is {}",sum_of_i(x));
            },
            "sum of i^2\n" => {
                let x = number_getter();
                println!("The sum is {}",sum_of_i2(x));
                },
            "sum of i^3\n" => { 
                let x = number_getter();
                println!("The sum is {}",sum_of_i3(x));
                },
            "sum of i^4\n" => {
                let x = number_getter();
                println!("The sum is {}",sum_of_i4(x));
                },
            "sum of i^5\n" => { 
                let x = number_getter();
                println!("The sum is {}",sum_of_i5(x));
                },
            "sum of i^6\n" => { 
                let x = number_getter();
                println!("The sum is {}",sum_of_i6(x))
                },
            "sum of i^7\n" => {
                let x = number_getter();
                println!("The sum is {}",sum_of_i7(x));
                },
            "sum of i^8\n" => {
                let x = number_getter();
                println!("The sum is {}",sum_of_i8(x));
                },
            "sum of i^9\n" => {
                let x = number_getter();
                println!("The sum is {}",sum_of_i9(x));
                },
            "sum of i^10\n" => {
                let x = number_getter();
                println!("The sum is {}",sum_of_i10(x));
                }, 

            _      => println!("what?")
        }
    }
}

fn number_getter() -> int {
    println("What number?");
        let mut number = BufferedReader::new(io::stdin());
        let number : &str = number.read_line().unwrap_or(~"What?");
        let num = from_str::<int>(number.slice_to(number.len() - 1));
        match num {
            Some(0) => return 0,
            Some(num) => return num,
            None => return 0
        }
}
 
fn sum_of_i(n: int) -> int {
    (n*(n+1))/2
}

fn sum_of_i2(n: int) -> int {
    (n*(n+1)*(2*n+1))/6
}

fn sum_of_i3(n: int) -> int {
    (int::pow(n, 2) * int::pow((n+1), 2))/4
}

fn sum_of_i4(n: int) -> int {
    (n*(2*n+1)*(n+1)*(3*n*n+3*n-1))/30
}

fn sum_of_i5(n: int) -> int {
    (n*n*(2*n*n+2*n-1)*(n+1)*(n+1))/12
}

fn sum_of_i6(n: int) -> int {
    (n*(2*n+1)*(n+1)*(3*int::pow(n,4)+6*int::pow(n,3)-3*n-1))/42
}

fn sum_of_i7(n: int) -> int {
    (n*n * (3*int::pow(n,4)+6*int::pow(n,3)-n*n-4*n+2)*(n+1)*(n+1))/24
}

fn sum_of_i8(n: int) -> int {
    (n *(2*n+1)*(n+1)*(5*int::pow(n,6)+15*int::pow(n,5)+5*int::pow(n,4)-15*int::pow(n,3)-n*n+9*n-3))/90
}

fn sum_of_i9(n: int) -> int {
    (n*n*(n*n+n-1)*(2*int::pow(n,4)+4*int::pow(n,3)-n*n-3*n+3)*int::pow((n+1),2))/20
}

fn sum_of_i10(n: int) -> int {
    (n*(2*n+1)*(n+1)*(n*n+n-1)*(3*int::pow(n,6)+9*int::pow(n,5)+2*int::pow(n,4)-11*int::pow(n,3)+3*n*n+10*n-5))/66
}
