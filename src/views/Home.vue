<script lang="ts" setup>
import { computed } from "@vue/runtime-core";
import NestedDraggable from "../components/inputs/NestedDraggable.vue";
import { useSidebarStore } from "../stores/sidebar";

const store = useSidebarStore();

interface Elements {
  id: Number;
  name: String;
  elements: Array<Elements>;
}

let elements = computed({
  get() {
    return store.elements;
  },
  set(value: Array<Elements>) {
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
    <NestedDraggable class="col-8" :list="elements" />

    <div class="bg-gray-500 h-60 p-3"></div>
  </div>
</template>