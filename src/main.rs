mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

trait Solution {
    fn run() -> () where Self: Sized;
}

use std::time::Instant;
use std::io::{self, Write};

extern crate core;

#[macro_use]
extern crate text_io;
extern crate regex;
extern crate proc_macro;
extern crate im;

fn main() {
    print!("Which day? ");
    io::stdout().flush().unwrap();

    let day: i32 = read!();
    let days:Vec<Box<dyn Fn()->()>> = vec!(
        Box::new(|| day_1::run()),
        Box::new(|| day_2::run()),
        Box::new(|| day_3::run()),
        Box::new(|| day_4::run()),
        Box::new(|| day_5::run()),
        Box::new(|| day_6::run()),
        Box::new(|| day_7::run()),
        Box::new(|| day_8::run())
    );

    let start = Instant::now();
    match days.get((day - 1) as usize) {
        Some(solution) => solution(),
        None if day == 0 => days.iter().enumerate().for_each(|(i, solution)| {
            println!("==== Day {} ====", i + 1);
            solution();
            println!();
        }),
        None => println!("Invalid Day {}", day)
    }

    println!();
    println!("Finished in {:.2?}", start.elapsed());
}

