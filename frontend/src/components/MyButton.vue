<script setup lang="ts">
const props = defineProps<{
    to?: string
}>()
const classes = 'btn py-2 px-5 bg-gray-800 text-white'
</script>
<template>
    <router-link :to="to" v-if="to" :class="[classes, 'no-underline']"><slot/></router-link>
    <button :class="[classes]" v-else><slot/></button>
</template>