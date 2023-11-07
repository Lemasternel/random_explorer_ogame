extern crate rand;
extern crate rand_distr;

use std::io::stdin;
use rand_distr::{Distribution, Uniform};

fn main() {
    const EXIT_KEY: &str = "1";
    
    let mut input = String::new();

    println!("Randomizes the coordinates for galaxy explorer in OGame");
    loop {
        randomize_explorer();
        println!("Press any key to re-run or '1' to exit! ");

        stdin().read_line(&mut input)
            .expect("Fail to read lines!");

        match input.trim() == EXIT_KEY {
            true => std::process::exit(0),
            false => continue
        }
    }
}

fn randomize_explorer()
{
    const PLANET: i32 = 16; //Represents the unexplored universe!

    let mut rng_galaxy = rand::thread_rng();
    let mut rng_star = rand::thread_rng();

    let range_galaxy = Uniform::new(1,5);
    let range_star = Uniform::new(1, 500);

    let galaxy = range_galaxy.sample(&mut rng_galaxy);
    let star = range_star.sample(&mut rng_star);

    let answer = format!("Coordinates {}:{}:{}", galaxy, star, PLANET);
    println!("{}", answer);
}