<script setup lang="ts">
import { ref } from "vue";
import BottomNav from "./components/BottomNav.vue";
import RemoteView from "./views/RemoteView.vue";
import ScanView from "./views/ScanView.vue";

// Estado para controlar qué vista se muestra
const currentTab = ref("remote"); // 'remote' | 'scan' | 'settings'

// Componente placeholder simple para ajustes
const SettingsPlaceholder = {
    template:
        '<div style="padding:20px"><h2>Ajustes</h2><p>Próximamente...</p></div>',
};
</script>

<template>
    <main class="app-content">
        <Transition name="fade" mode="out-in">
            <div v-if="currentTab === 'remote'" key="remote">
                <RemoteView />
            </div>

            <div v-else-if="currentTab === 'scan'" key="scan">
                <ScanView @change-tab="(tab) => (currentTab = tab)" />
            </div>

            <div v-else-if="currentTab === 'settings'" key="settings">
                <component :is="SettingsPlaceholder" />
            </div>
        </Transition>
    </main>

    <BottomNav
        :activeTab="currentTab"
        @change-tab="(tab) => (currentTab = tab)"
    />
</template>

<style scoped>
.app-content {
    /* Restamos la altura de la barra (70px) para que no tape contenido */
    height: calc(100vh - 70px);
    overflow-y: auto;
}

/* Animación de desvanecimiento simple */
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}
</style>
