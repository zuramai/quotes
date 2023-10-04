<script lang="ts" setup>
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

onMounted(() => {

})

</script>

<template>
  <main>
    <section id="hero" class="py-48 flex items-center justify-center bg-white min-h-screen">
      <div class="container px-5">
        <div class="hero-card  py-10 p-16 rounded-xl">
          <div class="hero-card-content max-w-[600px] mx-auto">
            <div class="hero-left pr-5 text-center mb-10">
              <h1 class="title text-4xl lg:text-6xl mb-5 leading-tight font-bold">Create and Manage your Links</h1>
              <p>Shortening your link can make the link is easier to be typed and read</p>
            </div>
            <div class="hero-right  w-full flex-grow">
              <form action="" method="post" class="w-full" @submit.prevent="shorten" >
                <label class="bg-gray-100/50 p-2 rounded-t-lg" for="link">Shorten your link</label>
                <div class="input-group-btn relative  mb-5">
                  <input type="link" id="link" v-model="link" class="rounded-md border-2 focus:outline-none focus:border-blue-500 border-gray-300 bg-gray-100/90 w-full p-3" placeholder="http://">
                  <button class="btn btn-submit bg-sky-800 text-white absolute right-0 h-full p-3 rounded-r-md">
                    <svg v-if="loading" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><g stroke="currentColor"><circle cx="12" cy="12" r="9.5" fill="none" stroke-linecap="round" stroke-width="3"><animate attributeName="stroke-dasharray" calcMode="spline" dur="1.5s" keySplines="0.42,0,0.58,1;0.42,0,0.58,1;0.42,0,0.58,1" keyTimes="0;0.475;0.95;1" repeatCount="indefinite" values="0 150;42 150;42 150;42 150"/><animate attributeName="stroke-dashoffset" calcMode="spline" dur="1.5s" keySplines="0.42,0,0.58,1;0.42,0,0.58,1;0.42,0,0.58,1" keyTimes="0;0.475;0.95;1" repeatCount="indefinite" values="0;-16;-59;-59"/></circle><animateTransform attributeName="transform" dur="2s" repeatCount="indefinite" type="rotate" values="0 12 12;360 12 12"/></g></svg>
                    <p v-else>
                      Shorten
                    </p>
                  </button>
                </div>
              </form>
              <div class="result p-3 bg-white/30 rounded-lg" v-if="result">
                <p>Shortened link:</p>
                <div class="flex gap-3">
                  <h2 class="text-2xl font-bold"><a :href="result.link" target="_blank">{{ result.link }}</a></h2>
                  <button class="bg-blue-500 p-1 px-3 hover:bg-blue-600 text-white rounded-md text-sm" @click="copyLink">{{copytext}}</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</template>
<style>
#hero {
  background: #7F7FD5;  /* fallback for old browsers */
  background: linear-gradient(125deg, #d8e6fc, #86A8E7, #7F7FD5); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
}

.hero-card {
  background-color: rgba(255, 255, 255, 0.165);
  backdrop-filter: blur(1rem);

}

</style>