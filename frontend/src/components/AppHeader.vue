<script setup lang="ts">
import { useStore } from '@/stores';

const store = useStore()
console.log(store.isLoggedin)
</script>
<template>
    <header class=" w-full text-center py-3  border-b border-gray-400 mb-10">
        <h1 class="title text-4xl lg:text-6xl mb-5 leading-tight font-bold">Quotes</h1>

        <ul class="flex gap-5 justify-center">
            <li>
                <router-link to="/">Home</router-link>
            </li>
            <li>
                <router-link to="/my-quotes">My Quotes</router-link>
            </li>
            <template v-if="!store.isLoggedin">
                <li>
                    <router-link to="/login">Sign In</router-link>
                </li>
            </template>
            <template v-else>
                <li>
                    <router-link to="#" @click.prevent="store.logout()">Sign Out</router-link>
                </li>
            </template>
        </ul>
    </header>
</template>