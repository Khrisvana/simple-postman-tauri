<script lang="ts" setup>
import draggable from "vuedraggable";

const emit = defineEmits(["update:modelValue"]);

let props = defineProps({
  modelValue: {
    default: null,
    type: Array,
  },
});
</script>

<template>
  <draggable
    tag="ul"
    class="dragArea px-3"
    :list="props.modelValue"
    item-key="name"
    group="name"
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