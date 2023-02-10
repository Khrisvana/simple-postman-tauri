<script lang="ts" setup>
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
                <details>
                    <summary>
                        <SidebarItem :item="element" />
                    </summary>
                    <template v-if="element.request.length > 0">
                        <SidebarItem
                            v-for="request in element.request"
                            :key="request.id"
                            :item="request"
                        />
                    </template>
                    <template v-else>
                        <SidebarNoitem/>
                    </template>
                    <nested-draggable :list="element.items" />
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