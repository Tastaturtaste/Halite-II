import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import { useAuthStore } from './stores/auth'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('./pages/Home.vue'),
  },
  {
    path: '/learn',
    name: 'learn',
    component: () => import('./pages/Learn.vue'),
  },
  {
    path: '/learn/starter-kits',
    name: 'starter-kits',
    component: () => import('./pages/StarterKits.vue'),
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('./pages/Login.vue'),
  },
  {
    path: '/play',
    name: 'play',
    component: () => import('./pages/Play.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/my-bots',
    name: 'my-bots',
    component: () => import('./pages/MyBots.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/visualize',
    name: 'visualize',
    component: () => import('./pages/Visualizer.vue'),
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: () => import('./pages/NotFound.vue'),
  },
]

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
  scrollBehavior() {
    return { top: 0 }
  },
})

router.beforeEach(async (to) => {
  if (!to.meta.requiresAuth) {
    return true
  }

  const auth = useAuthStore()
  // main.ts starts this at boot; awaiting it here prevents a redirect before
  // the session cookie has been checked.
  await auth.hydrateOnce()

  if (!auth.isLoggedIn) {
    return {
      name: 'login',
      query: { redirect: to.fullPath },
    }
  }

  return true
})

export default router
