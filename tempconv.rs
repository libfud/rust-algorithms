#![crate_id = "tempconv"]
#![crate_type = "bin"]

//! Converts decimal integers or floating point numbers of either fahrenheit
//! or celsius to celsius or fahrenheit, respectively.

use common::utils::parse_string_to_float;

pub mod common { pub mod utils; } 

fn fahr_to_cels(fahr: f64) -> f64 {
    return (fahr - 32.0) * 5.0 / 9.0;
}

fn cels_to_fahr(cels: f64) -> f64 {
    return (cels * 9.0 / 5.0) + 32.0;
}

fn main() {

    let args = std::os::args();
    
    let help_str = 
    "Type a number followed by f or F to signify Fahrenheit to convert it to\n"
    +"Celsius. Use c or C to convert to Celsius.";

    if args.len() < 2 {
        println!("{}",help_str);
        return;
    }
    
    let argument = args[1];
    let (temperature, scale) = parse_string_to_float(argument);

    if scale.len() > 0 {
        match scale.slice(0,scale.len()) {
            "F"|"f" => println!("{}C", fahr_to_cels(temperature)),
            "C"|"c" => println!("{}F", cels_to_fahr(temperature)),
            "-h"    => println!("{}", help_str),
            _ => println!("use tempconv -h for help")
        }
    } else {
        println!("I require you to specify fahrenheit or celsius.");
        return;
    }
}
