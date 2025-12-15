<script setup lang="ts">
import { Tv, Search, Settings } from "lucide-vue-next";

// Definimos las props para saber qué pestaña está activa
defineProps<{
    activeTab: string;
}>();

// Emitimos evento al padre cuando se pulsa una pestaña
const emit = defineEmits(["change-tab"]);
</script>

<template>
    <nav class="bottom-nav">
        <button
            class="nav-item"
            :class="{ active: activeTab === 'remote' }"
            @click="emit('change-tab', 'remote')"
        >
            <Tv :size="24" />
            <span>Remote</span>
        </button>

        <button
            class="nav-item"
            :class="{ active: activeTab === 'scan' }"
            @click="emit('change-tab', 'scan')"
        >
            <Search :size="24" />
            <span>Devices</span>
        </button>

        <button
            class="nav-item"
            :class="{ active: activeTab === 'settings' }"
            @click="emit('change-tab', 'settings')"
        >
            <Settings :size="24" />
            <span>Settings</span>
        </button>
    </nav>
</template>

<style scoped>
.bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 70px; /* Altura cómoda para pulgares */
    background-color: #25262b; /* Un poco más claro que el fondo negro */
    border-top: 1px solid #373a40;
    display: flex;
    justify-content: space-around;
    align-items: center;
    z-index: 100;
    padding-bottom: env(safe-area-inset-bottom); /* Para iPhones sin botón */
}

.nav-item {
    background: none;
    border: none;
    color: var(--text-secondary);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    width: 60px;
}

.nav-item span {
    font-size: 10px;
}

/* Estado Activo */
.nav-item.active {
    color: var(--accent-color); /* El azul que definimos antes */
    transform: translateY(-2px);
}

.nav-item.active span {
    font-weight: bold;
}
</style>
