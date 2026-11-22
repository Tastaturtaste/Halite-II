<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useAuthStore } from './stores/auth'
import logo from './assets/images/full_logo.svg'

const auth = useAuthStore()
const { email, loading } = storeToRefs(auth)

async function handleLogout() {
  await auth.logout()
}
</script>

<template>
  <header class="site-header">
    <div class="container nav-inner">
      <RouterLink to="/" class="navbar-brand" aria-label="Halite II home">
        <img :src="logo" alt="Halite II" />
      </RouterLink>

      <nav class="main-nav" aria-label="Main navigation">
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/learn">Learn</RouterLink>
        <RouterLink to="/learn/starter-kits">Starter Kits</RouterLink>
        <RouterLink to="/play">Play</RouterLink>
        <RouterLink to="/visualize">Visualize</RouterLink>
        <RouterLink to="/my-bots">My Bots</RouterLink>
      </nav>

      <div class="auth-area">
        <template v-if="email">
          <span class="auth-email" :title="email">{{ email }}</span>
          <button
            type="button"
            class="auth-logout"
            :disabled="loading"
            @click="handleLogout"
          >
            Logout
          </button>
        </template>
        <RouterLink v-else to="/login" class="auth-link">Login</RouterLink>
      </div>
    </div>
  </header>

  <main class="site-main">
    <RouterView />
  </main>

  <footer class="footer">
    <div class="container">
      <ul class="footer-nav">
        <li><RouterLink to="/">Home</RouterLink></li>
        <li><RouterLink to="/learn">Learn</RouterLink></li>
        <li><RouterLink to="/learn/starter-kits">Starter Kits</RouterLink></li>
        <li><RouterLink to="/play">Play</RouterLink></li>
        <li><RouterLink to="/visualize">Visualize</RouterLink></li>
        <li><RouterLink to="/my-bots">My Bots</RouterLink></li>
        <li><RouterLink to="/login">Login</RouterLink></li>
      </ul>

      <div class="footer-below">
        <div class="footer-copy">
          <img class="footer-copy-logo" :src="logo" alt="Halite II" />
          <p>Halite II — an open-source programming competition.</p>
        </div>
        <div class="footer-links">
          <RouterLink to="/learn">Documentation</RouterLink>
          <RouterLink to="/play">Submit a bot</RouterLink>
          <RouterLink to="/visualize">Watch games</RouterLink>
        </div>
      </div>
    </div>
  </footer>
</template>
