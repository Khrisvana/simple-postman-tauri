<script lang="ts" setup>
import { computed, onMounted, ref } from "@vue/runtime-core";
import NestedDraggable from "../components/sidebar/NestedDraggable.vue";
import { useSidebarStore } from "../stores/sidebar";
import { invoke } from '@tauri-apps/api/tauri';

const store = useSidebarStore();
const invoke = window.__TAURI__.invoke;

let queryRes = ref(null);

onMounted(async () => { 
  
  queryRes.value = await invoke('get_requests')
  console.log(queryRes.value);
})

interface Elements {
  id: Number;
  name: String;
  elements: Array<Elements>;
}

let elements = computed({
  get() {
    return store.elements;
  },
  set(value: any) {
    store.updateElements(value);
  },
});
</script>

<template>
  <div
    class="
      container
      p-3
      justify-start
      bg-white
      ml-60
      h-auto
      min-h-screen
      relative
    "
  >
    <NestedDraggable class="w-full" v-model="elements" />

    <pre>
      {{ queryRes }}
    </pre>
  </div>
</template>