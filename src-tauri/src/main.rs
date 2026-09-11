// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_fullscreen(window: tauri::Window) {
    let is_full = window.is_fullscreen().unwrap_or(false);
    window.set_fullscreen(is_full).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, toggle_fullscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
