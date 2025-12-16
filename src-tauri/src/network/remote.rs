use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message, Connector};
use url::Url;

// Imports específicos de Rustls
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, RootCertStore};

/// Estructura para ignorar la verificación de certificados (Peligroso, pero necesario para TVs)
#[derive(Debug)]
struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        // Le decimos a Rustls que confíe ciegamente en cualquier certificado
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA1,
            rustls::SignatureScheme::ECDSA_SHA1_Legacy,
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

/// Helper para crear el conector Rustls inseguro
fn get_insecure_connector() -> Connector {
    let root_store = RootCertStore::empty();

    let mut config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Inyectamos nuestro verificador que ignora errores SSL
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(NoCertificateVerification));

    Connector::Rustls(Arc::new(config))
}

// -------------------------------------------------------------------
// SAMSUNG TIZEN (Rustls Version)
// -------------------------------------------------------------------
pub async fn send_command_to_tv(ip: String, key: String) -> Result<String, String> {
    println!("Samsung: Enviando {} a {} (Puerto 8002)", key, ip);

    let ws_url = format!(
        "wss://{}:8002/api/v2/channels/samsung.remote.control?name=VGVzdFJlbW90ZQ==",
        ip
    );

    if Url::parse(&ws_url).is_err() {
        return Err("URL Samsung inválida".to_string());
    }

    // Usamos el helper de Rustls
    let connector = get_insecure_connector();

    let (mut ws_stream, _) = connect_async_tls_with_config(ws_url, None, false, Some(connector))
        .await
        .map_err(|e| format!("Error conectando a Samsung: {}", e))?;

    let command = json!({
        "method": "ms.remote.control",
        "params": {
            "Cmd": "Click",
            "DataOfCmd": key,
            "Option": "false",
            "TypeOfRemote": "SendRemoteKey"
        }
    });

    let msg = Message::Text(command.to_string().into());

    match ws_stream.send(msg).await {
        Ok(_) => {
            sleep(Duration::from_millis(300)).await;
            let _ = ws_stream.close(None).await;
            Ok("Comando Samsung OK".to_string())
        }
        Err(e) => Err(format!("Error enviando a Samsung: {}", e)),
    }
}

// -------------------------------------------------------------------
// LG WEBOS (Rustls Version)
// -------------------------------------------------------------------
pub async fn send_lg_command(
    ip: String,
    command_uri: String,
    client_key: Option<String>,
) -> Result<String, String> {
    println!(
        "LG: Conectando a wss://{}:3001 con llave: {:?}",
        ip, client_key
    );

    let ws_url = format!("wss://{}:3001", ip);

    // Usamos el helper de Rustls
    let connector = get_insecure_connector();

    let (mut ws_stream, _) = connect_async_tls_with_config(ws_url, None, false, Some(connector))
        .await
        .map_err(|e| format!("Error conectando LG: {}", e))?;

    let key_to_send = client_key.unwrap_or("null".to_string());

    let handshake = json!({
        "type": "register",
        "id": "register_0",
        "payload": {
            "forcePairing": false,
            "pairingType": "PROMPT",
            "client-key": key_to_send,
            "manifest": {
                "permissions": ["LAUNCH", "CONTROL_AUDIO", "CONTROL_INPUT_MEDIA_PLAYBACK", "CONTROL_POWER", "READ_CURRENT_CHANNEL", "CONTROL_DISPLAY", "CONTROL_INPUT_TEXT", "CONTROL_MOUSE_AND_KEYBOARD", "READ_RUNNING_APPS"],
                "appVersion": "1.0"
            }
        }
    });

    if let Err(e) = ws_stream
        .send(Message::Text(handshake.to_string().into()))
        .await
    {
        return Err(format!("Error enviando handshake LG: {}", e));
    }

    let mut captured_key: Option<String> = None;
    let mut is_registered = false;
    let mut attempts = 0;

    println!("LG: Esperando respuesta...");

    while attempts < 20 {
        match tokio::time::timeout(Duration::from_secs(2), ws_stream.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => {
                if let Ok(json_msg) = serde_json::from_str::<Value>(&text) {
                    if json_msg["type"] == "registered" {
                        println!("LG: ¡Registrado correctamente!");
                        is_registered = true;
                        if let Some(k) = json_msg["payload"]["client-key"].as_str() {
                            println!("LG: ¡Llave capturada!: {}", k);
                            captured_key = Some(k.to_string());
                        }
                        break;
                    }
                }
            }
            Ok(None) => break,
            _ => {}
        }
        attempts += 1;
    }

    if !is_registered {
        return Err("No se pudo registrar en LG (Timeout o Rechazado)".to_string());
    }

    let payload = json!({
        "type": "request",
        "id": "req_1",
        "uri": command_uri
    });

    if let Err(e) = ws_stream
        .send(Message::Text(payload.to_string().into()))
        .await
    {
        return Err(format!("Error enviando comando: {}", e));
    }

    sleep(Duration::from_millis(300)).await;
    let _ = ws_stream.close(None).await;

    if let Some(k) = captured_key {
        let response = json!({ "status": "OK", "new_key": k });
        return Ok(response.to_string());
    }

    Ok(json!({ "status": "OK" }).to_string())
}
