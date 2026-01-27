
fn is_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    return true;
}

fn merge_sort(arr: &mut [i32]){
    if (arr.len() <= 1){
        return;
    }
    if (is_sorted(arr)){
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    // merge

    let mut merged: Vec<i32> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    let n1 = mid;
    let n2 = arr.len() - n1;
    let cur = 0;
    while (i1 < n1 && i2 < n2){
        if(arr[i1] <= arr[mid + i2]){
            merged.push(arr[i1]);
            i1+=1;
        }else{
            merged.push(arr[mid + i2]);
            i2+=1;
        }
    }
    while (i1<n1){
        merged.push(arr[i1]);
        i1+=1;

    }
    while (i2<n2){
        merged.push(arr[mid + i2]);
        i2+=1;
    }

    for i in 0..arr.len(){
        arr[i] = merged[i];
    }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut arr);
    println!("{:?}", arr);
   
}
