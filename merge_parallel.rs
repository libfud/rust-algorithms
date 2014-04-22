#![crate_id = "Parallel_merge"]
#![crate_type = "bin"]

//! Parallel merge sort, using the green library to get scheduled threads.

extern crate getopts;
extern crate green;
use green::{SchedPool, PoolConfig};
use green::sched::PinnedTask;
use getopts::{reqopt, optflag, getopts, OptGroup};
use std::os;
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
pub fn merge_sort_parallel<T: Clone + Ord + Send>(mut array: ~[T]) -> ~[T] {

    let config = PoolConfig::new();
    let mut pool = SchedPool::new(config);
    // setting up the scheduling pool for handles to threads

    let array_refs = split(array);
    let mut sorted_subarrays = ~[~[array[0].clone()]];
    // initializes the sorted_split array
    sorted_subarrays.shift();
    //remove that initializer value
    let length = array_refs.len();

    let (tx, rx): (Sender<~[T]>, Receiver<~[T]>) = channel();
    // Threads need a sender and a receiver in a channel to communicate
    
    let mut i = 0;

    while i < length {
        let mut handle = pool.spawn_sched();
        //create a new handle
        let (subarray_begin, subarray_end) = array_refs[0];
        let subarray_unsort = array.slice(subarray_begin,
            subarray_end + 1).to_owned();
        let child_tx = tx.clone();
        // you can't overload one sending channel
        let task = pool.task(TaskOpts::new(), proc() {
            //sets up a task for the handle
            let subarray = insertion_sort(subarray_unsort);
            //let subarray_refs = (sorted_sub[0], sorted_sub[sorted_sub.len() - 1]);
            child_tx.send(subarray);
            // sends the sorted sub array to rx
        });
        handle.send(PinnedTask(task));
        // the handle takes the task
        drop(handle);
        // the handle is destroyed after doing its job
        sorted_subarrays.push(rx.recv());
        // get the sorted sub array
        i += 1;
    }
    
    pool.shutdown();
    // the pool must shutdown after it has done its duty
    let sorted_array_wrapped = merge_wrapper(sorted_subarrays);
    array = sorted_array_wrapped[0];
    // merge wrapper returns ~[~[T]]

    return array;
}

/// merge_wrapper will repeatedly merge arrays in an array until there
/// is only one remaining element. Takes an array of arrays of type T which 
/// has the traits Clone and Ord, and returns a single array in an array.
pub fn merge_wrapper<T: Ord + Clone + Send>(mut array_array: ~[~[T]]) -> ~[~[T]] {
    let array_n = array_array.len();
    if array_n == 1 { return array_array.to_owned() }

    let config = PoolConfig::new();
    let mut pool = SchedPool::new(config);

    let (tx, rx): (Sender<~[T]>, Receiver<~[T]>) = channel();

    let mut i = 0;
    let mut j = 0;
    while i < array_n - 1 {
        let mut handle = pool.spawn_sched();
        let sub_array1 = array_array[i].clone();
        let sub_array2 = array_array[i + 1].clone();
        let child_tx = tx.clone();
        let task = pool.task(TaskOpts::new(), proc() {
            let merged_sub = merge(sub_array1.clone(), sub_array2.clone());
            child_tx.send(merged_sub);
        });
        handle.send(PinnedTask(task));
        drop(handle);
        array_array[j] = rx.recv();
        i += 2;
        j += 1;
    }

    pool.shutdown();

    array_array = array_array.slice(0, j).to_owned();

    return merge_wrapper(array_array);
}

/// Takes an array of type T which can be Cloned, and the number of subarrays
/// for it to be broken into. Returns an array of tuples of uints for the array
/// to be split into.
pub fn split<T: Clone>(array: &[T]) -> ~[(uint, uint)] {

    let mut number_of_sub_arrays = 1;
    loop {
        if array.len() % (number_of_sub_arrays * 2) != 0 { break } 
        if number_of_sub_arrays > 256 { break }
        number_of_sub_arrays *= 2;
    }

    let mut i = 0;
    let length = array.len();

    let min_size = length / number_of_sub_arrays;

    let mut split_array: ~[(uint, uint)] = ~[(0, 0)];
    //initializes array

    while i + min_size < length {
        let sub_array2 = i + min_size - 1;
        split_array.push((i, sub_array2));
        i += min_size;
    }

    split_array.push((i, length - 1));
    split_array.shift();
    //removes first value that initialized split_array

    return split_array;
}

/// prints a help message
pub fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-f ,\t--file\t\t Input file");
}

fn main() {

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
        _               => ~"Invalid"
    };
    if input_filename == ~"Invalid" {
        println!("Invalid paramater.");
        return;
    }

    let array = number_array_from_file(input_filename, 0i);
    let sorted_array = merge_sort_parallel(array.clone());

    for elem in sorted_array.iter() {
        println!("{}", elem);
    }
    println!("\n{}", sorted_array.len());

}
