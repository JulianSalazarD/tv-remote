// src-tauri/src/lib.rs

pub mod network;

use network::{discovery, remote};
use tauri::command;

// ----------------------------------------------------------------
// 1. COMANDO DE ESCANEO
// ----------------------------------------------------------------
#[command]
async fn scan_devices() -> Vec<discovery::TVDevice> {
    discovery::scan_network().await
}

// ----------------------------------------------------------------
// 2. COMANDO PARA ENVIAR TECLAS
// Recibe: IP, Tecla, Marca y la Llave de Cliente (Opcional)
// ----------------------------------------------------------------
#[command]
async fn send_key(
    ip: String,
    key: String,
    brand: String,
    client_key: Option<String>, // <--- Nuevo parámetro vital para LG
) -> Result<String, String> {
    // Debug: Imprimimos en la terminal de Rust qué estamos recibiendo
    println!(
        "Cmd: {} | TV: {} ({}) | Key: {:?}",
        key, brand, ip, client_key
    );

    // --- LÓGICA LG (WebOS) ---
    if brand.to_uppercase().contains("LG") || brand.to_uppercase().contains("WEBOS") {
        // Mapeo ACTUALIZADO para LG WebOS Moderno
        let lg_cmd = match key.as_str() {
            // Audio (Estos ya te funcionaban)
            "VOL_UP" => "ssap://audio/volumeUp",
            "VOL_DOWN" => "ssap://audio/volumeDown",
            "MUTE" => "ssap://audio/setMute",

            // Canales (Requiere permiso READ_CURRENT_CHANNEL)
            "CH_UP" => "ssap://tv/channelUp",
            "CH_DOWN" => "ssap://tv/channelDown",

            // Sistema
            "POWER" => "ssap://system/turnOff",
            "HOME" => "ssap://system/launcher/open", // Botón de la casa
            "MENU" => "ssap://com.webos.applicationManager/launch",

            // Navegación (EL CAMBIO IMPORTANTE)
            // Usamos el servicio 'com.webos.service.networkinput' o 'input/button'
            "OK" => "ssap://input/button?name=ENTER",
            "BACK" => "ssap://input/button?name=BACK",
            "UP" => "ssap://input/button?name=UP",
            "DOWN" => "ssap://input/button?name=DOWN",
            "LEFT" => "ssap://input/button?name=LEFT",
            "RIGHT" => "ssap://input/button?name=RIGHT",

            // Entradas (HDMI, etc)
            "SOURCE" => "ssap://tv/openInputPicker",

            // Default
            _ => "ssap://audio/volumeUp",
        };

        return remote::send_lg_command(ip, lg_cmd.to_string(), client_key).await;
    }
    // --- LÓGICA SAMSUNG (Tizen) ---
    // Si no es LG, asumimos Samsung por defecto
    else {
        // Mapeo de teclas internas -> Códigos Tizen
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

        // Samsung no usa el client_key en este protocolo simple, así que no lo pasamos
        return remote::send_command_to_tv(ip, samsung_key.to_string()).await;
    }
}

// ----------------------------------------------------------------
// 3. CONFIGURACIÓN PRINCIPAL TAURI
// ----------------------------------------------------------------
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_devices,
            send_key // Registramos el comando
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
