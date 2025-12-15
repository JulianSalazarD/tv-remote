import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Estado global simple
const currentTvIp = ref<string | null>(null);
const isConnected = ref(false);

export function useTvConnection() {
  // Función para buscar TVs (Llama a Rust)
  async function scanDevices() {
    console.log("Escaneando...");
    // Aquí invocaremos al comando 'scan_network' de Rust más adelante
    // const devices = await invoke('scan_network');
    // return devices;
  }

  // Función para enviar tecla
  async function sendKey(key: string) {
    if (!currentTvIp.value) return;

    console.log(`Enviando ${key} a ${currentTvIp.value}`);
    await invoke("send_command", {
      ip: currentTvIp.value,
      key: key,
    });
  }

  return {
    currentTvIp,
    isConnected,
    scanDevices,
    sendKey,
  };
}
