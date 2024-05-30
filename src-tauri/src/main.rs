// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parse;
mod stats;

use self::models::*;
use app::*;
use diesel::prelude::*;

fn main() {
  use app::schema::hand::dsl::*;

  let file_path = "/home/noah/Games/poker_logs/PokerZhyte/HH20240326 Cornelia III - $0.01-$0.02 - USD No Limit Hold'em.txt";
  let hands = parse::parse_file(file_path);
  println!("{:#?}", hands);

  let connection = &mut establish_connection();
  let results = hand
    .select(Hand::as_select())
    .load(connection)
    .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  for post in results {
    println!("{}", post.title);
    println!("-----------\n");
    println!("{}", post.body);
  }
}
