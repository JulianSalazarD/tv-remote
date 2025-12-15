<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCw, Tv, Monitor } from "lucide-vue-next";

// Tipo idéntico al struct de Rust
interface TVDevice {
    id: string;
    name: string;
    ip: string;
    location: string;
    model: string;
}

const devices = ref<TVDevice[]>([]);
const isScanning = ref(false);
const errorMsg = ref("");

async function startScan() {
    isScanning.value = true;
    devices.value = [];
    errorMsg.value = "";

    try {
        // Llamada a Rust
        console.log("Invocando scan_devices...");
        const found = await invoke<TVDevice[]>("scan_devices");
        console.log("Encontrados:", found);
        devices.value = found;
    } catch (e) {
        console.error(e);
        errorMsg.value = "Error al escanear: " + e;
    } finally {
        isScanning.value = false;
    }
}

// Escanear automáticamente al entrar en la vista
onMounted(() => {
    startScan();
});
</script>

<template>
    <div class="scan-container">
        <div v-if="isScanning" class="status-box">
            <RefreshCw class="spin-icon" :size="32" color="#3b5bdb" />
            <p>Escaneando red Wi-Fi...</p>
        </div>

        <div v-else-if="devices.length > 0" class="device-list">
            <h3>Dispositivos encontrados:</h3>

            <div
                v-for="device in devices"
                :key="device.id"
                class="device-card"
                @click="console.log('Seleccionado:', device.ip)"
            >
                <div class="icon-box">
                    <Monitor v-if="device.model === 'Desconocido'" />
                    <Tv v-else color="#3b5bdb" />
                </div>
                <div class="info">
                    <span class="name">{{ device.model }} TV</span>
                    <span class="ip">{{ device.ip }}</span>
                    <span class="raw-name">{{ device.name }}</span>
                </div>
            </div>

            <button class="retry-btn" @click="startScan">
                Escanear de nuevo
            </button>
        </div>

        <div v-else class="empty-state">
            <p>No se encontraron TVs.</p>
            <p class="sub">Verifica que estén en la misma red Wi-Fi.</p>
            <p v-if="errorMsg" class="error">{{ errorMsg }}</p>
            <button class="retry-btn" @click="startScan">Reintentar</button>
        </div>
    </div>
</template>

<style scoped>
.scan-container {
    padding: 20px;
    height: 100%;
    overflow-y: auto;
}

.spin-icon {
    animation: spin 1s linear infinite;
    margin-bottom: 10px;
}
@keyframes spin {
    100% {
        transform: rotate(360deg);
    }
}

.status-box,
.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    margin-top: 50px;
    color: var(--text-secondary);
}

.device-list h3 {
    font-size: 14px;
    margin-bottom: 15px;
    color: var(--text-secondary);
}

.device-card {
    background: var(--surface-color);
    padding: 15px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    gap: 15px;
    margin-bottom: 10px;
    cursor: pointer;
    transition: background 0.2s;
}
.device-card:active {
    background: #373a40;
}

.icon-box {
    background: rgba(59, 91, 219, 0.1);
    padding: 10px;
    border-radius: 50%;
    display: flex;
}

.info {
    display: flex;
    flex-direction: column;
}
.name {
    font-weight: bold;
    color: var(--text-primary);
}
.ip {
    font-size: 12px;
    color: var(--accent-color);
    font-family: monospace;
}
.raw-name {
    font-size: 10px;
    color: var(--text-secondary);
    margin-top: 2px;
}

.retry-btn {
    margin-top: 20px;
    background: transparent;
    border: 1px solid var(--accent-color);
    color: var(--accent-color);
    padding: 10px 20px;
    border-radius: 20px;
    font-size: 14px;
}
.error {
    color: #ff6b6b;
    font-size: 12px;
    margin-top: 10px;
}
</style>
