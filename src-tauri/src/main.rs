// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parse;
mod stats;

use self::models::*;
use app::*;
use diesel::prelude::*;

fn main() {
  let file_path = "/home/noah/Games/poker_logs/PokerZhyte/HH20240326 Cornelia III - $0.01-$0.02 - USD No Limit Hold'em.txt";
  let hands_detail = parse::parse_file(file_path);
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
