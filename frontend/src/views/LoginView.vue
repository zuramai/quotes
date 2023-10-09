<script lang="ts" setup>
import Alert from '@/components/Alert.vue';
import MyButton from '@/components/MyButton.vue';
import Input from '@/components/MyInput.vue';
import { useStore } from '@/stores';
import axios from 'axios';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const username = ref('')
const password = ref('')
const error = ref(false)

const store = useStore()
const router = useRouter()

const login = () => {
    axios.post('/login', {
        username: username.value,
        password: password.value
    }).then((res) => {
        store.setToken(res.data.data.token)
        router.push('/')
    }).catch(err => {
        error.value = err.response.data.message
    });
}
</script>
<template>
    <main>
        <div class="container mx-auto">
            <div class="max-w-[700px] mx-auto">
                <h1 class="text-4xl font-bold mb-8">Sign In</h1>
                <Alert type="danger" class="mb-5" v-if="error">{{ error }}</Alert>
                <form action="" @submit.prevent="login">
                    <div class="input-group mb-3">
                        <Input label="Username" name="username" v-model="username"/>
                    </div>
                    <div class="input-group mb-3">
                        <Input type="password" label="Password" name="password" v-model="password"/>
                    </div>
                    <div class="flex justify-end items-center gap-3">
                        <p class="text-sm">Don't have an account? <router-link to="/register">Sign up</router-link></p>
                        <MyButton>Submit</MyButton>
                    </div>
                </form>
            </div>
        </div>
    </main>
</template>