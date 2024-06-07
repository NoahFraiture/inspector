// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod parse;
mod stats;

use core::panic;

use self::models::*;
use app::*;
use diesel::prelude::*;

fn main() {
  let directory = "/mnt/windows/Users/noah/AppData/Local/PokerStars.BE/HandHistory/PokerZhyte/play";
  let mut files = Vec::new();
  for entry in std::fs::read_dir(directory).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    let path_str = path.to_str().unwrap();
    files.push(path_str.to_string());
  }

  println!("number of files : {:#?}", files.len());
  println!("first file : {:#?}", files[0]);

  let hands_detail = match parse::parse_file(&files[0]) {
    Err(e) => panic!("Error {}\nparsing file : {:#?}", e, files[0]),
    Ok(h) => h,
  };
  println!("number of hands : {:#?}", hands_detail.len());

  // map the hands to transform every element to a Hand and then insert into the db
  for hand_detail in hands_detail {
    let hand = hand_detail.to_hand();
  }

  let connection = &mut establish_connection();
  let results = schema::hand::dsl::hand
    .select(Hand::as_select())
    .load(connection)
    .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  for h in results {
    println!("{}", h.winner);
    println!("-----------\n");
    println!("{}", h.pot);
  }
}
