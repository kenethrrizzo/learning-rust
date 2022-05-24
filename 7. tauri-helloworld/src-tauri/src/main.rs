#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn show_city(city_name: String) {
    println!("This is my city: {}", city_name);
}

#[tauri::command]
fn hello_world() {
    println!("Hello, world.");
}

#[tauri::command]
fn hello_user(username: String) -> String {
    format!("Hello from Rust, {}", username).into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_city, hello_world, hello_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
