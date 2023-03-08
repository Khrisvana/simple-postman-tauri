<script lang="ts" setup>
import { computed, onMounted, ref } from "@vue/runtime-core";
import { storeToRefs } from "pinia";
import { useSidebarStore } from "../stores/sidebar";
import NestedDraggable from "../components/sidebar/NestedDraggable.vue";

const store = useSidebarStore();
const { records } = storeToRefs(store);

let currentEditedMenu = ref(null);
let requestName: any = ref(null);
let sidebarValue: any = ref({
  index: null,
  oldValue: "",
});

onMounted(async () => {
  await store.getRecords();
  console.log(records.value);
  
});

function changeOrder() {
  console.log(records);
    
}

// let getFullData = computed(() => store.fullData ?? []);

// function triggerEdit(key: any, index: any, oldValue: String) {
//   currentEditedMenu.value = key;
//   sidebarValue.index = index;
//   sidebarValue.oldValue = oldValue;

//   requestName.value[index].focus();
//   document.addEventListener("click", disableEdit);
// }

// function disableEdit(event: Event): event is CustomEvent {
//   let target = event.target as HTMLInputElement;
//   if (!target.isSameNode(requestName.value[sidebarValue.index])) {
//     currentEditedMenu.value = null;
//     store.fullData[sidebarValue.index].name = sidebarValue.oldValue;
//     document.removeEventListener("click", disableEdit);
//   }

//   return "detail" in event;
// }

// async function submitName() {
//   currentEditedMenu.value = null;
//   await store.saveJsonFile();
//   document.removeEventListener("click", disableEdit);
// }
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
      <NestedDraggable class="w-full" v-model="records" group="folder"/>
    </ul>
  </div>
</template>

<style scoped>
</style>