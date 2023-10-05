<script setup lang="ts">
const props = defineProps({
    
})
</script>
<template>
    <div class="card card-quote border border-gray-400 p-5">
        <h3 class="quote text-3xl mb-5 font-bold">The greatest use of a life is to spend it on something that will outlast it.</h3>
        <div class="flex justify-between">
            <p class="quote-author text-xl">William James</p>
            <div class="like">
                <button>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 11v8a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-7a1 1 0 0 1 1-1h3a4 4 0 0 0 4-4V6a2 2 0 0 1 4 0v5h3a2 2 0 0 1 2 2l-1 5a2 3 0 0 1-2 2h-7a3 3 0 0 1-3-3"/></svg>
                </button>
            </div>
        </div>
    </div>
</template>