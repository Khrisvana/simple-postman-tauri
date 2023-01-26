<script setup lang="ts">
import { storeToRefs } from "pinia";
import { ref } from "vue";
import { useApiRunnerStore } from '../stores/apiRunner';
 
const store = useApiRunnerStore(); 
const { currentRequestConfig } = storeToRefs(store); 

let method = ref("get");

async function runUrl() {
  await store.runApi()
}
</script>


<template>
  <form class="flex gap-1 sticky top-2 bg-white z-10" @submit.prevent="runUrl()">
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
      v-model="currentRequestConfig.method"
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
      v-model="currentRequestConfig.url"
    />
    <button
      class="
        rounded
        bg-sky-500
        text-sm text-white
        px-3
        py-2
        hover:border-sky-700 hover:bg-sky-700
      "
      type="submit"
    >
      Run
    </button>
  </form>
</template>