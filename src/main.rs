extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Enter the number");
    let mut guess = String::new();  //Create new String
    let my_led = Pin::new(23); // User GPIO pin
    io::stdin()
        .read_line(&mut guess);

        if guess == "2" {   // Compare input to see if it will match correctly
            loop {
                println!("Answer is 2");

                my_led.set_value(0).expect("Guess value 2, LED LOW"); //Can ignore, irrelevant
                sleep(Duration::from_millis(200));

                my_led.set_value(1).expect("Guess value 2, LED HIGH");
                sleep(Duration::from_millis(200));
            }
        } else{
            loop{
                println!("Answer is not 2"); // If input is 2, it says "Answer is not 2"

                my_led.set_value(0).expect("Guess value NOT 2, LED HIGH");
                sleep(Duration::from_millis(200));

                my_led.set_value(1).expect("Guess value NOT 2, LED HIGH");
                sleep(Duration::from_millis(6000));

            }
        }
}

