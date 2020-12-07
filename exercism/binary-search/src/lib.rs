pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() { return None; }
    if key < array[0] || key > array[array.len()-1] { return None; }
    binary_search(array, key, 0)
}

pub fn binary_search(array: &[i32], key: i32, prefix: usize) -> Option<usize> {
    let half = array.len()/2;
    if key < array[half] {
        if half == 0 { return None; }
        return binary_search(&array[..half], key, prefix);
    } else if key > array[half] {
        if half == 0 { return None; }
        return binary_search(&array[half+1..], key, half + prefix + 1);
    } else {
        return Some(half + prefix);
    }
}
