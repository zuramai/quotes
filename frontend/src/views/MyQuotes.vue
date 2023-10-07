<script lang="ts" setup>
import Alert from '@/components/Alert.vue';
import MyButton from '@/components/MyButton.vue';
import Input from '@/components/MyInput.vue';
import { useStore } from '@/stores';
import axios from 'axios';
import { ref } from 'vue';
import { useRouter } from 'vue-router';


const quotes = ref([])
const isFetching = ref(true)

axios.get('/quotes')
    .then(res => {
        console.log(res)
        isFetching.value = false 
    }).catch(err => {
        isFetching.value = false 
    })
</script>
<template>
    <main>
        <div class="container mx-auto">
            <div class="max-w-[700px] mx-auto">
                <div class="flex justify-between  mb-8 items-center">
                    <h1 class="text-4xl font-bold">My Quotes</h1>
                    <MyButton to="/my-quotes/create">+ Create</MyButton>
                </div>
            </div>
            <p v-if="isFetching">Loading..</p>
            <p v-else-if="!quotes.length">No quote exists. You can <router-link to="/my-quotes/create">add quote</router-link>.</p>
        </div>
    </main>
</template>