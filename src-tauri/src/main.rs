#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde::{Serialize, Deserialize};
use tauri::Manager;
use std::fs;
use tauri_plugin_positioner::{WindowExt, Position};





#[derive(Serialize, Deserialize, Default, Debug)]
struct WindowState {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    fullscreen: bool,
}



#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[tauri::command]
fn toggle_fullscreen(window: tauri::Window) {
    let is_full = window.is_fullscreen().unwrap_or(false);
    window.set_fullscreen(!is_full).unwrap();
}



fn save_window_state(window: &tauri::Window, app: &tauri::AppHandle) {

    let maximaized = window.is_maximized().unwrap_or(false);
    if maximaized {return};

    let pos = window.outer_position().unwrap();
    let size = window.outer_size().unwrap();
    let fullscreen = window.is_fullscreen().unwrap_or(false);

    let state = WindowState {
        x: pos.x,
        y: pos.y,
        width: size.width,
        height: size.height,
        fullscreen,
    };

    let path = app
        .path_resolver()
        .app_config_dir()
        .unwrap()
        .join("window_state.json");

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::create_dir_all(path.parent().unwrap()).ok();
    fs::write(path, json).ok();
}





fn restore_window_state(window: &tauri::Window, app: &tauri::AppHandle) {
    let path = app
        .path_resolver()
        .app_config_dir()
        .unwrap()
        .join("window_state.json");

    if let Ok(json) = fs::read_to_string(path) {
        if let Ok(state) = serde_json::from_str::<WindowState>(&json) {

            window.set_size(tauri::Size::Physical(
                tauri::PhysicalSize { width: state.width, height: state.height }
            )).ok();
            
            window.set_position(tauri::Position::Physical(
                tauri::PhysicalPosition { x: state.x, y: state.y }
            )).ok();

            window.set_fullscreen(state.fullscreen).ok();
        }
    }
}




fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            restore_window_state(&window, &app.handle());

            let app_handle = app.handle();

            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    if let Some(w) = app_handle.get_window("main") {
                        save_window_state(&w, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, toggle_fullscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}




