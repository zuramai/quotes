<script lang="ts" setup>
import axios from 'axios';
import { ref } from 'vue';

const username = ref('')
const password = ref('')

const login = () => {
    axios.post('/login', {
        username: username.value,
        password: password.value
    }).then((res) => {
        console.log(res.data)
    }) ;
}
</script>
<template>
    <main>
        <div class="container mx-auto">
            <div class="max-w-[700px] mx-auto">
                <h1 class="text-4xl font-bold mb-8">Sign In</h1>
                <form action="" @submit.prevent="login">
                    <div class="input-group mb-3">
                        <label for="username">Username</label>
                        <input type="text" name="username" class="border border-gray-400 focus:border-gray-500 focus:outline-none w-full px-2 py-1">
                    </div>
                    <div class="input-group mb-3">
                        <label for="password">Password</label>
                        <input type="password" name="password" class="border border-gray-400 focus:border-gray-500 focus:outline-none w-full px-2 py-1">
                    </div>
                    <div class="flex justify-end">
                        <button class="btn py-2 px-5 bg-gray-800 text-white">Submit</button>
                    </div>
                </form>
            </div>
        </div>
    </main>
</template>