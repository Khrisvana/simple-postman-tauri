<script lang="ts" setup>
import { onMounted } from "@vue/runtime-core"
import { storeToRefs } from "pinia";
import Sortable from "sortablejs";
import draggable from "vuedraggable";
import { useSidebarStore } from "../../stores/sidebar";
import SidebarItem from "./SidebarItem.vue";
import SidebarNoitem from "./SidebarNoitem.vue";

const emit = defineEmits(["update:modelValue"])
const store = useSidebarStore();
const { records } = storeToRefs(store);

let props = defineProps({
    modelValue: {
        default: null,
        type: Array,
    }, 
    group: {
        default: 'name',
        type: String
    }
})

function test() {
    console.log(records.value)
}

onMounted(async () => {
    await console.log(props.modelValue);
})
</script>

<template>
    <draggable
        tag="ul"
        class="pl-3 border-l"
        style="min-height: 1px;"
        :list="props.modelValue"
        item-key="name"
        :group="group"
        @change="test"
        :emptyInsertThreshold="70"
    >
        <template #item="{ element }">
            <li>
                <details :open="true">
                    <summary>
                        <SidebarItem :item="element" />
                    </summary>
                    <nested-draggable v-if="element.items && element.items.length >= 0" :list="element.items" :group="group"/>
                    <nested-draggable v-if="element.request && element.request.length >= 0" :list="element.request" group="request"/>
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