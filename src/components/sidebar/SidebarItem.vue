<script lang="ts" setup>
import { onMounted } from "@vue/runtime-core"

const props = defineProps({
    item: {
        type: Object,
        default: {},
    },
})

onMounted(async () => {})
</script>

<template>
    <span
        v-if="!item.method"
        class="
            folder
            flex
            items-center
            text-sm
            py-2
            pl-2
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
        :to="{ name: 'Request', params: { id: item.id } }"
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
    >
        <div class="collapse-indicator">
            <font-awesome-icon icon="fa-solid fa-angle-right" />
        </div>
        <font-awesome-icon icon="fa-regular fa-folder" />

        <span :class="{ hidden: item.id == currentEditedMenu }">{{
            item.name
        }}</span>
        <form
            :class="{
                show: item.id == currentEditedMenu,
                hidden: item.id != currentEditedMenu,
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
    </span>

    <router-link
        v-else
        class="
            request
            flex
            items-center
            text-sm
            py-2
            pl-2
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
        :to="{ name: 'Request', params: { id: item.id } }"
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
    >
        <span class="uppercase text-amber-400 font-medium">{{
            item.method
        }}</span>

        <span :class="{ hidden: item.id == currentEditedMenu }">{{
            item.name
        }}</span>
        <form
            :class="{
                show: item.id == currentEditedMenu,
                hidden: item.id != currentEditedMenu,
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
</template>