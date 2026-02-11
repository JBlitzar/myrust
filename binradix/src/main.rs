
fn radix_sort(arr: &mut  Vec<u32> ){
    if(arr.is_empty()) {
        return;
    }
    // find the maximum number to know the number of digits
    let max = arr.iter().max().unwrap().clone();
    let mut exp: usize = 0;
    let mut new = vec![0; arr.len()];
    let mut bitmask = vec![false; arr.len()];
    let mut counter: usize = 0;
    while (max >> exp) > 0 {
        sub_sort(arr, exp, &mut new, &mut bitmask, &mut counter);
        exp += 1;
    }

}

fn sub_sort(arr: &mut Vec<u32> , exp: usize, new: &mut Vec<u32>, bitmask: &mut Vec<bool>, counter: &mut usize) {
    // binary counting sort: scan through for all the zeros, add them to a new array, then scan through for all the ones, add them to the new array, then copy the new array back to the original array
    *counter = 0;
    for i in 0..arr.len() {
        bitmask[i] = (arr[i] >> exp) & 1 == 1;
        // if the bit at exp is 0, add to new array
        if !bitmask[i] {
            new[*counter] = arr[i];
            *counter += 1;
        }
    }
    for i in 0..arr.len() {
        if bitmask[i] {
            new[*counter] = arr[i];
            *counter += 1;
        }
    }

    arr.copy_from_slice(&new);
}

fn main() {
    println!("Hello, world!");
    let mut arr = vec![10,9,8,1,2,3,4,7,6,5,1_000_000, 999_999, 1_000_001];
    radix_sort(&mut arr);
    println!("{:?}", arr);
}
