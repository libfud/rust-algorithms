/* merge sort */

use std::vec;

pub mod common{ pub mod utils; }

fn merge_sort(array: ~[int]) -> ~[int] {
    let length = array.len();
    if length <= 1 {
        return array.to_owned();
    } //returns the value of the array  to whatever is trying to assign value to it, see below

    let middle = length / 2;
    let mut left = array.slice(0, middle).to_owned();
    let mut right = array.slice(middle, length).to_owned();

    //split the array into two halves

    left = merge_sort(left);
    right = merge_sort(right);

    //split the arrays recursively until their length is <= 1 with return array.to_owned()

    merge(left, right)

}

fn merge(left_orig: ~[int], right_orig: ~[int]) -> ~[int] {
    let mut left = left_orig.clone();
    let mut right = right_orig.clone();
    let mut result = vec::from_elem(0, left[0].clone());
    //clones the input vectors, and creates a new vector from the first element of the left vector


    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                result.push(left.shift());
                //shift removes the first element from a vector and returns it
            }
            else {
                result.push(right.shift());
            }
        }
        else if left.len() > 0 { //this is done if the right vector is empty
            result.push(left.shift());
        }
        else { //this is done if the  left vector is empty
            result.push(right.shift());
        }
    }

    return result;
}


fn main() {
    let x = 100000;
    let pathname = ~"common/numbers.txt";
    let array2 = common::utils::int_array_from_file(pathname,x);
    let array = merge_sort(array2.clone());
    for &elem in array.iter() {
        println!("{}",elem);
    }
    let mut y = 0;
    while y < x {
        println!("{}",array[y]);
        y+=1;
    }
}
