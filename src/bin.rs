extern crate cookbook;
extern crate rand;
use ::rand::{Rng, Rand};

use cookbook::basic::read_file;
use cookbook::basic::rand_gen;
use cookbook::basic::rand_gen::{Point};
use cookbook::basic::command;
use cookbook::basic::lazy_eval;


pub fn main() {
  read_file::run();
  read_file::run2();
  rand_gen::gen_f64_rand();
  rand_gen::gen_numbder_from_range();
  rand_gen::gen_with_distribution();

  let mut rng = ::rand::thread_rng();
  let rand_tuple = rng.gen::<(i32, bool, f64)>();
  let rand_point: Point = rng.gen();
  println!("Random tuple: {:?}", rand_tuple);
  println!("Random Point: {:?}", rand_point);

  command::run();



  lazy_eval::show_access("Jim");
  lazy_eval::run();
}
