use std::io;
fn main(){
    println!("Enter the string to get a reverse version :) ez: ");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to get the input");

    let string = string.trim();
    let reverse = string.chars().rev().collect::<String>();
    println!("{}", reverse);




}