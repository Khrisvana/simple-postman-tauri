<script setup lang="ts">
import { getClient } from '@tauri-apps/api/http';
import { ref } from "vue";

const emit = defineEmits(['result']);
let url = ref("");
let method = ref("get");

async function runUrl() {
  let response = null;
  const client = await getClient();
  if (method.value === 'get') {
    response = await client.get(url.value);  
  } else if (method.value === 'post') {
    response = await client.get(url.value);  
  }
 
  emit('result', response?.data);
} 
</script>


<template>
  <div class="flex gap-1 sticky top-2 bg-white z-10">
    <select
      name=""
      id=""
      class="
        px-3
        py-2
        block
        rounded
        bg-white
        border-2 border-slate-300
        focus:border-sky-500 focus:outline-none focus:ring-1 focus:ring-sky-500
        text-sm
        shadow-sm
      "
      v-model="method"
    >
      <option value="get">GET</option>
      <option value="method">POST</option>
    </select>
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
        focus:border-sky-500 focus:outline-none focus:ring-1 focus:ring-sky-500
        text-sm
        shadow-sm
      "
      v-model="url"
    />
    <button
      class="rounded bg-sky-500 text-sm text-white px-3 py-2"
      @click.prevent="runUrl()"
    >
      Run
    </button>
  </div>
</template>