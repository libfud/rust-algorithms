#![crate_id = "linear_search_generic"]
#![crate_type = "bin"]

use common::utils::{array_from_file, linear_search, check_args};
pub mod common { pub mod utils; }

fn main() {
    let message = "Please include the following arguments:\n"+
    "-i FILENAME for the file to be searched\n"+
    "-k STRING for the key ";

    let args = std::os::args();
    let args_to_check = ~[~"-k", ~"-i"];
    let (args_found, args_table) = check_args(args_to_check.clone(), args.clone());
    
    if args_found == false { 
        println!("{}",message);
        return;
    }

    let search_array_index = args_table.get(& ~"-i") + 1;
    let search_array_path = args[search_array_index].clone();
    let mut search_array = array_from_file(search_array_path);
    let mut i = 0;
    while i < search_array.len() {
        search_array[i] = search_array[i].slice_to(search_array[i].len() - 1).to_owned();
        i += 1;
    }

    let key_index = args_table.get(& ~"-k") + 1;
    let key = args[key_index].clone();

    let (found, at_index) = linear_search(search_array.clone(), key.clone());
    println!("{} {}", found, at_index);

}
