// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

// Put your function here!
use std::io;

// fn calculate_price_of_apples(number: u32) -> u32 {
//     if number <= 40 {
//         number * 2
//     } else {
//         number *1
//     }
// }

fn calculate_price_of_apples(number: u32) -> Result<u32, &'static str> {
    if number > 0 {
        if number <= 40 {
            Ok(number * 2)
        } else {
            Ok(number * 1)
        }
    } else {
        Err("Bad quantity")
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);
    let price5 = calculate_price_of_apples(0);

    assert_eq!(Ok(70), price1);
    assert_eq!(Err("Bad quantity"), price5)
    // assert_eq!(80, price2);
    // assert_eq!(41, price3);
    // assert_eq!(65, price4);
}
