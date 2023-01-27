<script lang="ts" setup>
import Dragable from "../components/inputs/Dragable.vue";
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
    <Dragable />
    <div class="bg-gray-500 h-60 p-3"></div>
  </div>
</template>