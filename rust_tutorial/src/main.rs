#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age:io::stdin = "47";
    let mut age:u32 = age.trim().parse().expect("Agent wasnt assigned");
    age=age +1
    println!("I'm {} and I want ${}", age, ONE_MILLION);
}
