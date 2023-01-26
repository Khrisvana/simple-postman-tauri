<script lang="ts" setup>
import { onMounted } from "@vue/runtime-core";
import Sidebar from "../Sidebar.vue";
// import { useApiRunnerStore } from '../../stores/apiRunner'

// const store = useApiRunnerStore();

// onMounted(async () => {
//     await store.readFiles()
// })
</script>

<template>
    <Sidebar />
    <RouterView/>
</template>