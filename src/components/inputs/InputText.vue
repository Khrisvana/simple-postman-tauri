<script lang="ts" setup> 
import { ref } from 'vue';

const emit = defineEmits([ 'update:modelValue' ]);
const props = defineProps({
    modelValue: {
        type: [ String, Number ],
        default: null,
    },
    name: {
        type: String,
        default: ''
    }
});

</script>

<template>
    <input
      type="text"
      class="
        px-3
        py-2
        block
        rounded
        bg-white
        border-slate-500
        w-full
        focus:border-sky-500 focus:outline-none focus:ring-1 focus:ring-sky-500
        text-sm
        shadow-sm
      "
      :name="props.name"
      :value="props.modelValue"
      @input="$emit('update:modelValue')"
    />
</template>