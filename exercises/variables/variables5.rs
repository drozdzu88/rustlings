// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    
    {
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }

    println!("Spell a Number : {}", number);
    
}
