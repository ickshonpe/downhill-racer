extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use find_folder::Search;
use find_folder::Error;
use std::path::PathBuf;

pub fn select_random<T>(x: T, y: T) -> T {
   if rand::thread_rng().gen() { x } else { y }
}
pub fn select_random_3<T>(x: T, y: T, z: T) -> T {
   let mut rng = rand::thread_rng();
   let range = Range::new(0, 3);
   let choice = range.ind_sample(&mut rng);
   match choice {
      0 => {x}
      1 => {y}
      2 => {z}
      _ => { panic!("Random PANIC!!") }
   }
}

pub fn select_random_in_range(min: usize, max_exclusize: usize) -> usize {
   Range::new(min, max_exclusize)
      .ind_sample(&mut rand::thread_rng())
}


pub fn find_assets_folder() -> PathBuf {
   let p = Search::KidsThenParents(3, 3).for_folder("assets");
   match p {
      Result::Ok(path) => { path }
      Result::Err(E) => { panic!("Assets folder not found.") }
   }
}

pub fn find_asset(filename: &str) -> PathBuf {
   find_assets_folder().join(filename)
}