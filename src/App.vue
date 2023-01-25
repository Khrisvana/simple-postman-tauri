<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import Sidebar from "./components/Sidebar.vue";
import ApiRunner from "./components/ApiRunner.vue";
import VueJsonPretty from "vue-json-pretty";
import "vue-json-pretty/lib/styles.css";

import { ref } from "vue";

let result = ref(null);

function getResult(data: any) {
  result.value = data
}
</script>

<template>
  <div class="flex">
    <Sidebar />
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
      
      <ApiRunner @result="getResult"/> 
      <vue-json-pretty :data="result" showLineNumber showIcon class="mt-3"/>
    </div>
  </div>
</template>

<style>
.vjs-carets {
  /* transform: translateX(-100%)!important; */
  right: 100%!important;
}
</style>