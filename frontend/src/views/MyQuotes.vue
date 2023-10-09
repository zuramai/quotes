<script lang="ts" setup>
import Alert from '@/components/Alert.vue';
import CardQuote from '@/components/CardQuote.vue';
import MyButton from '@/components/MyButton.vue';
import { useStore } from '@/stores';
import type { Quote } from '@/types';
import axios from 'axios';
import { onMounted, reactive, ref } from 'vue';


const quotes = ref<Quote[]>([])
const isFetching = ref(true)
const store = useStore()

const pagination = reactive({
  page: 0,
  size: 15
})


onMounted(() => {
    fetchQuotes()

    window.addEventListener('scroll', e => {
        if(window.scrollY + window.innerHeight + 2 > document.body.clientHeight) {
        fetchQuotes()
        }
    })
})
const fetchQuotes = () => {
    axios.get('/my-quotes', {params: pagination})
    .then(res => {
        console.log(res.data.data)
        quotes.value.push(...res.data.data)
        isFetching.value = false 
        pagination.page++
    }).catch(err => {
        isFetching.value = false 
    })
}
</script>
<template>
    <main class="pb-24">
        <div class="container mx-auto">
            <div class="flex justify-between  mb-8 items-center" >
                <h1 class="text-4xl font-bold">My Quotes</h1>
                <MyButton to="/my-quotes/create" v-if="store.isLoggedin">+ Create</MyButton>
            </div>
            <p v-if="!store.isLoggedin">You are not logged in. Please <router-link to="/login">sign in</router-link> to continue.</p>
            <p v-else-if="isFetching">Loading..</p>
            <p v-else-if="!quotes.length">No quote exists. You can <router-link to="/my-quotes/create">add quote</router-link>.</p>
            
            <div v-else class="cards grid grid-cols-1 lg:grid-cols-3 gap-5">
                <CardQuote :quote="quote" v-for="quote in quotes" ></CardQuote>
            </div>
        </div>
    </main>
</template>