<script lang="ts" setup>
import Alert from '@/components/Alert.vue';
import CardQuote from '@/components/CardQuote.vue';
import MyButton from '@/components/MyButton.vue';
import Input from '@/components/MyInput.vue';
import { useStore } from '@/stores';
import type { Quote } from '@/types';
import axios from 'axios';
import { ref } from 'vue';
import { useRouter } from 'vue-router';


const quotes = ref<Quote[]>([])
const isFetching = ref(true)


axios.get('/my-quotes')
    .then(res => {
        console.log(res.data.data)
        quotes.value.push(...res.data.data)
        isFetching.value = false 
    }).catch(err => {
        isFetching.value = false 
    })
</script>
<template>
    <main>
        <div class="container mx-auto">
            <div class="flex justify-between  mb-8 items-center">
                <h1 class="text-4xl font-bold">My Quotes</h1>
                <MyButton to="/my-quotes/create">+ Create</MyButton>
            </div>
            <p v-if="isFetching">Loading..</p>
            <p v-else-if="!quotes.length">No quote exists. You can <router-link to="/my-quotes/create">add quote</router-link>.</p>
            
            <div v-else class="cards grid grid-cols-1 lg:grid-cols-3 gap-5">
                <CardQuote :quote="quote" v-for="quote in quotes" ></CardQuote>
            </div>
        </div>
    </main>
</template>