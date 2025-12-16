import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// --- ESTADO GLOBAL ---
// Definimos las variables fuera de la función para que mantengan su valor
// aunque cambies de vista (de Scan a Remote).

// 1. IP de la TV seleccionada
const currentTvIp = ref<string>(localStorage.getItem('saved_tv_ip') || '');

// 2. Modelo de la TV (Samsung / LG / etc.)
const currentTvModel = ref<string>(localStorage.getItem('saved_tv_model') || 'Samsung');

// 3. Llave de Cliente LG (Vital para evitar el popup constante)
const lgClientKey = ref<string>(localStorage.getItem('lg_client_key_v2') || '');

export function useTvConnection() {

  /**
   * Guarda la TV seleccionada en memoria y LocalStorage
   * @param ip La dirección IP de la TV
   * @param model La marca o modelo ("Samsung", "LG", etc)
   */
  function selectTv(ip: string, model: string) {
    console.log(`Seleccionando TV -> IP: ${ip}, Modelo: ${model}`);

    currentTvIp.value = ip;
    currentTvModel.value = model;

    localStorage.setItem('saved_tv_ip', ip);
    localStorage.setItem('saved_tv_model', model);

    // NOTA: No borramos lgClientKey aquí. Si el usuario tiene varias LGs,
    // lo ideal sería guardar un diccionario {ip: key}, pero para este ejemplo
    // simple, mantenemos la última llave usada.
  }

  /**
   * Envía una tecla al Backend (Rust)
   * Gestiona automáticamente el envío y guardado de llaves de seguridad.
   */
  async function sendKey(key: string) {
    // Validación básica
    if (!currentTvIp.value) {
      console.warn("No hay TV seleccionada. Ve a la pestaña Scan.");
      return;
    }

    try {
      // Llamamos a Rust pasando: IP, Tecla, Marca y la Llave (si tenemos una)
      // Nota: Tauri convierte automáticamente 'clientKey' (JS) a 'client_key' (Rust)
      const response = await invoke<string>('send_key', {
        ip: currentTvIp.value,
        key: key,
        brand: currentTvModel.value,
        clientKey: lgClientKey.value || null // Si está vacío, enviamos null
      });

      // --- MANEJO DE RESPUESTA INTELIGENTE ---
      // Rust puede devolver un JSON simple o una cadena. Intentamos parsear.
      try {
        const jsonRes = JSON.parse(response);

        // Si la respuesta trae una 'new_key', significa que nos acabamos de registrar
        if (jsonRes.new_key) {
          console.log("🔐 ¡Nueva llave de seguridad LG guardada!", jsonRes.new_key);

          // Actualizamos estado y persistencia
          lgClientKey.value = jsonRes.new_key;
          localStorage.setItem('lg_client_key', jsonRes.new_key);
        }
      } catch (e) {
        // Si falla el parseo JSON, es que Rust devolvió texto plano (ej: "Samsung OK")
        // No pasa nada, ignoramos.
      }

    } catch (e) {
      console.error("❌ Error enviando comando:", e);
      // Re-lanzamos el error para que la UI (RemoteView) pueda mostrarlo en rojo
      throw e;
    }
  }

  return {
    currentTvIp,
    currentTvModel,
    lgClientKey, // Exportamos por si queremos mostrarla en la UI (debug)
    selectTv,
    sendKey
  };
}