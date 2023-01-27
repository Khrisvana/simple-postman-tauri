<script lang="ts" setup>
import { ref } from "vue";

let dragItem: any = null;

function dragStart(event: any) {
  console.log("drag started");
  dragItem = event.target;
  setTimeout(() => (dragItem.className = "invisible"), 0);
  return true;
}
function dragEnd(event: any) {
  console.log('test');
  let target = event.target;
  target.className = "item";
  dragItem = null;
  return true;
}
function dragEnter(event: any) {
  console.log("drag entered");
  event.preventDefault();
  return true;
}
function dragLeave(event: any) {
  event.preventDefault();

  console.log("drag left");
  return true;
}
function dragOver(event: any) {
  event.preventDefault();
  console.log("drag over");
  return true;
}
function dragDrop(event: any) {
  console.log("drag dropped");
  console.log(dragItem);
  let target = event.target;
  console.log(target);
  target.append(dragItem);
  return true;
}
</script>

<template>
  <div class="flex flex-col h-80"
      @dragenter.prevent
      @dragover.prevent
      @drop.prevent="dragDrop"
      @dragleave="dragLeave"
  >
    <div
      class="item"
      draggable="true"
      @dragstart="dragStart"
      @dragend="dragEnd"
      @dragenter.prevent
      @dragover.prevent
      @dragleave="dragLeave"
    >
      test 1
    </div>
    <div
      class="item"
      draggable="true"
      @dragstart="dragStart"
      @dragend="dragEnd"
      @dragenter.prevent
      @dragover.prevent
      @dragleave="dragLeave"
    >
      test 2
    </div>
    <div
      class="item"
      draggable="true"
      @dragstart="dragStart"
      @dragend="dragEnd"
      @dragenter.prevent
      @dragover.prevent
      @dragleave="dragLeave"
    >
      test 3
    </div>
    <div
      class="item"
      draggable="true"
      @dragstart="dragStart"
      @dragend="dragEnd"
      @dragenter.prevent
      @dragover.prevent
      @dragleave="dragLeave"
    >
      test 4
    </div>
  </div>
</template>