// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mouse_position::mouse_position::Mouse;
use std::cell::RefCell;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![update])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

thread_local! {static X: RefCell<i32> = const {RefCell::new(0)}}
thread_local! {static Y: RefCell<i32> = const {RefCell::new(0)}}

#[tauri::command]
fn update() -> [i32; 2] {
  let position = Mouse::get_mouse_position();
  match position {
    Mouse::Position { x, y } => {
      X.with_borrow_mut(|x_cell| {
        *x_cell = x;
      });
      Y.with_borrow_mut(|y_cell| {
        *y_cell = y;
      });
      [x, y]
    }
    Mouse::Error => {
      println!("Error");
      [0, 0]
    }
  }
}
