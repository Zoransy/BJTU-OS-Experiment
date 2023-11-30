#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("My first rust array program.");
    let mut numbers = [1, 2, 3, 4, 5];
    print!("Numbers: ");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!("Array End");
    0
}
