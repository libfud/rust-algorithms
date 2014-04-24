//! Sorting functions

/// Insertion sort takes an array of type T which has the Ord and Clone
/// traits. Returns an array of type T that has been sorted. It has two 
/// loops, one nested in the other. The outer loop sets an immutable 
/// variable, val, to the value of the ith index and sets a mutable variable 
/// j to the value of i - 1. The inner loop executes while j +1 != 0
/// the length of the array and while the value of the jth element is 
/// greater than the value of val, and sets the value of the element to
/// the "right" of j to the value of the jth element. j is then decremented.
/// When j is equal to zero and decrments, it will fulfill the first
/// termination condition of the inner while loop. After the termination
/// of the inner loop, the element to the right of the jth element is
/// assigned the value of val, and i increments.
pub fn insertion_sort<T: Ord + Clone>(array_orig: Vec<T>) -> Vec<T> {
    if array_orig.len() <= 1 { return array_orig }

    let mut array = array_orig.clone();
    let mut i = 1u;
    while i < array.len() {
        let val = array.as_slice()[i].clone();
        let mut j = i - 1;
        while j + 1 != 0 && array.as_slice()[j] > val {
            array.as_mut_slice()[j+1] = array.as_slice()[j].clone();
            j -= 1;
        }
        array.as_mut_slice()[j+1] = val; //done because of the loop termination
        i += 1;
    }

    return array
}

/// Selection sort takes an array of type T which has the Ord and Clone 
/// traits. Returns an array of type T that has been sorted. It has nested
/// loops: the outer loop begins with the first element of the array,
/// while the inner loop compares each element, starting with the ith,
/// to the element next to it. If the element is smaller, then that index
/// is marked as the smallest, until either another smaller element is found
/// or the loop terminates with the last element. After the inner loop
/// terminates, the values of the ith index and the index holding the
/// smallest element are swapped, provided they actually differ. i is then
/// incremented, and the outer loop proceeds in this fashion until every
/// element has been examined.
pub fn selection_sort<T: Ord + Clone>(array_orig: Vec<T>) -> Vec<T> {
    let mut array = array_orig.clone();
    let mut i = 0;

    while i < array.len() - 1 {
        let mut j = i;
        let mut min_index = j;

        while j < array.len() - 1 {
            j += 1;
            if array.as_slice()[j] < array.as_slice()[min_index] {
                min_index = j
            }
        }

        if array.as_slice()[min_index] < array.as_slice()[i] {
            let placeholder = array.as_slice()[i].clone();
            let second_holder = array.as_slice()[min_index].clone();
            array.as_mut_slice()[i] = second_holder;
            array.as_mut_slice()[min_index] = placeholder;
        }

        i += 1;
    }

    return array
}

/// Merge sort with insertion sort. If you want to use plain merge sort,
/// pass 1 as an argument for the minimum size. Takes an array of type T
/// which implements Ord and Clone traits, and returns an array of type T.
/// It recursively slices the given array to two arrays of half that size,
/// until those arrays are of the minimum length, then calls insertion_sort
/// to sort those arrays. Those sorted arrays are then merged until an
/// array of equal length to the original is returned. So if you pass
/// an array with 128 elements and pass 32 as min_size, it'll split it
/// into two arrays holding 64 elements, then the first array will be split
/// into two arrays holding 32 elements, then those arrays will be sorted
/// with insertion sort, and merged. This will be repeated for the other
/// array holding 64 elements, and then these two sorted arrays will
/// be merged and returned.
pub fn merge_sort<T: Ord + Clone>(array: Vec<T>, mut min_size: uint) -> Vec<T> {
    if min_size < 1 {
        println!("{} is an invalid minimum size. Using 1.", min_size);
        min_size = 1;
    }
    let length = array.len();
    if length <= min_size { return insertion_sort(array) }

    let middle = length / 2;
    let mut left = Vec::from_slice(array.slice(0, middle));
    let mut right = Vec::from_slice(array.slice(middle, length));

    left = merge_sort(left, min_size);
    right = merge_sort(right, min_size);

    return merge(left, right)
}

/// Merge takes two arrays of type T which implements the traits Ord and
/// Clone, and returns an array of a size equal to the sum of the sizes
/// of the arrays passed to it. While each array holds elements, the first
/// element from each is compared, and the lesser of the two will be
/// pushed onto a result array, and the first element will shifted from
/// the array. Else, whichever array holds elements will successively have
/// its first element pushed onto the result and be shifted. Note that
/// this behavior is dependent on the arrays already being sorted, either
/// by previous merges or by another method of sorting.
pub fn merge<T: Ord + Clone>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {

    let mut result: Vec<T> = Vec::new();

    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left.as_slice()[0] < right.as_slice()[0] { 
                result.push(left.as_slice()[0].clone());
                left.shift();
            } else { 
                result.push(right.as_slice()[0].clone());
                right.shift();
            }
        } else if  left.len() > 0 { 
            result.push(left.as_slice()[0].clone());
            left.shift();
        } else {
            result.push(right.as_slice()[0].clone());
            right.shift();
        }
    }

    return result
}
