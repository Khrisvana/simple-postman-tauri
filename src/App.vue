<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import Sidebar from "./components/Sidebar.vue";
import VueJsonPretty from 'vue-json-pretty';
import 'vue-json-pretty/lib/styles.css';

import axios from 'axios'
import { computed, ref } from 'vue'

let url = ref('');
let result = ref(null);

async function runUrl() { 
  let json = await axios.get(url.value);
  result.value = json.data;
}

const prettyResult = computed(() => {
  if (!result.value) {
    return '';
  }

  return JSON.stringify(result.value, undefined, 2);
});
  // result.value = JSON.stringify(json.data, undefined, 2);
</script>

<template>
  <div class="flex">
    <Sidebar />
    <div class="container p-3 justify-start bg-white ml-60 h-auto min-h-screen relative">
      <div class="flex gap-1 sticky top-2 bg-white z-10">
        <input
          type="text"
          class="
            px-3
            py-2
            block
            rounded
            bg-white
            border-slate-300
            w-full
            focus:border-sky-500
            focus:outline-none
            focus:ring-1
            focus:ring-sky-500
            text-sm
            shadow-sm
          "
          v-model="url"
        />
        <button class="rounded bg-sky-500 text-sm text-white px-3 py-2" @click.prevent="runUrl()">
          Run
        </button>
      </div>

      <!-- <pre class="w-full h-full text-sm text-left bg-white py-3">{{ prettyResult }}</pre> -->
      <vue-json-pretty :data="result" showLineNumber showIcon/>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
