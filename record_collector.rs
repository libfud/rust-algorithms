#[crate_id = "records"];

//! Gathers data for a complex number, a date and a person

use common::records::Complex_Number;

pub mod common {
    pub mod records;
    pub mod utils;
}

fn main() {
    let mut real = common::utils::float_getter("Input a real number.");
    let imaginary = common::utils::float_getter("Input an imaginary number.");
    let z = Complex_Number{real_num: real, imag_num: imaginary};
    println!("{}, {}",z.real_num,z.imag_num);
    real = 7.5;
    println!("{}",real);
    println!("{}", z.real_num);
}
