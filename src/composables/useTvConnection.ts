import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Estado Global (fuera de la función para persistencia)
const currentTvIp = ref<string>(localStorage.getItem('saved_tv_ip') || '');
// ⚠️ IMPORTANTE: Aquí guardamos la marca. Por defecto Samsung, pero cambiará al seleccionar.
const currentTvModel = ref<string>(localStorage.getItem('saved_tv_model') || 'Samsung');

export function useTvConnection() {

  // 1. CORRECCIÓN CLAVE: Esta función AHORA debe recibir (ip, model)
  function selectTv(ip: string, model: string) {
    console.log(`Guardando selección -> IP: ${ip}, Modelo: ${model}`); // Debug

    currentTvIp.value = ip;
    currentTvModel.value = model; // Actualizamos la variable reactiva

    // Guardamos en memoria del teléfono para la próxima vez
    localStorage.setItem('saved_tv_ip', ip);
    localStorage.setItem('saved_tv_model', model);
  }

  // 2. CORRECCIÓN CLAVE: Al enviar, leemos el modelo guardado
  async function sendKey(key: string) {
    if (!currentTvIp.value) {
      console.warn("No hay TV seleccionada");
      return;
    }

    try {
      // Enviamos al backend: IP, Tecla y la MARCA guardada
      await invoke('send_key', {
        ip: currentTvIp.value,
        key: key,
        brand: currentTvModel.value
      });
    } catch (e) {
      console.error("Error enviando comando:", e);
      throw e;
    }
  }

  return {
    currentTvIp,
    currentTvModel,
    selectTv,
    sendKey
  };
}