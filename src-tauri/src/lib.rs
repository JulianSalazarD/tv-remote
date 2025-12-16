// src-tauri/src/lib.rs

pub mod network;

use network::discovery::{self, TVDevice};
use tauri::command; // Importamos nuestro módulo

#[command]
async fn scan_devices() -> Vec<TVDevice> {
    discovery::scan_network().await
}

// ... imports anteriores
use network::remote; // Importar el nuevo módulo

#[command]
async fn send_key(ip: String, key: String) -> Result<String, String> {
    // Mapeamos las teclas de nuestro UI a las de Samsung
    let samsung_key = match key.as_str() {
        "VOL_UP" => "KEY_VOLUP",
        "VOL_DOWN" => "KEY_VOLDOWN",
        "CH_UP" => "KEY_CHUP",
        "CH_DOWN" => "KEY_CHDOWN",
        "POWER" => "KEY_POWER",
        "UP" => "KEY_UP",
        "DOWN" => "KEY_DOWN",
        "LEFT" => "KEY_LEFT",
        "RIGHT" => "KEY_RIGHT",
        "OK" => "KEY_ENTER",
        "BACK" => "KEY_RETURN",
        "HOME" => "KEY_HOME",
        "MENU" => "KEY_MENU",
        _ => "KEY_VOLDOWN", // Default seguro
    };

    remote::send_command_to_tv(ip, samsung_key.to_string()).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_devices, send_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
