<script setup lang="ts">
import RemoteButton from "../components/RemoteButton.vue";
import {
    Power,
    Menu,
    ChevronUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    Volume2,
    Volume1,
    ArrowLeft,
    Home,
} from "lucide-vue-next";

// Función temporal para simular acciones
const send = (cmd: string) => console.log("Enviando:", cmd);
</script>

<template>
    <div class="remote-container">
        <header class="row-header">
            <RemoteButton variant="round" @click="send('SOURCE')">
                <span style="font-size: 12px; font-weight: bold">INPUT</span>
            </RemoteButton>
            <RemoteButton variant="round" color="danger" @click="send('POWER')">
                <Power :size="24" />
            </RemoteButton>
        </header>

        <div class="d-pad-area">
            <div class="d-pad-circle">
                <RemoteButton
                    class="pad-btn up"
                    variant="pad"
                    @click="send('UP')"
                >
                    <ChevronUp />
                </RemoteButton>
                <RemoteButton
                    class="pad-btn left"
                    variant="pad"
                    @click="send('LEFT')"
                >
                    <ChevronLeft />
                </RemoteButton>
                <RemoteButton
                    class="pad-btn center"
                    variant="round"
                    @click="send('OK')"
                >
                    <span style="font-weight: 900">OK</span>
                </RemoteButton>
                <RemoteButton
                    class="pad-btn right"
                    variant="pad"
                    @click="send('RIGHT')"
                >
                    <ChevronRight />
                </RemoteButton>
                <RemoteButton
                    class="pad-btn down"
                    variant="pad"
                    @click="send('DOWN')"
                >
                    <ChevronDown />
                </RemoteButton>
            </div>
        </div>

        <div class="row-actions">
            <RemoteButton variant="round" @click="send('BACK')">
                <ArrowLeft :size="20" />
                <span style="font-size: 9px">BACK</span>
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
                <RemoteButton variant="pad" @click="send('VOL_UP')"
                    ><Volume2
                /></RemoteButton>
                <span class="control-label">VOL</span>
                <RemoteButton variant="pad" @click="send('VOL_DOWN')"
                    ><Volume1
                /></RemoteButton>
            </div>

            <div class="vertical-control">
                <RemoteButton variant="pad" @click="send('CH_UP')"
                    ><ChevronUp
                /></RemoteButton>
                <span class="control-label">CH</span>
                <RemoteButton variant="pad" @click="send('CH_DOWN')"
                    ><ChevronDown
                /></RemoteButton>
            </div>
        </div>
    </div>
</template>

<style scoped>
.remote-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 20px 30px;
    gap: 30px;
    justify-content: space-evenly;
    max-width: 400px;
    margin: 0 auto;
}

/* Cabecera */
.row-header {
    display: flex;
    justify-content: space-between;
}

/* D-Pad Styling (La parte compleja) */
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
    box-shadow: inset 0 0 15px rgba(0, 0, 0, 0.5);
}
.pad-btn {
    color: #e0e0e0;
}
.pad-btn.up {
    grid-area: up;
}
.pad-btn.down {
    grid-area: down;
}
.pad-btn.left {
    grid-area: left;
}
.pad-btn.right {
    grid-area: right;
}
.pad-btn.center {
    grid-area: center;
    background: #3b5bdb;
    width: 70px;
    height: 70px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.4);
}

/* Acciones centrales */
.row-actions {
    display: flex;
    justify-content: space-between;
    padding: 0 10px;
}

/* Volumen y Canal */
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
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.2);
}
.control-label {
    font-size: 12px;
    font-weight: bold;
    color: var(--text-secondary);
    pointer-events: none;
}
</style>
