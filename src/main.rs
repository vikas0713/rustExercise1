mod swap;
use swap::swap_numbers;

// This is a practice program of rust,
// First exercise (https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary) solution
use std::io;


fn main() {
    println!("Enter your choice: ");
    println!("1. Temperature \n2. Fibonacci Number \n3. Print Lyrics \n");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Please enter valid option");
    let option:i32 = option.trim().parse().unwrap();
    if option == 1 {
        println!("Converted Temperature is: {}", temperature_conversion());
    }else if  option == 2 {
        println!("Fibonacci Series , coming soon!");
    } else if option == 3 {
        println!("Printing Lyrics.... coming soon");
    } else if option == 4 {
        println!("Swapping 23, 45");
    }
    else{
        println!("Wrong option!!!")
    }
}

fn temperature_conversion() -> f32 {
    let mut temp = String::new();
    println!("Enter valid temperature value:  ");
    io::stdin().read_line(&mut temp).expect("Failed to read!!");
    let temperature: f32 = temp.trim().parse().unwrap();
    let formula: f32 = 5.00 / 9.00;
    (temperature - 32.00) * formula
}
