<script lang="ts" setup>
import CardQuote from '@/components/CardQuote.vue';
import type { Quote } from '@/types';
import axios, { AxiosError } from 'axios';
import { onMounted, reactive, ref } from 'vue';

const link = ref('')
const loading = ref(false)
const result = ref({
  link: "http://localhost/xByX3",
  shortened: "http://google.com"
})

const shorten = () => {
  loading.value = true 
  setTimeout(() => loading.value = false, 500)
}

const copytext = ref('copy')
const copyLink = async () => {
  copytext.value = "Copied!"
  await navigator.clipboard.writeText(result.value.link)
  setTimeout(() => {
    copytext.value = "Copy"
  }, 300);
}

const quotes = ref<Quote[]>([])

const pagination = reactive({
  page: 0,
  size: 15
})

const isLoading = ref(true)

const fetchQuotes = () => {
  axios.get('/quotes', {params: pagination})
    .then(res => {
      quotes.value.push(...res.data.data)
      isLoading.value = false
      pagination.page++
    }).catch((err) => {
      console.log('error fetching data')
      isLoading.value = false
    })
}

onMounted(() => {
  fetchQuotes()

  // Infinite scroll

  window.addEventListener('scroll', e => {
    if(window.scrollY + window.innerHeight + 2 > document.body.clientHeight) {
      fetchQuotes()
    }
  })
})

</script>

<template>
  <main class="pb-24">
    <section id="hero" class="flex items-center justify-center bg-white">
      <div class="container px-5">
        <p v-if="!quotes.length && !isLoading">No quote exists.</p>
        <p v-else-if="isLoading">Loading</p>
        <div v-else class="cards grid grid-cols-1 lg:grid-cols-3 gap-5">
          <CardQuote :quote="quote" v-for="quote in quotes" ></CardQuote>
        </div>
      </div>
    </section>
  </main>
</template>
<style>
</style>