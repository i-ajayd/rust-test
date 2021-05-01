use std::io;
use rand::prelude::*;

pub fn run() {
  let mut num = String::new();
  io::stdin().read_line(&mut num);
  println!("number is {}", num);
  let mut rng = thread_rng();
  let y: u32 = rng.gen_range(1..100);
  println!("sec num is {}", y);
}