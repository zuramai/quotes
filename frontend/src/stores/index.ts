import { ref, computed, onMounted } from 'vue'
import { defineStore } from 'pinia'
import type { User } from '@/types'
import { useRouter } from 'vue-router'
import axios from 'axios'


export const useStore = defineStore('main', () => {
  const user = ref<null|User>(null)
  const token = ref("")
  const setToken = (t: string) => {
    localStorage.setItem('token', t)
    token.value = t
    axios.defaults. headers.common.Authorization = "Bearer "+t
  }
  onMounted(() => {
    const localToken = localStorage.getItem('token')
    if(localToken) {
      token.value = localToken
      axios.defaults. headers.common.Authorization = "Bearer "+localToken
    }
  })
  const isLoggedin = computed(() => token.value !== "")
  const logout = () => {
    localStorage.removeItem('token')
    token.value = ""
  }
  return {
    logout,
    user,
    token,
    setToken,
    isLoggedin
  }
})
