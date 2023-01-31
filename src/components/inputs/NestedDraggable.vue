<script lang="ts" setup>
import { computed } from "@vue/runtime-core";
import draggable from "vuedraggable";

let props = defineProps({
  list: {
    default: null,
    type: Array,
    required: true,
  },
});
</script>

<template>
  <draggable
    v-bind="dragOption"
    tag="ul"
    class="dragArea px-3"
    :list="list"
    item-key="name"
  >
    <template #item="{ element }">
      <li>
        <p>{{ element.name }}</p>
        <nested-draggable :list="element.elements" />
      </li>
    </template>
  </draggable>
</template>

<style scoped>
.dragArea {
  min-height: 50px;
  outline: 1px dashed;
}
</style>