<script lang="ts" setup>
</script>

<template>
  <li
    class="relative"
    v-for="(item, index) in fullData"
    :key="item.key"
    @dblclick="triggerEdit(item.key, index, item.name)"
  >
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
      :to="{ name: 'Request', params: { id: item.key } }"
      data-mdb-ripple="true"
      data-mdb-ripple-color="dark"
    >
      <span class="uppercase text-amber-400 font-medium">{{
        item.method
      }}</span>
      <span :class="{ hidden: item.key == currentEditedMenu }">{{
        item.name
      }}</span>
      <form
        :class="{
          show: item.key == currentEditedMenu,
          hidden: item.key != currentEditedMenu,
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
  </li>
</template>