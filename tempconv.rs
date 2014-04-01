#[crate_id = "tempconv"];
#[crate_type = "bin"];

//! Converts decimal integers or floating point numbers of either fahrenheit
//! or celsius to celsius or fahrenheit, respectively.

use common::utils::parse_string_to_float;

pub mod common { pub mod utils; } 

fn fahr_to_cels(fahr: f64) -> f64 {
    return (fahr - 32.0) * 5.0 / 9.0;
}

fn cels_to_fahr(cels: f64) -> f64 {
    return cels * 5.0 / 9.0 + 32.0;
}

fn main() {
    let args = std::os::args();
    if args.len() < 3 { 
        println(":^o");
        return;
    }

    let (temperature, scale) = parse_string_to_float(args[2].to_owned());

    match args[1].to_owned() {
        ~"-f"|~"f" => println!("{}", fahr_to_cels(temperature)),
        _ => println("Woops")
    }
}
