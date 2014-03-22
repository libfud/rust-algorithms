/*binary search implementation*/

pub fn binary_search(array: ~[int], search_value: int, first_elem: int, last_elem: int) -> int {
    if last_elem - first_elem <= 0 { return -1 }
    else { 
        let middle_elem = first_elem+(last_elem-first_elem)/2;
        if array[middle_elem] > search_value {
            return binary_search(array,search_value,first_elem,middle_elem)
        }
        else if array[middle_elem] < search_value {
            return binary_search(array,search_value,middle_elem+1,last_elem)
        }
        else { return middle_elem }
    }
}
