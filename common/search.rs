///generic linear search
///For any type which allows binary comparisons, it iterates through
///the array and returns a boolean  for whether or not the key
///was found, and if true returns the element at which it was found.
pub fn linear_search<T: Eq>(array: &[T], key: T) -> (bool, uint) {
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
