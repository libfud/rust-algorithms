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
    if args.len() < 2 { 
        println(":^o");
        return;
    }

    let (temperature, scale) = parse_string_to_float(args[1].to_owned());

    match scale.slice_to(1){
        "F"|"f" => println!("{}C", fahr_to_cels(temperature)),
        "C"|"c" => println!("{}F", cels_to_fahr(temperature)),
        _ => println("Woops")
    }
}
