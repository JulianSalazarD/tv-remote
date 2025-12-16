use futures_util::SinkExt;
use native_tls::TlsConnector;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message, Connector};
use url::Url; // Necesario para ignorar el certificado SSL de Samsung

pub async fn send_command_to_tv(ip: String, key: String) -> Result<String, String> {
    println!("Intentando enviar {} a {} (Puerto 8002 Seguro)", key, ip);

    // 1. URL SEGURA (WSS + Puerto 8002)
    // El 'name' en base64 es importante. "VGVzdFJlbW90ZQ==" significa "TestRemote"
    let ws_url = format!(
        "wss://{}:8002/api/v2/channels/samsung.remote.control?name=VGVzdFJlbW90ZQ==",
        ip
    );
    let url = Url::parse(&ws_url).map_err(|_| "URL inválida")?;

    // 2. CONFIGURACIÓN TLS (La parte mágica)
    // Las TV Samsung usan certificados autofirmados. Debemos decirle a Rust que confíe en ellos.
    let tls_connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true) // <--- ESTO ES LA CLAVE
        .build()
        .map_err(|e| format!("Error TLS: {}", e))?;

    let connector = Connector::NativeTls(tls_connector);

    // 3. CONECTAR USANDO EL CONECTOR PERSONALIZADO
    let (mut ws_stream, _) =
        connect_async_tls_with_config(url.to_string(), None, false, Some(connector))
            .await
            .map_err(|e| {
                format!(
                    "Error conectando a la TV (Revisa si debes aceptar permiso en pantalla): {}",
                    e
                )
            })?;

    println!("¡Conectado exitosamente al puerto seguro 8002!");

    // 4. PREPARAR COMANDO
    let command = json!({
        "method": "ms.remote.control",
        "params": {
            "Cmd": "Click",
            "DataOfCmd": key,
            "Option": "false",
            "TypeOfRemote": "SendRemoteKey"
        }
    });

    // 5. ENVIAR
    let msg = Message::Text(command.to_string().into());

    match ws_stream.send(msg).await {
        Ok(_) => {
            println!("Comando enviado. Esperando a que la TV procese...");
            // Esperamos 300ms para asegurar que la TV lee el comando antes de cortar
            sleep(Duration::from_millis(300)).await;

            let _ = ws_stream.close(None).await;
            Ok("Comando enviado OK".to_string())
        }
        Err(e) => Err(format!("Error al enviar bytes: {}", e)),
    }
}
