<script setup lang="ts">
// Props para configurar el botón
defineProps<{
    label?: string; // Texto opcional
    variant?: "round" | "pill" | "pad"; // Forma del botón
    color?: "default" | "danger" | "primary";
}>();

// Haptic Feedback simple (Vibración)
const vibrate = () => {
    if (navigator.vibrate) navigator.vibrate(15); // Vibra 15ms
};
</script>

<template>
    <button
        class="btn"
        :class="[variant || 'round', color || 'default']"
        @click="vibrate"
    >
        <slot></slot>
        <span v-if="label" class="label">{{ label }}</span>
    </button>
</template>

<style scoped>
.btn {
    border: none;
    background: var(--surface-color);
    color: var(--text-primary);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition:
        transform 0.1s,
        background 0.2s;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    /* Efecto active nativo */
    -webkit-tap-highlight-color: transparent;
}

.btn:active {
    transform: scale(0.92);
    background: #2c2e33;
}

/* Variantes de forma */
.round {
    width: 60px;
    height: 60px;
    border-radius: 50%;
}

.pill {
    width: 60px;
    height: 100px; /* Alargado verticalmente para Vol/Ch */
    border-radius: 30px;
}

.pad {
    width: 100%;
    height: 100%;
    background: transparent;
    box-shadow: none;
}

/* Colores */
.danger {
    color: #ff6b6b;
}
.primary {
    color: var(--accent-color);
}

.label {
    font-size: 10px;
    margin-top: 4px;
    color: var(--text-secondary);
    font-weight: 600;
}
</style>
