import { createApp } from "vue"; 
import { createPinia } from 'pinia'
import App from "./App.vue";

import router from "./router";
import "./style.css";
import "./index.css";

/* fontawesome */
import fontAwesome from './helpers/font-awesome' 

const app = createApp(App)
const pinia = createPinia()

app.use(pinia);
app.use(fontAwesome);
app.use(router);
app.mount("#app"); 