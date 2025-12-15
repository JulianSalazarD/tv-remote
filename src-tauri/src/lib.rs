// src-tauri/src/lib.rs

pub mod network;

use network::discovery::{self, TVDevice};
use tauri::command; // Importamos nuestro módulo

#[command]
async fn scan_devices() -> Vec<TVDevice> {
    discovery::scan_network().await
}

// ... (tu comando send_command existente)

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // ¡REGISTRA EL COMANDO AQUÍ!
        .invoke_handler(tauri::generate_handler![
            scan_devices,
            // enviar_comando (el que tenías antes)
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
