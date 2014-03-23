//! Implementations of record types as found in Niklaus Wirth's Algorithms and Data Structures

/// A complex number is composed of a real number and an imaginary number
pub struct Complex_Number {
    real_num: f64,
    imag_num: f64
}

/// A date is composed of an integer day, month and year.
pub struct Date {
    day: uint,
    month: uint,
    year: uint
}

/// A person's record is composed of their family name, first name,
/// date of birth, their sex and marital status
pub struct Person {
    family_name   : ~str,
    first_name    : ~str,
    date_of_birth : Date,
    Is_Female     : bool,
    Is_Married    : bool
}
