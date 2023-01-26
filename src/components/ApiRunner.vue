<script setup lang="ts">
import { getClient } from "@tauri-apps/api/http";
import { ref } from "vue";

const props = defineProps({
  modelValue: {
    type: [String],
    default: ''
  }
});
const emit = defineEmits(["result", "update:modelValue"]);

let method = ref("get");

async function runUrl() {
  console.log(props.modelValue);
  let response = null;
  const client = await getClient();
  if (method.value === "get") {
    response = await client.get(props.modelValue);
  } else if (method.value === "post") {
    response = await client.get(props.modelValue);
  }

  emit("result", response?.data);
}
</script>


<template>
  <form class="flex gap-1 sticky top-2 bg-white z-10" @submit.prevent="runUrl()">
    <select
      name=""
      id=""
      class="
        px-3
        py-2
        block
        rounded
        bg-white
        border-2 border-slate-300
        focus:border-sky-500 focus:outline-none focus:ring-1 focus:ring-sky-500
        text-sm
        shadow-sm
      "
      v-model="method"
    >
      <option value="get">GET</option>
      <option value="method">POST</option>
    </select>
    <input
      type="text"
      class="
        px-3
        py-2
        block
        rounded
        bg-white
        border-slate-300
        w-full
        focus:border-sky-500 focus:outline-none focus:ring-1 focus:ring-sky-500
        text-sm
        shadow-sm
      "
      :value="modelValue"
      @input="$emit('update:modelValue', $event.target.value)"
    />
    <button
      class="
        rounded
        bg-sky-500
        text-sm text-white
        px-3
        py-2
        hover:border-sky-700 hover:bg-sky-700
      "
      type="submit"
    >
      Run
    </button>
  </form>
</template>