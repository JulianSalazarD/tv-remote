<script setup lang="ts">
import { ref } from 'vue';
import RemoteButton from '../components/RemoteButton.vue';
import { useTvConnection } from '../composables/useTvConnection'; 
import { 
  Power, Menu, ChevronUp, ChevronDown, ChevronLeft, ChevronRight, 
  Volume2, Volume1, ArrowLeft, Home 
} from 'lucide-vue-next';

// 1. Traemos la función de conexión desde nuestro composable
const { sendKey, currentTvIp } = useTvConnection();

// 2. Estado para mostrar feedback visual al usuario
const statusMsg = ref('Listo');
const isError = ref(false);

// 3. Función principal que envía el comando
async function send(cmd: string) {
  // Validación rápida
  if (!currentTvIp.value) {
    statusMsg.value = "¡Selecciona una TV primero!";
    isError.value = true;
    return;
  }

  statusMsg.value = "Enviando...";
  isError.value = false;

  try {
    // Llamada real al Backend (Rust)
    await sendKey(cmd);
    
    // Si llegamos aquí, Rust no dio error
    statusMsg.value = "Comando enviado OK";
    
    // Volver a mostrar la IP después de 2 segundos
    setTimeout(() => {
        if (!isError.value) statusMsg.value = "Conectado";
    }, 2000);

  } catch (e) {
    console.error(e);
    statusMsg.value = "Error de conexión";
    isError.value = true;
  }
}
</script>

<template>
  <div class="remote-container">
    
    <div class="status-bar" :class="{ 'error': isError }">
      <div class="indicator" :class="{ 'active': !isError && currentTvIp }"></div>
      <span>{{ statusMsg }}</span>
      <span v-if="currentTvIp" class="ip-tag">{{ currentTvIp }}</span>
    </div>

    <header class="row-header">
      <RemoteButton variant="round" @click="send('SOURCE')">
        <span style="font-size:12px; font-weight:bold">INPUT</span>
      </RemoteButton>
      <RemoteButton variant="round" color="danger" @click="send('POWER')">
        <Power :size="24" />
      </RemoteButton>
    </header>

    <div class="d-pad-area">
      <div class="d-pad-circle">
        <RemoteButton class="pad-btn up" variant="pad" @click="send('UP')">
          <ChevronUp />
        </RemoteButton>
        <RemoteButton class="pad-btn left" variant="pad" @click="send('LEFT')">
          <ChevronLeft />
        </RemoteButton>
        <RemoteButton class="pad-btn center" variant="round" @click="send('OK')">
          <span style="font-weight:900">OK</span>
        </RemoteButton>
        <RemoteButton class="pad-btn right" variant="pad" @click="send('RIGHT')">
          <ChevronRight />
        </RemoteButton>
        <RemoteButton class="pad-btn down" variant="pad" @click="send('DOWN')">
          <ChevronDown />
        </RemoteButton>
      </div>
    </div>

    <div class="row-actions">
      <RemoteButton variant="round" @click="send('BACK')">
        <ArrowLeft :size="20" />
        <span style="font-size:9px">BACK</span>
      </RemoteButton>
      <RemoteButton variant="round" @click="send('HOME')">
        <Home :size="20" />
      </RemoteButton>
      <RemoteButton variant="round" @click="send('MENU')">
        <Menu :size="20" />
      </RemoteButton>
    </div>

    <div class="row-controls">
      <div class="vertical-control">
        <RemoteButton variant="pad" @click="send('VOL_UP')"><Volume2 /></RemoteButton>
        <span class="control-label">VOL</span>
        <RemoteButton variant="pad" @click="send('VOL_DOWN')"><Volume1 /></RemoteButton>
      </div>
      
      <div class="vertical-control">
        <RemoteButton variant="pad" @click="send('CH_UP')"><ChevronUp /></RemoteButton>
        <span class="control-label">CH</span>
        <RemoteButton variant="pad" @click="send('CH_DOWN')"><ChevronDown /></RemoteButton>
      </div>
    </div>

  </div>
</template>

<style scoped>
.remote-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 10px 30px;
  gap: 20px;
  justify-content: space-evenly;
  max-width: 400px;
  margin: 0 auto;
}

/* --- Estilos de la Barra de Estado --- */
.status-bar {
  background: #25262b;
  padding: 8px 15px;
  border-radius: 20px;
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 12px;
  color: var(--text-secondary);
  border: 1px solid #373a40;
  min-height: 36px;
}

.status-bar.error {
  border-color: #ff6b6b;
  color: #ff6b6b;
}

.indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #555;
  transition: all 0.3s;
}

.indicator.active {
  background: #40c057; /* Verde cuando hay IP seleccionada */
  box-shadow: 0 0 8px rgba(64, 192, 87, 0.4);
}

.ip-tag {
  margin-left: auto;
  font-family: monospace;
  background: rgba(255,255,255,0.1);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 10px;
}

/* --- Estilos del Control --- */
.row-header {
  display: flex;
  justify-content: space-between;
}

.d-pad-area {
  display: flex;
  justify-content: center;
  align-items: center;
}

.d-pad-circle {
  position: relative;
  width: 220px;
  height: 220px;
  background: #2c2e33;
  border-radius: 50%;
  display: grid;
  grid-template-areas: 
    ". up ."
    "left center right"
    ". down .";
  grid-template-rows: 1fr 1.2fr 1fr;
  grid-template-columns: 1fr 1.2fr 1fr;
  place-items: center;
  box-shadow: inset 0 0 15px rgba(0,0,0,0.5);
}

.pad-btn { color: #e0e0e0; }
.pad-btn.up { grid-area: up; }
.pad-btn.down { grid-area: down; }
.pad-btn.left { grid-area: left; }
.pad-btn.right { grid-area: right; }
.pad-btn.center { 
  grid-area: center; 
  background: #3b5bdb; 
  width: 70px; height: 70px;
  box-shadow: 0 4px 10px rgba(0,0,0,0.4);
}

.row-actions {
  display: flex;
  justify-content: space-between;
  padding: 0 10px;
}

.row-controls {
  display: flex;
  justify-content: space-between;
  padding: 0 20px;
}

.vertical-control {
  background: var(--surface-color);
  width: 70px;
  height: 140px;
  border-radius: 35px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 10px 0;
  align-items: center;
  box-shadow: 0 4px 6px rgba(0,0,0,0.2);
}

.control-label {
  font-size: 12px;
  font-weight: bold;
  color: var(--text-secondary);
  pointer-events: none;
}
</style>