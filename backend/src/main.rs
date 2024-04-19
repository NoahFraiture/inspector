#![allow(dead_code)]
#![allow(unused_variables)]
mod db;
mod overlay;
mod parse;

fn main() {
  overlay::overlay().unwrap();
}
