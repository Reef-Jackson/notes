// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fmt::format, path::Path};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save])
    .run(tauri::generate_context!())
    .expect("Error running Save");
  app_lib::run();
}

#[tauri::command]
fn save(title: String) {   
  let filename = title + ".txt";
  let filename_parsed = filename.replace(" ", "_");
  println!("{}", filename_parsed);
}
