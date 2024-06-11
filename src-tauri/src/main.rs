// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod db;
mod parse;
mod stats;
mod track;

use core::panic;
use db::models;
use diesel::SqliteConnection;
use parse::HandDetail;
use std::path::Path;

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
  let hand = hands_detail[0].get_hand();
  println!("Hand detail : {:#?}", hands_detail[0]);
  println!("hand from hand_detail : {:#?}", hand);

  let mut conn = establish_connection().unwrap();
  db::insert_hand(&mut conn, &hand);
  db::show_hands(&mut conn).unwrap();

  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  use std::sync::mpsc::channel;
  let (tx, rx) = channel();

  let path = Path::new(r"/home/noah/test");
  if let Err(error) = track::watch(path, tx, rx) {
    log::error!("Error : {error:?}");
  }
  println!("here");
}

fn update_db(conn: &mut SqliteConnection, hands_detail: &Vec<HandDetail>) {
  for hand_detail in hands_detail {
    let hand = hand_detail.get_hand();
    db::insert_hand(conn, &hand);

    let actions = hand_detail.get_actions();
  }
}
