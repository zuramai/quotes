<script setup lang="ts">
const props = defineProps<{
    label?: string
    name: string
    modelValue: string
    type?: string
    placeholder?: string
}>()
const emit = defineEmits(["update:modelValue"])
</script>
<template>
    <label :for="name" v-if="label">{{ label }}</label>
    <input 
        :type="type ? type : 'text'" 
        :name="name" 
        :id="name" 
        class="border border-gray-400 focus:border-gray-500 focus:outline-none w-full px-2 py-1" 
        :value="modelValue" 
        :placeholder="placeholder"
        @input="e => emit('update:modelValue', (e.target! as HTMLInputElement).value)">
</template>