<script lang="ts" setup>
import { onMounted } from "@vue/runtime-core"
import draggable from "vuedraggable"
import SidebarItem from "./SidebarItem.vue"
import SidebarNoitem from "./SidebarNoitem.vue"

const emit = defineEmits(["update:modelValue"])

let props = defineProps({
    modelValue: {
        default: null,
        type: Array,
    },
})

onMounted(async () => {
    await console.log(props.modelValue);
})
</script>

<template>
    <draggable
        tag="ul"
        class="pl-3"
        :list="props.modelValue"
        item-key="name"
        group="name"
    >
        <template #item="{ element }">
            <li>
                <details :open="true">
                    <summary>
                        <SidebarItem :item="element" />
                    </summary>
                    <nested-draggable v-if="element.request && element.request.length > 0" :list="element.request" />
                    <!-- <template v-if="element.request && element.request.length == 0">
                        <SidebarNoitem/>
                    </template> -->
                    <nested-draggable v-if="element.items && element.items.length >= 0" :list="element.items" />
                </details>
            </li>
        </template>
    </draggable>
</template>

<style>
summary::marker {
    display: none!important;
    content: "";
}
details[open] > summary span .collapse-indicator {
  transform: rotate(90deg);
}
</style>