import { createApp } from "vue";
import { invoke } from '@tauri-apps/api'
import App from "./App.vue";

import "./style.css";
import "./index.css";

createApp(App).mount("#app");

async function greet() {
    await invoke('greet', { name: 'applied.math.coding' });
}