#[crate_id = "records"];

//! Gathers data for a complex number, a date and a person

use common::records::{ Complex_Number, Date, Person };
use common::utils::{string_getter, float_getter, parse_date, answer_to_bool};

pub mod common {
    pub mod records;
    pub mod utils;
}

fn main() {
    let real = float_getter("Input a real number.");
    let imaginary = float_getter("Input an imaginary number.");
    let z = Complex_Number{real_num: real, imag_num: imaginary};
    println!("{}, {}",z.real_num,z.imag_num);

    let date_string = string_getter("What is today's date?");
    let (current_day, current_month, current_year) = parse_date(date_string);
    let date = Date {day: current_day, month: current_month, year: current_year};
    println!("The date is {}/{}/{}",date.day,date.month,date.year);

   
    let last_name = string_getter("what is your last name?");
    let given_name = string_getter("What is your first name?");
    
    let dob_string = string_getter("What is your birth date?");
    let (dobday, dobmonth, dobyear) = parse_date(dob_string);
    let dob = Date { day: dobday, month: dobmonth, year: dobyear };
    
    let mut sex_str = string_getter("Are you female? y/n");
    let is_female = answer_to_bool(sex_str);
    let mut married_str = string_getter("Are you married? y/n");
    let is_married = answer_to_bool(married_str);
    
    let you = Person { family_name: last_name, first_name: given_name,
        date_of_birth: dob, Is_Female: is_female, Is_Married: is_married };

    sex_str = match you.Is_Female {
        false => ~"are",
        _     => ~"are not"
    };

    married_str = match you.Is_Married {
        true => ~"are",
        _    => ~"are not"
    };

    let mut age: uint = 0;

    if you.date_of_birth.year <  date.year {
        age = date.year - you.date_of_birth.year;
        if date.month == you.date_of_birth.month {
            if date.day > you.date_of_birth.day { age -= 1; }
        }
        else if date.month < you.date_of_birth.month { age -= 1; }
    }
    
    println!("Your name is {} {}. You were born on {}/{} in {}. You {} male. You {} married.",
        you.first_name, you.family_name, you.date_of_birth.day, you.date_of_birth.month,
        you.date_of_birth.year, sex_str, married_str);
    println!("You are {} years old.", age);

}
