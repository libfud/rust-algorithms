#[crate_id = "records"];

//! Gathers data for a complex number, a date and a person

use common::records::{ Complex_Number, Date };
use common::utils::{string_getter, float_getter, parse_date};

pub mod common {
    pub mod records;
    pub mod utils;
}

fn main() {
    let real = float_getter("Input a real number.");
    let imaginary = float_getter("Input an imaginary number.");
    let z = Complex_Number{real_num: real, imag_num: imaginary};
    println!("{}, {}",z.real_num,z.imag_num);
    println!("{}",real);
    println!("{}", z.real_num);

    let date_string = string_getter("What is today's date?");
    let (day1, month1, year1) = parse_date(date_string);
    let date = Date {day: day1, month: month1, year: year1};
    println!("The date is {}/{}/{}",date.day,date.month,date.year);

}
