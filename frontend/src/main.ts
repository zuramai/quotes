import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import './assets/main.css'

import App from './App.vue'
import router from './router'

import axios from 'axios'

const app = createApp(App)

axios.defaults.baseURL = "http://localhost:8000/api"

app.use(createPinia())
app.use(router)

app.mount('#app')
