#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::iter;

use rand::{thread_rng, Rng};

const CHARSET: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+/*-=";

#[tauri::command]
fn generate(password_length: usize) -> String {
    iter::repeat_with(|| CHARSET[thread_rng().gen_range(0..CHARSET.len())] as char)
        .take(password_length)
        .collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
