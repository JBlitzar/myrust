use rand::seq::SliceRandom;
use rand::rng;

fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    return true;
}

fn radix_sort<T: Ord + Copy>(arr: &mut [T]){
    let mut maximum: T = arr[0];
    for &item in arr.iter() {
        if item > maximum  {
            maximum = item;
        }
    }

    

    
}

fn main() {
    let mut arr = vec![38, 27, 43, 3,3,3,1,1, 9, 82, 10,1_000_000, -5, 0];
    radix_sort(&mut arr);


    println!("{:?}", arr);
    assert! (is_sorted(&arr));

   

    let mut random_arr: Vec<i32> = (0..1_000).collect();

    let mut rng = rng();
    random_arr.shuffle(&mut rng);

    radix_sort(&mut random_arr);
    assert! (is_sorted(&random_arr));

    println!("All tests passed!");


   
   
}
