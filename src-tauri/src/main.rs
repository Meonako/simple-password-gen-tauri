#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::{thread_rng, distributions::{Alphanumeric, DistString}};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn generate(password_length: usize) -> Result<String, String> {
    let password = Alphanumeric.sample_string(&mut thread_rng(), password_length);

    Ok(password)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
