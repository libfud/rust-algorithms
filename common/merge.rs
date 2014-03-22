use std::vec;

pub fn merge(left_orig: ~[int], right_orig: ~[int]) -> ~[int] {
    let mut left = left_orig.clone();
    let mut right = right_orig.clone();
    let mut result = vec::from_elem(0, left[0].clone());

    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                result.push(left.shift());
            }
            else {
                result.push(right.shift());
            }
        }
        else if left.len() > 0 {
            result.push(left.shift());
        }
        else {
            result.push(right.shift());
        }
    }
    return result;
}
