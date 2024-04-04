#![allow(dead_code)]
#![allow(unused_variables)]
mod db;
mod parse;
use crate::db::HandDB;
use crate::parse::{parse, Hand};

fn main() {
    let hands: Vec<Hand> = parse("/home/noah/Games/poker_logs/PokerZhyte/HH20240326 Cornelia III - $0.01-$0.02 - USD No Limit Hold'em.txt");
    let hand_db = HandDB::new();
    for hand in hands {}
}
