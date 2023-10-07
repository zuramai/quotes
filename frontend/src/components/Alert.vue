<script setup lang="ts">
interface Props {
    type: "success" | "danger",
}
const props = defineProps<Props>()

const alertClasses: Record<Props['type'], string> = {
    success: 'bg-green-300 text-green-600',
    danger: 'bg-rose-100 text-red-600 border border-rose-200',
}
</script>
<template>
    <div :class="['alert p-3  bg-gray-500', alertClasses[type]]">

        <slot></slot>
    </div>
</template>