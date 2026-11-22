import { defineStore } from 'pinia'
import * as api from '../api'

interface AuthState {
  email: string | null
  loading: boolean
}

// Shared so the startup hydration and the router guard await the same request.
let hydration: Promise<void> | null = null

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    email: null,
    loading: false,
  }),

  getters: {
    isLoggedIn: (state): boolean => state.email !== null,
  },

  actions: {
    async login(email: string) {
      this.loading = true
      try {
        const response = await api.login(email)
        this.email = response.email
      } finally {
        this.loading = false
      }
    },

    async logout() {
      this.loading = true
      try {
        await api.logout()
      } finally {
        this.email = null
        this.loading = false
      }
    },

    async fetchMe() {
      this.loading = true
      try {
        const response = await api.fetchMe()
        this.email = response.email
      } catch {
        // 401 simply means there is no active session.
        this.email = null
      } finally {
        this.loading = false
      }
    },

    /**
     * Runs the initial `/api/me` lookup at most once and lets the router guard
     * wait for it before deciding whether the visitor is authenticated.
     */
    hydrateOnce(): Promise<void> {
      if (!hydration) {
        hydration = this.fetchMe()
      }
      return hydration
    },
  },
})
