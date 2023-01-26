<script setup lang="ts"> 
/* 
  Reminder: use pinia to manage state through component instead of emit
  Next Progress: Read File then display on sidebar
*/

import { onBeforeUnmount, ref } from "vue";
import { BaseDirectory, createDir, writeFile } from "@tauri-apps/api/fs";
import { register, unregister, isRegistered } from '@tauri-apps/api/globalShortcut';
import { appWindow } from "@tauri-apps/api/window";

//components
import ApiRunner from "../components/ApiRunner.vue";
import VueJsonPretty from "vue-json-pretty";
import "vue-json-pretty/lib/styles.css";

let result = ref(null);
let fullResult = ref({
  methods: 'get',
  url: '',
})

function getResult(data: any) {
  result.value = data
}

const createDataFolder = async () => {
  try {
    await createDir("Simple Postman", {
      dir: BaseDirectory.Document,
      recursive: true
    });
  } catch (error) {
    console.log(error);
  }
}

const createDataFile = async () => {
  try { 
    await createDataFolder();
    await writeFile(
      {
        contents: JSON.stringify(fullResult.value),
        path: `./Simple Postman/data.json`
      },
      {
        dir: BaseDirectory.Document
      }
    )
  } catch (error) {
    console.log(error);
  }
}

const unlisten = async () => {
  await appWindow.onFocusChanged(async ({ payload: focused }) => {
    let commandRegistered = await isRegistered('CommandOrControl+S');
    if (focused) {
      if (!commandRegistered) {
        await register('CommandOrControl+S', createDataFile);
      }
    } else {
      if (commandRegistered) {
        await unregister('CommandOrControl+S'); 
      }
    }
  })
};

unlisten() 
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
      <ApiRunner @result="getResult" v-model="fullResult.url"/>
      <vue-json-pretty :data="result" showLineNumber showIcon class="mt-3" />
    </div>
  </div>
</template>