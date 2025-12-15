use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

// Esta estructura será lo que reciba tu Frontend (Vue)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TVDevice {
    pub id: String,       // USN (Unique Service Name)
    pub name: String,     // Friendly Name (o modelo)
    pub ip: String,       // La IP para enviarle comandos
    pub location: String, // La URL del XML de descripción
    pub model: String,    // Samsung, LG, Roku...
}

// Constantes del protocolo SSDP
const SSDP_MULTICAST_ADDR: &str = "239.255.255.250:1900";
const SEARCH_TIMEOUT_SECS: u64 = 3;

// El mensaje "Grito" que enviaremos a la red
// ST: ssdp:all -> Busca todo (Roku, Samsung, LG...)
const M_SEARCH_PACKET: &str = "M-SEARCH * HTTP/1.1\r\n\
HOST: 239.255.255.250:1900\r\n\
MAN: \"ssdp:discover\"\r\n\
MX: 1\r\n\
ST: ssdp:all\r\n\
\r\n";

pub async fn scan_network() -> Vec<TVDevice> {
    println!("Iniciando escaneo SSDP...");
    let mut devices = Vec::new();

    // 1. Crear el socket UDP (bind a 0.0.0.0 permite escuchar de cualquiera)
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error al crear socket UDP: {}", e);
            return vec![];
        }
    };

    // 2. Configurar broadcast (por si acaso) y enviar el M-SEARCH
    let _ = socket.set_broadcast(true);
    if let Err(e) = socket
        .send_to(M_SEARCH_PACKET.as_bytes(), SSDP_MULTICAST_ADDR)
        .await
    {
        eprintln!("Error enviando M-SEARCH: {}", e);
        return vec![];
    }

    // 3. Bucle de escucha con Timeout
    let mut buf = [0u8; 2048];
    let end_time = tokio::time::Instant::now() + Duration::from_secs(SEARCH_TIMEOUT_SECS);

    while tokio::time::Instant::now() < end_time {
        // Usamos timeout en cada lectura para no quedarnos bloqueados
        let remaining_time = end_time - tokio::time::Instant::now();

        match timeout(remaining_time, socket.recv_from(&mut buf)).await {
            Ok(Ok((size, addr))) => {
                let response = String::from_utf8_lossy(&buf[..size]);

                // Parseamos la respuesta cruda
                if let Some(device) = parse_ssdp_response(&response, addr) {
                    // Evitamos duplicados basándonos en la IP
                    if !devices.iter().any(|d: &TVDevice| d.ip == device.ip) {
                        println!("Dispositivo encontrado: {} ({})", device.name, device.ip);
                        devices.push(device);
                    }
                }
            }
            _ => break, // Timeout alcanzado o error
        }
    }

    devices
}

// Función auxiliar para extraer datos del texto HTTP crudo
fn parse_ssdp_response(response: &str, addr: SocketAddr) -> Option<TVDevice> {
    // Buscamos cabeceras clave
    let location = extract_header(response, "LOCATION")?;
    let usn = extract_header(response, "USN").unwrap_or_default();
    let server = extract_header(response, "SERVER").unwrap_or_default();

    // Filtro simple: Solo nos interesan cosas que parecen TVs o Media Renderers
    // Puedes quitar esto para ver TODO lo que hay en tu red
    /* if !server.to_lowercase().contains("tv") && !server.to_lowercase().contains("smart") {
       return None;
    }
    */

    Some(TVDevice {
        id: usn,
        name: server.clone(), // Temporalmente usamos el Server string como nombre
        ip: addr.ip().to_string(),
        location,
        model: identify_brand(&server),
    })
}

fn extract_header(response: &str, header: &str) -> Option<String> {
    for line in response.lines() {
        if line.to_uppercase().starts_with(header) {
            // Ejemplo: "LOCATION: http://192.168.1.5..." -> "http://..."
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
    if s.contains("android") {
        return "Android TV".to_string();
    }
    "Desconocido".to_string()
}
