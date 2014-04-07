#![crate_id = "linear_search_generic"]
#![crate_type = "bin"]

use common::utils::{array_from_file, linear_search, check_args};
pub mod common { pub mod utils; }


/*fn linear_search<T: Eq>(array: &[T], key: T) -> (bool, uint) {
    let mut found = false;

    if array.len() < 1 { return (found, 0); }

    let mut i: uint = 0;

    while i < array.len() {
        if array[i] == key {
            found = true;
            break;
        } else { i += 1; }
    }

    return (found, i);
}


fn check_args(args_to_check: ~[~str],args_given: ~[~str]) -> (bool, HashMap<~str, uint>) {
    let mut exists = true;                                                    
    let mut args_table = HashMap::new();
    let j: uint = 0;

    if args_to_check.len() < 1 || args_given.len() < 1 {
        args_table.insert(~"nothing", j);
        return (false, args_table);
    }

    for key in args_to_check.iter() {
        let (found, i) = linear_search(args_given.clone(), key.to_owned());
        if found == true && i % 2 > 0 {
            //the first argument is zero; any flag proceeding should be odd
            args_table.insert(key.to_owned(), i);
        } else {
            exists = false;
            break;
        }
    }

    return (exists, args_table);
}  */

fn main() {
    let message = "Please include the following arguments:\n"+
    "-i FILENAME for the file to be searched\n"+
    "-k STRING for the key ";

    let args = std::os::args();
    let args_to_check = ~[~"-k", ~"-i"];
    if args.len() - 1 > args_to_check.len() * 2 {
        println!("{}", message);
    }
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
