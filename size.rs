#[crate_id = "size"];

/// Turns a file with a list of file sizes into a total size

use common::utils::{float_array_from_file, string_getter, answer_to_bool};
use common::utils::parse_string_to_chars;

pub mod common { pub mod utils; }

fn main() {
    
    let args = std::os::args();
    let mut pathname: ~str;
    if args.len() > 1 {
        pathname = args[1];
    }
    else {
        pathname = string_getter("What is the name of the file?");
    }
    let (array, block_size_array)  = float_array_from_file(pathname);
    // The function float_array_from_file uses parse_string_to_float, which
    // when used as intended splits a string into a floating point number and
    // the remainder of the string. Since we're using this on a file which lists
    // files using different sizes (KB, MB, GB...) We'll need the rest of the
    // string to determine what size each file listed is
    let array_size = array.len();
    let mut total: f64 = 0.0;
    let mut i = 0;
    
    while i < array_size {
        let block_size = parse_string_to_chars(block_size_array[i].clone());
        /*String manipulation in rust is kind of a chore, so I prefer to
        convert strings to char arrays when doing in this kind of thing.
        This is not very efficient and rather lazy on my part, until I
        figure out a proper method of doing this. */
        let file_size = match block_size[0] {
            'K' => array[i]/1000.0,
            'M' => array[i],
            'G' => array[i]*1000.0,
            _   => array[i] //I could take out the M case, but I think it
            //preserves intentions by leaving it in, if I add other cases.
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
