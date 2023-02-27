<script lang="ts" setup>
import { onMounted, ref } from "@vue/runtime-core"
import { storeToRefs } from "pinia"
import Sortable from "sortablejs"
import draggable from "vuedraggable"
import { useSidebarStore } from "../../stores/sidebar"
import SidebarItem from "./SidebarItem.vue"
import SidebarNoitem from "./SidebarNoitem.vue"
import { invoke } from "@tauri-apps/api/tauri"
import ContextMenu from "../ContextMenu.vue"

const emit = defineEmits(["update:modelValue"])
const store = useSidebarStore()
const { records } = storeToRefs(store)

let props = defineProps({
    modelValue: {
        default: null,
        type: Array,
    },
    group: {
        default: "name",
        type: String,
    },
    parent: {
        default: null,
        type: Number,
    },
})

async function updateOrder(e: any, parent: any) {
    console.log(e)

    let result = {}

    if (e.added) {
        let added = e.added
        result = {
            parent: parent,
            targetId: added.element.id,
            index: added.newIndex,
        }
    } else if (e.moved) {
        let moved = e.moved
        result = {
            parent: parent,
            targetId: moved.element.id,
            index: moved.newIndex,
        }
    } else {
        return //WIP: reorder removed item
    }

    await invoke("update_order", result)
}

let sidebarItemContextMenu = ref()
const folderMenuItem = [
    {
        name: "New Folder",
    },
    {
        name: "New Request",
    },
    {
        name: "Edit",
    },
    {
        name: "Delete",
    },
]

const requestMenuItem = [
    {
        name: "Edit",
    },
    {
        name: "Delete",
    },
]

function triggerContextMenu(event: Event) {
    sidebarItemContextMenu.value.showMenu(event)
}

onMounted(async () => {
    await console.log(props.modelValue)
})
</script>

<template>
    <draggable
        tag="ul"
        class="pl-3 pb-1 border-l"
        style="min-height: 1px"
        :list="props.modelValue"
        item-key="name"
        :group="props.group"
        @change="updateOrder($event, props.parent)"
        :emptyInsertThreshold="20"
    >
        <template #item="{ element }">
            <li>
                <details open>
                    <summary>
                        <SidebarItem
                            :item="element"
                            @contextmenu.prevent="triggerContextMenu"
                        />
                    </summary>
                    <nested-draggable
                        v-if="element.items && !element.method"
                        :list="element.items"
                        :group="props.group"
                        :parent="element.id"
                    />
                    <ContextMenu
                        ref="sidebarItemContextMenu"
                        :items="element.method ? requestMenuItem : folderMenuItem"
                    />
                </details>
            </li>
        </template>
    </draggable>
</template>

<style>
summary::marker {
    display: none !important;
    content: "";
}
details[open] > summary span .collapse-indicator {
    transform: rotate(90deg);
}
</style>