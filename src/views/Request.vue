<script setup lang="ts">
/* 
  Next Progress: Add new request and change its name
*/

import { computed, onBeforeUnmount, onMounted, watch as vueWatcher } from "vue";
import {
  register,
  unregister,
  isRegistered,
} from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";
import { useApiRunnerStore } from "../stores/apiRunner";

//components
import ApiRunner from "../components/ApiRunner.vue";
import VueJsonPretty from "vue-json-pretty";
import "vue-json-pretty/lib/styles.css";
import { useRoute } from "vue-router";

const route = useRoute();
const store = useApiRunnerStore();
const currentRequestResult = computed(() => store.currentRequestResult);

const unlisten = async () => {
  let commandRegistered = await isRegistered("CommandOrControl+S");
  await appWindow.onFocusChanged(async ({ payload: focused }) => {
    if (focused) {
      if (!commandRegistered) {
        await register("CommandOrControl+S", async () => {
          await store.storeDataFile();
        });
      }
    } else {
      if (commandRegistered) {
        await unregister("CommandOrControl+S");
      }
    }
  });
};

onMounted(async () => {
  // await unlisten();
  // if (!store.fullData) {
  //   await store.readFiles();
  // }
  store.currentPageConfig(parseInt(route.params.id.toString()));
});

vueWatcher(route, async (newValue, oldValue) => {
  store.currentPageConfig(parseInt(newValue.params.id.toString()));
});

// onBeforeUnmount(async () => {
//   // let commandRegistered = await isRegistered('CommandOrControl+S');
//   await unregister("CommandOrControl+S");
// });
</script>

<template>
  <div class="flex">
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
      <ApiRunner />
      <VueJsonPretty
        :data="currentRequestResult"
        showLineNumber
        showIcon
        class="mt-3"
      />
    </div>
  </div>
</template>