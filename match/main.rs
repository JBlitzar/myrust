enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("Quarter is worth 25 cents.");
            25
        },
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("The value of the coin is {} cents.", value_in_cents(coin));

    let c2 = Coin::Quarter;
    println!("The value of the coin is {} cents.", value_in_cents(c2));
}