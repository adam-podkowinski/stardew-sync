#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn backup_saves(path: String) {
    println!("Backing up saves from {}!", path)
}

use std::thread;

fn main() {
    let path = "/home/bodzio/.config/StardewValley/Saves";
    let mut server_builder = file_serve::ServerBuilder::new(&path);
    server_builder.hostname("172.25.15.70");
    server_builder.port(3000);

    let server = server_builder.build();

    println!("Serving {}", path);
    println!("Hit CTRL-C to stop");

    thread::spawn(move || {
        server.serve().unwrap();
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![backup_saves])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
