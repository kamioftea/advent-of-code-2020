mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

extern crate core;

#[macro_use] extern crate text_io;
extern crate regex;

fn main() {
    println!("Which day? ");
    let day: i32 = read!();
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        _ => println!("Invalid day: {}", day)
    }
}

