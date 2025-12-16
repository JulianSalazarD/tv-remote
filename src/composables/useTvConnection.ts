import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Estado global (fuera de la función para que se comparta entre vistas)
const currentTvIp = ref<string>(localStorage.getItem("saved_tv_ip") || "");

export function useTvConnection() {
  // Guardar IP seleccionada
  function selectTv(ip: string) {
    currentTvIp.value = ip;
    localStorage.setItem("saved_tv_ip", ip);
    console.log("TV Seleccionada:", ip);
  }

  // Enviar tecla a Rust
  async function sendKey(key: string) {
    if (!currentTvIp.value) {
      console.warn("No hay TV seleccionada");
      return;
    }

    try {
      await invoke("send_key", { ip: currentTvIp.value, key: key });
    } catch (e) {
      console.error("Error enviando comando:", e);
    }
  }

  return {
    currentTvIp,
    selectTv,
    sendKey,
  };
}
