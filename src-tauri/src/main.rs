// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod math;

#[tauri::command]
fn getFibonnacci(index: String) -> u64 {
  let mut i = match index.parse::<u32>() {
    Ok(i) => i,
    Err(_) => 0,
  };
  return math::fib(i);
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![getFibonnacci])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}