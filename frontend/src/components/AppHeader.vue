<template>
    <header class="absolute w-full">
        <div class="container mx-auto flex justify-between p-5 items-center">
            <div class="header-logo">
                <router-link to="/" class="font-bold text-xl">BukaLink</router-link>
            </div>
            <div class="header-menu">
                <ul class="flex gap-5">
                    <li>
                        <router-link to="/">Home</router-link>
                    </li>
                    <li>
                        <router-link to="/">My Links</router-link>
                    </li>
                    <li>
                        <router-link to="/">Sign In</router-link>
                    </li>
                </ul>
            </div>
        </div>
    </header>
</template>