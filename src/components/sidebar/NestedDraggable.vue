<script lang="ts" setup>
import { onMounted, ref } from "@vue/runtime-core"
import { storeToRefs } from "pinia"
import Sortable from "sortablejs"
import draggable from "vuedraggable"
import { useSidebarStore } from "../../stores/sidebar"
import SidebarItem from "./SidebarItem.vue"
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
let showChild = ref(true)
let menuItems = ref()
let activeElement = ref()

function contextMenuItem(show: Boolean) : Array<Object> {
    let menuItems: Array<Object> = [
        { title: "Edit", class: "text-black", method: "editItem" },
        { title: "Delete", class: "text-red-500", method: "deleteItem" },
    ]

    if (show) {
        menuItems.unshift(
            { title: "New Folder", class: "text-black", method: "newFolder" },
            { title: "New Request", class: "text-black", method: "newRequest" }
        )
    }

    return menuItems
}

async function updateOrder(e: any, parent: any) {
    document.dispatchEvent(new Event("click"))

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

    try {
        await invoke("update_order", result)
    } catch (error) {
        console.log(error)
    }
}

async function addNewItems(
    parent: Number | null,
    order: Number,
    isFolder: Boolean
) {
    document.dispatchEvent(new Event("click"))

    try {
        let payload = {
            parent: parent,
            order: order,
            isFolder: isFolder,
        }

        await invoke("store_request", payload)
        await store.getRecords()
    } catch (error) {
        console.log(error)
    }
}

let sidebarItemContextMenu = ref()

function triggerContextMenu(event: Event, status: string|null, element: Object) {
    event.preventDefault()
    
    sidebarItemContextMenu.value.showMenu(event)
    menuItems.value = contextMenuItem(!status)
    activeElement.value = element
}

function menuHandler(method: string, item: Object) {
    console.log(method, item)
}

onMounted(async () => {})
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
                <SidebarItem :item="element" @contextmenu.stop="triggerContextMenu($event, element.method, element)" @click.prevent="showChild = !showChild" :class="{'open': showChild}"/>
                <nested-draggable
                    v-if="!element.method"
                    :class="{'show': showChild, 'hidden': !showChild}"
                    :list="element.items"
                    :group="props.group"
                    :parent="element.id"
                />
                <ContextMenu ref="sidebarItemContextMenu">
                    <div
                        class="w-full px-3 hover:bg-gray-100 text-sm py-1 cursor-pointer font-thin"
                        :class="menu.class"
                        v-for="(menu, index) in menuItems"
                        :key="index"
                        @click="menuHandler(menu.method, activeElement)"
                    >
                        {{ menu.title }}
                    </div>
                </ContextMenu>
            </li>
        </template>
    </draggable>
</template>

<style>
span.open .collapse-indicator {
    transform: rotate(90deg);
}
</style>