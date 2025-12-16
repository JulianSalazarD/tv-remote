<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { RefreshCw, Tv, Monitor, Wifi } from 'lucide-vue-next';
import { useTvConnection } from '../composables/useTvConnection';

// 1. Definimos los eventos hacia el padre (App.vue)
const emit = defineEmits<{
  (e: 'change-tab', tab: string): void
}>();

// 2. Traemos la lógica de conexión global
const { selectTv } = useTvConnection();

// 3. Definimos la estructura exacta que nos envía Rust
interface TVDevice {
  id: string;
  name: string;
  ip: string;
  location: string;
  model: string; // "Samsung", "LG", "Roku", "Android TV", etc.
}

// Estado local de la vista
const devices = ref<TVDevice[]>([]);
const isScanning = ref(false);
const errorMsg = ref('');

// Función para llamar al backend (Rust)
async function startScan() {
  isScanning.value = true;
  devices.value = []; // Limpiamos lista anterior
  errorMsg.value = '';

  try {
    console.log("Invocando escáner SSDP...");
    // Llamada al comando Rust 'scan_devices'
    const found = await invoke<TVDevice[]>('scan_devices');
    
    // Filtro opcional: Eliminar duplicados por IP si Rust no lo hizo
    devices.value = found;
    
  } catch (e) {
    console.error(e);
    errorMsg.value = "Error al escanear: " + e;
  } finally {
    isScanning.value = false;
  }
}

// Función al tocar una tarjeta de TV
function handleSelectDevice(ip: string, model: string) {
  // Guardamos IP y MARCA (Crucial para saber si usar puerto 8002 o 3000)
  selectTv(ip, model);
  
  // Pedimos cambiar a la vista del control remoto
  emit('change-tab', 'remote');
}

// Escanear automáticamente al entrar
onMounted(() => {
  startScan();
});
</script>

<template>
  <div class="scan-container">
    
    <div class="header">
        <h2>Dispositivos</h2>
        <button class="scan-btn" @click="startScan" :disabled="isScanning">
            <RefreshCw :class="{ 'spinning': isScanning }" :size="20" />
        </button>
    </div>

    <div v-if="isScanning && devices.length === 0" class="status-box">
      <Wifi class="pulse-icon" :size="48" color="#3b5bdb" />
      <p>Buscando TVs en tu red Wi-Fi...</p>
    </div>

    <div v-else-if="devices.length > 0" class="device-list">
      
      <div 
        v-for="device in devices" 
        :key="device.id" 
        class="device-card"
        @click="handleSelectDevice(device.ip, device.model)"
      >
        <div class="icon-box" :class="device.model.toLowerCase()">
          <Monitor v-if="device.model === 'Desconocido'" />
          <Tv v-else />
        </div>

        <div class="info">
          <span class="model-name">{{ device.model }} TV</span>
          <span class="friendly-name">{{ device.name }}</span>
          <span class="ip-addr">{{ device.ip }}</span>
        </div>

        <div class="chevron">›</div>
      </div>

      <p class="scan-hint">¿No ves tu TV? Asegúrate de que esté encendida.</p>
    </div>

    <div v-else class="empty-state">
      <div class="empty-content">
        <Monitor :size="48" color="#555" />
        <h3>No se encontraron TVs</h3>
        <p>Verifica que tu celular y la TV estén conectados a la misma red Wi-Fi.</p>
        <p v-if="errorMsg" class="error-text">{{ errorMsg }}</p>
        
        <button class="retry-btn" @click="startScan">
          Intentar de nuevo
        </button>
      </div>
    </div>

  </div>
</template>

<style scoped>
.scan-container {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}
.header h2 { margin: 0; font-size: 24px; font-weight: 700; }

.scan-btn {
    background: var(--surface-color);
    border: none;
    color: var(--text-primary);
    width: 40px; height: 40px;
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
    cursor: pointer;
}
.spinning { animation: spin 1s linear infinite; }
.pulse-icon { animation: pulse 2s infinite; margin-bottom: 15px; }

@keyframes spin { 100% { transform: rotate(360deg); } }
@keyframes pulse { 0% { opacity: 0.5; } 50% { opacity: 1; } 100% { opacity: 0.5; } }

/* --- Lista de Dispositivos --- */
.device-list {
    flex: 1;
    overflow-y: auto;
}

.device-card {
  background: var(--surface-color);
  padding: 16px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 12px;
  cursor: pointer;
  transition: transform 0.1s, background 0.2s;
  border: 1px solid transparent;
}

.device-card:active {
    transform: scale(0.98);
    background: #303238;
}

.icon-box {
  background: #303238;
  width: 50px; height: 50px;
  border-radius: 12px;
  display: flex; align-items: center; justify-content: center;
  color: var(--text-secondary);
}
/* Colores por marca (opcional) */
.icon-box.samsung { color: #0b5ed7; background: rgba(11, 94, 215, 0.1); }
.icon-box.lg { color: #d63384; background: rgba(214, 51, 132, 0.1); }

.info { flex: 1; display: flex; flex-direction: column; }
.model-name { font-weight: 700; font-size: 16px; color: var(--text-primary); }
.friendly-name { font-size: 12px; color: var(--text-secondary); margin-bottom: 4px; }
.ip-addr { 
    font-family: monospace; font-size: 11px; 
    background: rgba(255,255,255,0.05); 
    padding: 2px 6px; border-radius: 4px; 
    align-self: flex-start;
    color: var(--accent-color);
}

.chevron {
    color: var(--text-secondary);
    font-size: 24px;
    font-weight: 300;
}

.scan-hint {
    text-align: center; font-size: 12px; color: var(--text-secondary); margin-top: 20px;
}

/* --- Estados Vacíos --- */
.status-box, .empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: var(--text-secondary);
}

.empty-content { max-width: 250px; }
.empty-content h3 { color: var(--text-primary); margin-top: 10px; }
.empty-content p { font-size: 14px; line-height: 1.5; margin-bottom: 20px; }
.error-text { color: #ff6b6b; font-size: 12px; }

.retry-btn {
  background: var(--accent-color);
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 25px;
  font-weight: 600;
  font-size: 14px;
}
</style>