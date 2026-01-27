
fn is_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    return true;
}

fn merge_sort_wrap(arr: &mut [i32]){
    let mut buf = vec![0; arr.len()];
    merge_sort(arr, &mut buf, 0);
}

fn merge_sort(arr: &mut [i32], buf: &mut [i32], b_offset: usize) {
    if arr.len() <= 1 {
        return;
    }
 
    let mid = arr.len() / 2;

    // w recursion
    merge_sort(&mut arr[..mid], buf, b_offset);
    merge_sort(&mut arr[mid..], buf, b_offset + mid);

    // merge


    let mut i1 = 0;
    let mut i2 = 0;
    let n1 = mid;
    let n2 = arr.len() - n1;
    let mut i = 0;

    while i1 < n1 && i2 < n2{
        if arr[i1] <= arr[mid + i2] {
            buf[b_offset + i] = arr[i1];
            i1+=1;
            i+=1;
        }else{
            buf[b_offset + i] = arr[mid + i2];
            i2+=1;
            i+=1;
        }
    }
    while i1<n1{
        buf[b_offset + i] = arr[i1];
        i1+=1;
        i+=1;
    }
    while i2<n2{
        buf[b_offset + i] = arr[mid + i2];
        i2+=1;
        i+=1;
    }

    for i in 0..arr.len(){
        arr[i] = buf[b_offset + i];
    }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3,3,3,1,1, 9, 82, 10,1_000_000, -5, 0];
    merge_sort_wrap(&mut arr);


    println!("{:?}", arr);
    assert! (is_sorted(&arr));

    println!("All tests passed!");
   
   
}
