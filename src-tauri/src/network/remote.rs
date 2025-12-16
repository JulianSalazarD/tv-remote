use futures_util::SinkExt; // Importante para poder usar .send() y .close()
use native_tls::TlsConnector;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{
    connect_async, connect_async_tls_with_config, tungstenite::protocol::Message, Connector,
};
use url::Url;

/// Función para enviar comandos a SAMSUNG (Tizen OS 2016+)
/// Usa Puerto 8002 (WSS) y requiere ignorar certificados SSL.
pub async fn send_command_to_tv(ip: String, key: String) -> Result<String, String> {
    println!("Samsung: Enviando {} a {} (Puerto 8002)", key, ip);

    // 1. Construir URL Segura
    // 'VGVzdFJlbW90ZQ==' es "TestRemote" en base64. Puedes cambiarlo por el nombre de tu app.
    let ws_url = format!(
        "wss://{}:8002/api/v2/channels/samsung.remote.control?name=VGVzdFJlbW90ZQ==",
        ip
    );
    let url_string = ws_url.clone();

    // Validación básica de URL
    if Url::parse(&ws_url).is_err() {
        return Err("URL Samsung inválida".to_string());
    }

    // 2. Configurar TLS para aceptar certificados autofirmados (CRUCIAL para Samsung)
    let tls_connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Error creando TLS: {}", e))?;

    let connector = Connector::NativeTls(tls_connector);

    // 3. Conectar
    let (mut ws_stream, _) =
        connect_async_tls_with_config(url_string, None, false, Some(connector))
            .await
            .map_err(|e| format!("Error conectando a Samsung (Revisa el popup en TV): {}", e))?;

    // 4. Construir Payload JSON
    let command = json!({
        "method": "ms.remote.control",
        "params": {
            "Cmd": "Click",
            "DataOfCmd": key,
            "Option": "false",
            "TypeOfRemote": "SendRemoteKey"
        }
    });

    // 5. Enviar Mensaje
    let msg = Message::Text(command.to_string().into());

    match ws_stream.send(msg).await {
        Ok(_) => {
            // Esperamos un poco para asegurar que la TV procese el comando antes de cortar
            sleep(Duration::from_millis(300)).await;
            let _ = ws_stream.close(None).await;
            Ok("Comando Samsung OK".to_string())
        }
        Err(e) => Err(format!("Error enviando bytes a Samsung: {}", e)),
    }
}

/// Función para enviar comandos a LG (WebOS)
/// Usa Puerto 3000 (WS) y requiere un Handshake de registro previo.
pub async fn send_lg_command(ip: String, command_uri: String) -> Result<String, String> {
    println!("LG: Enviando {} a {} (Puerto 3000)", command_uri, ip);

    // 1. Construir URL Simple (LG suele usar ws:// sin encriptar en puerto 3000)
    let url = format!("ws://{}:3000", ip);

    // 2. Conectar
    let (mut ws_stream, _) = connect_async(&url)
        .await
        .map_err(|e| format!("Error conectando a LG: {}", e))?;

    // 3. HANDSHAKE (Registro)
    // LG requiere presentarse antes de obedecer.
    // Si es la primera vez, saldrá un popup en la TV "Permitir App".
    let handshake = json!({
        "type": "register",
        "id": "register_0",
        "payload": {
            "forcePairing": false,
            "pairingType": "PROMPT",
            "client-key": "null",
            "manifest": {
                "permissions": ["LAUNCH", "CONTROL_AUDIO", "CONTROL_INPUT_MEDIA_PLAYBACK"],
                "appVersion": "1.0"
            }
        }
    });

    // Enviamos el saludo
    if let Err(e) = ws_stream
        .send(Message::Text(handshake.to_string().into()))
        .await
    {
        return Err(format!("Error en handshake LG: {}", e));
    }

    // Esperamos a que la TV nos registre (200ms es suficiente usualmente)
    sleep(Duration::from_millis(200)).await;

    // 4. Enviar el Comando Real
    let payload = json!({
        "type": "request",
        "id": "req_1",
        "uri": command_uri
    });

    match ws_stream
        .send(Message::Text(payload.to_string().into()))
        .await
    {
        Ok(_) => {
            sleep(Duration::from_millis(300)).await;
            let _ = ws_stream.close(None).await;
            Ok("Comando LG OK".to_string())
        }
        Err(e) => Err(format!("Error enviando comando LG: {}", e)),
    }
}
