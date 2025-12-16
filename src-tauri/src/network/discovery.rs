use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TVDevice {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub location: String,
    pub model: String,
}

const SSDP_MULTICAST_ADDR: &str = "239.255.255.250:1900";
const SEARCH_TIMEOUT_SECS: u64 = 3;

pub async fn scan_network() -> Vec<TVDevice> {
    println!("Iniciando escaneo SSDP agresivo...");
    let mut devices = Vec::new();

    // Bind a 0.0.0.0:0 permite al SO elegir el puerto libre
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error crítico socket: {}", e);
            return vec![];
        }
    };

    // Configuración vital para redes domésticas
    let _ = socket.set_broadcast(true);
    // Intentamos configurar el TTL para que el paquete salte routers domésticos si es necesario
    let _ = socket.set_ttl(4);

    // Paquetes a enviar: Buscamos TODO y buscamos Root Devices
    let queries = vec!["ssdp:all", "upnp:rootdevice"];

    // Enviamos los "gritos" a la red
    for target in queries {
        let msg = format!(
            "M-SEARCH * HTTP/1.1\r\n\
            HOST: 239.255.255.250:1900\r\n\
            MAN: \"ssdp:discover\"\r\n\
            MX: 2\r\n\
            ST: {}\r\n\
            \r\n",
            target
        );

        if let Err(e) = socket.send_to(msg.as_bytes(), SSDP_MULTICAST_ADDR).await {
            eprintln!("Error enviando paquete: {}", e);
        }
    }

    // Escucha
    let mut buf = [0u8; 4096];
    let end_time = tokio::time::Instant::now() + Duration::from_secs(SEARCH_TIMEOUT_SECS);

    while tokio::time::Instant::now() < end_time {
        let remaining_time = end_time - tokio::time::Instant::now();

        // Timeout para no bloquear
        if let Ok(Ok((size, addr))) = timeout(remaining_time, socket.recv_from(&mut buf)).await {
            let response = String::from_utf8_lossy(&buf[..size]);

            // DEBUG: Descomenta esto para ver TODO lo que llega (incluso si no es TV)
            // println!("Recibido de {}: \n{}", addr, response);

            if let Some(device) = parse_ssdp_response(&response, addr) {
                if !devices.iter().any(|d: &TVDevice| d.ip == device.ip) {
                    println!("¡EUREKA! Encontrado: {} en {}", device.model, device.ip);
                    devices.push(device);
                }
            }
        } else {
            // Timeout alcanzado
            break;
        }
    }

    println!(
        "Escaneo finalizado. Dispositivos totales: {}",
        devices.len()
    );
    devices
}

fn parse_ssdp_response(response: &str, addr: SocketAddr) -> Option<TVDevice> {
    let location = extract_header(response, "LOCATION")?;
    let usn = extract_header(response, "USN").unwrap_or_default();
    let server = extract_header(response, "SERVER")
        .or_else(|| extract_header(response, "USER-AGENT")) // A veces viene aquí
        .unwrap_or_else(|| "Dispositivo Genérico".to_string());

    // NOTA: He quitado el filtro de "TV" para que veas SIQUIERA ALGO (Router, Impresora, etc)
    // Así sabremos si el código funciona.

    Some(TVDevice {
        id: usn,
        name: server.clone(),
        ip: addr.ip().to_string(),
        location,
        model: identify_brand(&server),
    })
}

fn extract_header(response: &str, header: &str) -> Option<String> {
    for line in response.lines() {
        if line.to_uppercase().starts_with(header) {
            return line.splitn(2, ':').nth(1).map(|s| s.trim().to_string());
        }
    }
    None
}

fn identify_brand(server: &str) -> String {
    let s = server.to_lowercase();
    if s.contains("samsung") || s.contains("tizen") {
        return "Samsung".to_string();
    }
    if s.contains("lg") || s.contains("webos") {
        return "LG".to_string();
    }
    if s.contains("roku") {
        return "Roku".to_string();
    }
    if s.contains("android") || s.contains("google") {
        return "Android TV".to_string();
    }
    if s.contains("linux") {
        return "Posible TV/Server".to_string();
    }
    "Desconocido".to_string()
}
