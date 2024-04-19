// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mouse_position::mouse_position::Mouse;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![mouse_position])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn mouse_position() -> [i32; 2] {
  let position = Mouse::get_mouse_position();

  match position {
    Mouse::Position { x, y } => [x, y],
    Mouse::Error => [0; 2],
  }
}
