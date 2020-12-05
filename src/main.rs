extern crate sysfs_gpio;
use abecele;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::io;


fn main() {
    println!("Enter the number");
    let mut guess = String::new();  //Create new String
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    let _my_led = Pin::new(24); // User GPIO pin 
    let guess_proc: Vec<char> = guess.trim().chars().collect();
    for x in 0..guess_proc.len(){
        if guess_proc[x] == '0' {
            abecele::numbers::zero();
        }
    }
}