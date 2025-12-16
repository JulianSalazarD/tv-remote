// src-tauri/src/lib.rs

pub mod network;

use network::{discovery, remote};
use tauri::command;

// 1. Comando de Escaneo (Ya lo tenías, sin cambios)
#[command]
async fn scan_devices() -> Vec<discovery::TVDevice> {
    discovery::scan_network().await
}

// 2. Comando para Enviar Teclas (¡Aquí está la magia nueva!)
// Recibe: IP, La Tecla (ej "VOL_UP") y la Marca
#[command]
async fn send_key(ip: String, key: String, brand: String) -> Result<String, String> {
    println!("Recibido: {} para {} ({})", key, brand, ip);

    // --- LÓGICA PARA LG (WebOS) ---
    if brand.to_uppercase().contains("LG") || brand.to_uppercase().contains("WEBOS") {
        // Mapeo de teclas de nuestra App -> Comandos URI de LG
        let lg_cmd = match key.as_str() {
            "VOL_UP" => "ssap://audio/volumeUp",
            "VOL_DOWN" => "ssap://audio/volumeDown",
            "MUTE" => "ssap://audio/setMute",
            "CH_UP" => "ssap://tv/channelUp",
            "CH_DOWN" => "ssap://tv/channelDown",
            "POWER" => "ssap://system/turnOff", // Solo apagar (Encender requiere WoL)
            "OK" => "ssap://com.webos.service.ime/sendEnterKey",
            "HOME" => "ssap://system/launcher/open", // Abre el menú de apps
            "BACK" => "ssap://input/button?name=BACK", // A veces varía según versión
            "MENU" => "ssap://com.webos.applicationManager/launch",
            "UP" => "ssap://com.webos.service.ime/sendKeyCode?code=19", // Códigos raw raros
            "DOWN" => "ssap://com.webos.service.ime/sendKeyCode?code=20",
            "LEFT" => "ssap://com.webos.service.ime/sendKeyCode?code=21",
            "RIGHT" => "ssap://com.webos.service.ime/sendKeyCode?code=22",
            _ => "ssap://audio/volumeUp", // Default seguro
        };

        return remote::send_lg_command(ip, lg_cmd.to_string()).await;
    }
    // --- LÓGICA PARA SAMSUNG (Tizen) ---
    // Si no es LG, asumimos Samsung por defecto
    else {
        // Mapeo de teclas de nuestra App -> Códigos Tizen
        let samsung_key = match key.as_str() {
            "VOL_UP" => "KEY_VOLUP",
            "VOL_DOWN" => "KEY_VOLDOWN",
            "MUTE" => "KEY_MUTE",
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
            "SOURCE" => "KEY_SOURCE",
            _ => "KEY_VOLDOWN",
        };

        return remote::send_command_to_tv(ip, samsung_key.to_string()).await;
    }
}

// 3. Configuración Principal de Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Registramos AMBOS comandos aquí
        .invoke_handler(tauri::generate_handler![scan_devices, send_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
