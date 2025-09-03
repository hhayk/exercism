pub fn find(array: &[i32], key: i32) -> Option<usize> {
    //    todo!(
    //        "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."
    //    );
    let mut si = 0;
    let mut ei = array.len();

    while si < ei {
        let mid = si + (ei - si) / 2;
        let el = array[mid];

        if el < key {
            si = mid + 1;
        } else if el > key {
            ei = mid;
        } else {
            return Some(mid);
        }
    }

    None
}
