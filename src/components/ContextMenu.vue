<script lang="ts" setup>
import { onBeforeUnmount, onMounted, ref } from "vue"

const props = defineProps({
    items: {
        type: Array,
        default: [],
    },
})

let me = ref()
let display = ref({
    name: "hidden",
    cursorX: 0,
    cursorY: 0,
})

function showMenu(event: MouseEvent, displayName: string) {
    document.dispatchEvent(new Event('click'));

    display.value = {
        name: "block",
        cursorX: event.clientX,
        cursorY: event.clientY,
    }
}

function closeMenu(event: Event) {
    if (!me.value.contains(event.target)) {
        display.value = {
            name: "hidden",
            cursorX: 0,
            cursorY: 0,
        }
    }
}

defineExpose({ showMenu })
onMounted(() => {
    document.addEventListener("click", closeMenu)
})

onBeforeUnmount(() => {
    document.removeEventListener("click", closeMenu)
})
</script>

<template>
    <div
        class="bg-white absolute p-3 rounded drop-shadow w-max"
        :class="display.name"
        :style="{ left: `${display.cursorX}px`, top: `${display.cursorY}px` }"
        ref="me"
    >
        <div v-for="(item, index) in items" :key="index">
            <!-- <i>[Ic]</i> -->
            {{ item.name }}
            <!-- <ContextMenu :items="item.child"> -->
        </div>
    </div>
</template>