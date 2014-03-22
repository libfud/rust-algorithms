/*Turns a file with a list of file sizes into a total size */

use common::utils::{float_array_from_file, string_getter, answer_to_bool};
use common::utils::parse_string_to_chars;

pub mod common {
    pub mod utils;
}

fn main() {
    let pathname = ~"common/output2.txt";
    let (array, block_size_array)  = float_array_from_file(pathname);
    let array_size = array.len();
    let mut total: f64 = 0.0;
    let mut i = 0;
    
    while i < array_size {
        let block_size = parse_string_to_chars(block_size_array[i].clone());
        let file_size = match block_size[0] {
            'K' => array[i]/1000.0,
            'M' => array[i],
            'G' => array[i]*1000.0,
            _   => array[i]
        };
        total += file_size;
        i+=1;
    }
    i = 0;
    
    let mut answer = string_getter("Would you like to print a table of the sizes of the files? : y/N");
    let print_tables = answer_to_bool(answer);
    if print_tables {
        while i < array_size - 6 {
            print!("{},\t\t{},\t\t{},\t\t",array[i],array[i+1],array[i+2]);
            println!("{},\t\t{},\t\t{}",array[i+3],array[i+4],array[i+5]);
            i+=6;
        }
        i -= 6;
        while i < array_size {
            print!("{}\t",array[i]);
            i+=1;
        }
        println("");
    }
    answer = string_getter("How do you want the size formatted? Default iS Megabytes. : K/M/G");
    total = match answer {
        ~"K" => total*1000.0,
        ~"M" => total,
        ~"G" => total/1000.0,
        _    => total
    };
    println!("{}",total);
}
