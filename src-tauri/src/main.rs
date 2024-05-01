// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{DateTime, FixedOffset};

mod db;
mod parse;
mod stats;

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Default, Debug, PartialEq)]
pub struct Hand {
  pub content: String,
  pub id: u64, // u32 is too small
  pub date: DateTime<FixedOffset>,
  pub small_limit: f64,
  pub big_limit: f64,
  pub table_name: String,
  pub table_size: u8,
  pub button_position: u8, // usefull to shift position and guess real position
  pub players: [Option<parse::Player>; 9],
  pub small_blind: parse::Blind,
  pub big_blind: parse::Blind,
  pub end: parse::End, // NOTE: not used
  pub players_card: [Option<[String; 2]>; 9],
  pub preflop: Vec<parse::Action>,
  pub flop: Vec<parse::Action>,
  pub turn: Vec<parse::Action>,
  pub river: Vec<parse::Action>,
  pub flop_card: Option<[String; 3]>,
  pub turn_card: Option<String>,
  pub river_card: Option<String>,
}
