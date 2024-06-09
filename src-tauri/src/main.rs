// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod db;
mod parse;
mod stats;

use core::panic;
use db::models;

use crate::db::establish_connection;

fn main() {
  let directory = "/mnt/windows/Users/noah/AppData/Local/PokerStars.BE/HandHistory/PokerZhyte/play";
  let mut files = Vec::new();

  // add file content in vectore
  for entry in std::fs::read_dir(directory).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    let path_str = path.to_str().unwrap();
    files.push(path_str.to_string());
  }

  // parse the first file
  let hands_detail = match parse::parse_file(&files[0]) {
    Err(e) => panic!("Error {}\nparsing file : {:#?}", e, files[0]),
    Ok(h) => h,
  };

  // compute the stats for PokerZhyte player
  let mut poker_zhyte = models::Player::new("PokerZhyte");
  stats::add(&mut poker_zhyte, &hands_detail);
  println!("player after stats : {:#?}", poker_zhyte);

  // compute hand of DB from hand detail
  let hand = hands_detail[0].to_hand();
  println!("Hand detail : {:#?}", hands_detail[0]);
  println!("hand from hand_detail : {:#?}", hand);

  let mut conn = establish_connection().unwrap();
  db::insert_hand(&mut conn, &hand).unwrap();

  db::show_hands(&mut conn).unwrap();
}
