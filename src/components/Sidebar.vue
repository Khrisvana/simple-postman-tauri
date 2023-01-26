<script lang="ts" setup>
import { computed, onMounted } from '@vue/runtime-core';
import { storeToRefs } from 'pinia';
import { useApiRunnerStore } from '../stores/apiRunner';

const store = useApiRunnerStore();  

onMounted(async () => {
    await store.readFiles()
})

let getFullData = computed(() => store.fullData ?? [])
</script>

<template>
  <div class="w-60 h-screen shadow-md bg-white px-1 z-10 fixed">
    <ul class="relative">
      <router-link
          class="
            flex
            items-center
            text-sm
            py-4
            px-6
            h-12
            overflow-hidden
            text-gray-700 text-ellipsis
            whitespace-nowrap
            rounded
            hover:text-gray-900 hover:bg-gray-100
            transition
            duration-300
            ease-in-out
            gap-3
          "
          :to="{name: 'Home'}"
          data-mdb-ripple="true"
          data-mdb-ripple-color="dark"
          ><span class="uppercase text-amber-400 font-medium">home</span></router-link
        >
      <li class="relative">
        <span
          class="
            flex
            items-center
            text-sm
            py-4
            px-6
            h-12
            overflow-hidden
            text-gray-700 text-ellipsis
            whitespace-nowrap
            rounded
            hover:text-gray-900 hover:bg-gray-100
            transition
            duration-300
            ease-in-out
            gap-3
            cursor-pointer
          " 
          href="#"
          data-mdb-ripple="true"
          data-mdb-ripple-color="dark"
          @click="store.addNewRequest"
          ><span class="uppercase text-green-500 font-medium">+</span>Add Request</span
        >
      </li>
      <li class="relative" v-for="item in getFullData" :key="item.key">
        <router-link
          class="
            flex
            items-center
            text-sm
            py-4
            px-6
            h-12
            overflow-hidden
            text-gray-700 text-ellipsis
            whitespace-nowrap
            rounded
            hover:text-gray-900 hover:bg-gray-100
            transition
            duration-300
            ease-in-out
            gap-3
          "
          :to="{name: 'Request', params: {id: item.key} }"
          data-mdb-ripple="true"
          data-mdb-ripple-color="dark"
          ><span class="uppercase text-amber-400 font-medium">{{ item.method }}</span>{{item.name}}</router-link
        >
      </li>  
    </ul>
  </div>
</template>

<style scoped>
</style>