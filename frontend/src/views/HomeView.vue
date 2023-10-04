<script lang="ts" setup>
import { ref } from 'vue';

const link = ref('')

</script>

<template>
  <main>
    <section id="hero" class="py-48 flex items-center justify-center bg-white">
      <div class="container">
        <div class="hero-card  py-10 p-16 rounded-xl">
          <div class="hero-card-content max-w-[600px] mx-auto">
            <div class="hero-left pr-5 text-center mb-10">
              <h1 class="title text-6xl mb-5 leading-tight font-bold">Create and Manage your Links</h1>
              <p>Shortening your link can make the link is easier to be typed and read</p>
            </div>
            <div class="hero-right flex items-center w-full flex-grow">
              <form action="" method="post" class="w-full">
                <label class="bg-gray-100/50 p-2 rounded-t-lg" for="link">Shorten your link</label>
                <input type="link" id="link" class="rounded-md border-2 border-gray-300 bg-gray-100/90 w-full p-3" placeholder="http://">
              </form>
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