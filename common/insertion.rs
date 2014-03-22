pub fn ins_sort(array_clone: ~[int],array_size: int) -> ~[int] {
    let mut array = array_clone;
    let mut i = 0;
    while i < array_size {
        let val = array[i];
        let mut j = i - 1;
        while (j >= 0 && array[j] > val) { 
            array[j+1] = array[j];
            j-=1;
        }
        array[j+1] = val;
        i+=1;
    }
    return array;
}
