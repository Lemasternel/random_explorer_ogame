extern crate rand;
extern crate rand_distr;

use std::{io::stdin, i32};
use rand::Rng;

fn main() {
    let exit_key: &str = "1";
    let planet: i32 = 16;
    
    let mut input = String::new();

    println!("Randomizes the coordinates for galaxy explorer in OGame");
    loop {
        randomize_explorer(planet);
        println!("Press any key to re-run or '1' to exit! ");

        stdin().read_line(&mut input)
            .expect("Fail to read lines!");

        match input.trim() == exit_key {
            true => std::process::exit(0),
            false => continue
        }
    }
}

fn randomize_explorer(planet: i32)
{
    let galaxy = rand::thread_rng().gen_range(1..=4);
    let star = rand::thread_rng().gen_range(1..=499);

    let answer = format!("Coordinates {}:{}:{}", galaxy, star, planet);
    println!("{}", answer);
}