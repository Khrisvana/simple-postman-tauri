<script lang="ts" setup>
import { computed, onMounted, ref } from "@vue/runtime-core";
import { useSidebarStore } from "../stores/sidebar";

const store = useSidebarStore();

interface Elements {
  id: Number;
  name: String;
  elements: Array<Elements>;
}
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
  
  </div>
</template>