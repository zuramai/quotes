<script lang="ts" setup>
import CardQuote from '@/components/CardQuote.vue';
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

const quotes = ref([])

const fetchQuotes = () => {
  axios.get('/quotes')
    .then(res => {
      console.log(res.data.data)
      quotes.value = res.data.data
    }).catch((err) => {
      console.log('error fetching data')
    })
}

onMounted(() => {
  fetchQuotes()
})

</script>

<template>
  <main>
    <section id="hero" class="flex items-center justify-center bg-white">
      <div class="container px-5">
        <p v-if="!quotes.length">No quote exists.</p>
        <div v-else class="cards grid grid-cols-3 gap-5">
          <CardQuote :quote="quote" v-for="quote in quotes" ></CardQuote>
        </div>
      </div>
    </section>
  </main>
</template>
<style>
</style>