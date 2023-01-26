<script lang="ts" setup>
</script>

<template>
  <div class="w-60 h-screen shadow-md bg-white px-1 z-10 fixed">
    <ul class="relative">
      <li class="relative">
        <router-link
          class="
            flex
            items-center
            text-sm
            py-4
            px-6
            h-12
            overflow-hidden
            text-gray-700 text-ellipsis
            whitespace-nowrap
            rounded
            hover:text-gray-900 hover:bg-gray-100
            transition
            duration-300
            ease-in-out
          "
          :to="{name: 'Home'}"
          data-mdb-ripple="true"
          data-mdb-ripple-color="dark"
          >Dummy Link 1</router-link
        >
      </li>
      <li class="relative">
        <router-link
          class="
            flex
            items-center
            text-sm
            py-4
            px-6
            h-12
            overflow-hidden
            text-gray-700 text-ellipsis
            whitespace-nowrap
            rounded
            hover:text-gray-900 hover:bg-gray-100
            transition
            duration-300
            ease-in-out
          "
          :to="{name: 'Home'}"
          data-mdb-ripple="true"
          data-mdb-ripple-color="dark"
          >Dummy Link 2</router-link
        >
      </li> 
    </ul>
  </div>
</template>

<style scoped>
</style>