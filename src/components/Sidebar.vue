<script lang="ts" setup>
import { computed, onMounted, ref } from "@vue/runtime-core";
import { storeToRefs } from "pinia";
import { useApiRunnerStore } from "../stores/apiRunner";

const store = useApiRunnerStore();
const { fullData } = storeToRefs(store);

let currentEditedMenu = ref(null);
let requestName: any = ref(null);
let sidebarValue: any = ref({
  index: null,
  oldValue: "",
});

onMounted(async () => {
  await store.readFiles();
});

// let getFullData = computed(() => store.fullData ?? []);

function triggerEdit(key: any, index: any, oldValue: String) {
  currentEditedMenu.value = key;
  sidebarValue.index = index;
  sidebarValue.oldValue = oldValue;

  requestName.value[index].focus();
  document.addEventListener("click", disableEdit);
}

function disableEdit(event: Event): event is CustomEvent {
  let target = event.target as HTMLInputElement;
  if (!target.isSameNode(requestName.value[sidebarValue.index])) {
    currentEditedMenu.value = null;
    store.fullData[sidebarValue.index].name = sidebarValue.oldValue;
    document.removeEventListener("click", disableEdit);
  }

  return "detail" in event;
}

async function submitName() {
  currentEditedMenu.value = null;
  await store.saveJsonFile();
  document.removeEventListener("click", disableEdit);
}
</script>

<template>
  <div class="w-60 h-screen shadow-md bg-white z-10 fixed">
    <router-link
      class="
        flex
        items-center
        text-sm
        py-2
        px-6
        h-auto
        overflow-hidden
        text-gray-700 text-ellipsis
        whitespace-nowrap
        hover:text-gray-900 hover:bg-gray-100
        transition
        duration-300
        ease-in-out
        gap-3
      "
      :to="{ name: 'Home' }"
      data-mdb-ripple="true"
      data-mdb-ripple-color="dark"
      ><span class="uppercase font-medium">home</span></router-link
    >
    <ul class="relative">
      <li class="relative">
        <span
          class="
            flex
            items-center
            text-sm
            py-2
            px-6
            h-auto
            overflow-hidden
            text-gray-700 text-ellipsis
            whitespace-nowrap
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
          ><span class="uppercase text-green-500 font-medium">+</span>Add
          Request</span
        >
      </li>
      <li
        class="relative"
        v-for="(item, index) in fullData"
        :key="item.key"
        @dblclick="triggerEdit(item.key, index, item.name)"
      >
        <router-link
          class="
            flex
            items-center
            text-sm
            py-2
            px-6
            h-auto
            overflow-hidden
            text-gray-700 text-ellipsis
            whitespace-nowrap
            hover:text-gray-900 hover:bg-gray-100
            transition
            duration-300
            ease-in-out
            gap-3
          "
          :to="{ name: 'Request', params: { id: item.key } }"
          data-mdb-ripple="true"
          data-mdb-ripple-color="dark"
        >
          <span class="uppercase text-amber-400 font-medium">{{
            item.method
          }}</span>
          <span :class="{ hidden: item.key == currentEditedMenu }">{{
            item.name
          }}</span>
          <form
            :class="{
              show: item.key == currentEditedMenu,
              hidden: item.key != currentEditedMenu,
            }"
            @submit.prevent="submitName()"
          >
            <input
              type="text"
              v-model="item.name"
              ref="requestName"
              class="
                bg-white
                p-2
                rounded
                border-slate-300
                focus:border-sky-500
                focus:ring-1
                focus:ring-sky-500
                focus:outline-none
                w-full
              "
            />
          </form>
        </router-link>
      </li>
    </ul>
  </div>
</template>

<style scoped>
</style>