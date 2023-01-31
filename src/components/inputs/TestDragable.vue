<script lang="ts" setup>
import { ref } from "vue";

let dragItem: any = null;
let menuList: Array<String> = ["item 1", "item 2", "item 3", "item 4"];

function dragStart(event: any) {
  dragItem = event.target;
  setTimeout(() => (dragItem.className = "invisible"), 0);
  return true;
}
function dragEnd(event: any) {
  let target = event.target;
  target.className = "item";
  dragItem = null;
  return true;
}
function dragEnter(event: any) {
  event.preventDefault();
  return true;
}
function dragLeave(event: any) {
  event.preventDefault();
  return true;
}
function dragOver(event: any) {
  event.preventDefault();
  return true;
}
function dragDrop(event: any) {
  let target = event.target;
  // if (target.classList.contains("dragable-container")) {
  dragItem.append(target);
  return true;
  // }
  // return false;
}
</script>

<template>
  <div
    class="dragable-container flex flex-col h-80"
    @dragenter.prevent
    @dragover.prevent
    @drop.prevent="dragDrop"
    @dragleave="dragLeave"
  >
    <div
      v-for="menu in menuList"
      :key="menu"
      class="item"
      draggable="true"
      @dragstart="dragStart"
      @dragend="dragEnd"
      @dragenter.prevent
      @dragover.prevent
      @dragleave="dragLeave"
    >
      {{ menu }}
    </div>
  </div>
</template>