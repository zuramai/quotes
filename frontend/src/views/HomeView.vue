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

</script>

<template>
  <main>
    <section id="hero" class="flex items-center justify-center bg-white">
      <div class="container px-5">
      </div>
    </section>
  </main>
</template>
<style>
</style>