fn main() {
    let a = "Blue";
    println!("Hello, world!");
    println!("The color is: {}", a);
    if let Some(letter) = a.chars().nth(2) {
        println!("The letter at index 2 is: {}", letter);
    }


}