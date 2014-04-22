#![crate_id = "Parallel_merge"]
#![crate_type = "bin"]

//! Parallel merge sort, using the green library to get scheduled threads.

extern crate getopts;
extern crate green;
use green::{SchedPool, PoolConfig};
use green::sched::PinnedTask;
use getopts::{reqopt, optflag, getopts, OptGroup};
use std::os;
use std::fmt::Show;
use std::task::TaskOpts;
use common::utils::number_array_from_file;
use common::sort::{insertion_sort, merge};


pub mod common {
    pub mod utils;
    pub mod sort;
}

/// Takes an array of type T which is cloneable (Clone), can be compared with
/// binary operators like > and < (Ord), is sendable for channels (Send), and
/// showable with println! (Show). Also takes a number of sub arrays as a power
/// of two to split the array into. Returns a sorted array of the same type T.
pub fn merge_sort_parallel<T: Clone + Ord + Send + Show>(array: ~[T], 
    number_of_sub_arrays: uint) -> ~[T] {

    let config = PoolConfig::new();
    let mut pool = SchedPool::new(config);
    // setting up the scheduling pool for handles to threads

    let split_array = split(array, number_of_sub_arrays);
    let mut sorted_split = split_array.clone();
    // initializes the sorted_split array
    let split_len = split_array.len();

    let (tx, rx): (Sender<~[T]>, Receiver<~[T]>) = channel();
    // Threads need a sender and a receiver in a channel to communicate
    
    let mut i = 0;

    while i < split_len {
        let mut handle = pool.spawn_sched();
        //create a new handle
        let split_sub = split_array[i].clone();
        let child_tx = tx.clone();
        // you can't overload one sending channel
        let task = pool.task(TaskOpts::new(), proc() {
            //sets up a task for the handle
            child_tx.send(insertion_sort(split_sub));
            // sends the sorted sub array to rx
        });
        handle.send(PinnedTask(task));
        // the handle takes the task
        drop(handle);
        // the handle is destroyed after doing its job
        sorted_split[i] = rx.recv();
        // get the sorted sub array
        i += 1;
    }
    
    pool.shutdown();
    // the pool must shutdown after it has done its duty

    let sorted_array_wrapped = merge_wrapper(sorted_split.clone());
    let sorted_array = sorted_array_wrapped[0];
    // merge wrapper returns ~[~[T]]

    return sorted_array;
}

pub fn split<T: Clone>(array: ~[T], number_of_sub_arrays: uint) -> ~[~[T]] {

    let mut i = 0;
    let length = array.len();

    let min_size = length / number_of_sub_arrays;
    let mut split_array: ~[~[T]] = ~[array.clone()];
    //initializess split_array

    while i + min_size < length - 1 {
        let subarray = array.slice(i, i + min_size);
        split_array.push(subarray.to_owned());
        i += min_size;
    }

    split_array.push(array.slice(i - min_size, length).to_owned());
    split_array.shift();
    //removes first value that initialized split_array


    return split_array;
}

pub fn merge_wrapper<T: Ord + Clone + Send>(array_array: ~[~[T]]) -> ~[~[T]] {
    let array_n = array_array.len();
    if array_n == 1 { return array_array.to_owned() }

    let mut array_clone = array_array.slice(0, array_n / 2).to_owned();

    let mut i = 0;
    let mut j = 0;
    while i < array_n - 1 {
        let sub_array1 = array_array[i].clone();
        let sub_array2 = array_array[i + 1].clone();
        array_clone[j] = merge(sub_array1.clone(), sub_array2.clone());
        i += 2;
        j += 1;
    }

    return merge_wrapper(array_clone);
}
pub fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-f ,\t--file\t\t Input file");
}

fn main() {

    let bs = ~"INVALID"; // bs == bad string 
    let args = os::args();
    let program = args[0].clone();
    let opts = ~[
        reqopt("f", "file", "input file name", "FILENAME"),
        optflag("h", "help", "print this help message")
    ];
    
    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => { m }
        Err(f)  => { fail!(f.to_err_msg()) }
    }; 
    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    }

    let input_filename = match matches.opt_str("f") {
        Some(string)    => string,
        _               => bs.clone()
    };

    if input_filename == bs {
        println!("Invalid paramater.");
        return;
    }
    let array = number_array_from_file(input_filename, 0i);
    
    let mut n = 2;
    while array.len() / n > 400 {
        n *= 2;
    }

    let sorted_array = merge_sort_parallel(array.clone(), n);

    for elem in sorted_array.iter() {
        println!("{}", elem);
    }

}
