import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/LoginView.vue')
    },
    {
      path: '/my-quotes',
      name: 'myquotes',
      component: () => import('../views/MyQuotes.vue')
    },
    {
      path: '/search',
      name: 'search',
      component: () => import('../views/Search.vue')
    },
    {
      path: '/my-quotes/create',
      name: 'my-quotes',
      component: () => import('../views/CreateQuote.vue')
    },
  ]
})

export default router
