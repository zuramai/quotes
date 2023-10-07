import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import type { User } from '@/types'


export const useStore = defineStore('main', () => {
  const user = ref<null|User>(null)
  const token = ref("")

  return {
    user,
    token 
  }
})
