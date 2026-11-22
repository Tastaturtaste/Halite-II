<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ApiError } from '../api'
import { useAuthStore } from '../stores/auth'

const auth = useAuthStore()
const route = useRoute()
const router = useRouter()

const email = ref('')
const error = ref('')
const submitting = ref(false)
const checking = ref(true)

const redirectTarget = computed(() => {
  const queryValue = route.query.redirect
  const value = Array.isArray(queryValue) ? queryValue[0] : queryValue
  // Only allow local paths, so a crafted ?redirect= cannot leave the site.
  if (typeof value === 'string' && value.startsWith('/') && !value.startsWith('//')) {
    return value
  }
  return '/play'
})

onMounted(async () => {
  await auth.hydrateOnce()
  checking.value = false
})

async function handleSubmit() {
  error.value = ''
  submitting.value = true

  try {
    await auth.login(email.value.trim())
    await router.replace(redirectTarget.value)
  } catch (err) {
    error.value =
      err instanceof ApiError ? err.message : 'Login failed. Please check your connection and try again.'
  } finally {
    submitting.value = false
  }
}

async function handleLogout() {
  await auth.logout()
}
</script>

<template>
  <div class="container login-page">
    <div class="login-card">
      <p v-if="checking" class="login-checking">Checking your session…</p>

      <template v-else-if="auth.email">
        <h1>You are logged in</h1>
        <p class="login-email">{{ auth.email }}</p>
        <p class="login-help">
          You can submit bots and see your stats. Log out to sign in with a different email.
        </p>
        <div class="login-actions">
          <RouterLink class="btn-ha" to="/play">Submit a bot</RouterLink>
          <button type="button" class="btn-ha btn-ha-clear" :disabled="auth.loading" @click="handleLogout">
            Log out
          </button>
        </div>
      </template>

      <template v-else>
        <h1>Login</h1>
        <p class="login-intro">Enter your email to sign in or create your account.</p>

        <form class="login-form" @submit.prevent="handleSubmit">
          <label class="login-label" for="login-email">Email</label>
          <input
            id="login-email"
            v-model="email"
            class="login-input"
            type="email"
            name="email"
            autocomplete="email"
            placeholder="you@example.com"
            required
            :disabled="submitting"
          />

          <p v-if="error" class="login-error" role="alert">{{ error }}</p>

          <button
            type="submit"
            class="btn-ha login-submit"
            :disabled="submitting || email.trim().length === 0"
          >
            {{ submitting ? 'Signing in…' : 'Sign in' }}
          </button>
        </form>

        <p class="login-help">
          No password required. A one-time password will be emailed in the future; for now, any
          valid email address signs you in.
        </p>
      </template>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  padding-top: 50px;
}

.login-card {
  max-width: 470px;
  margin: 0 auto;
  padding: 40px 34px;
  background-color: #1c1a27;
  border-radius: 12px;
  text-align: center;
}

.login-card h1 {
  margin-bottom: 10px;
  font-family: 'Equinox', 'Teko', sans-serif;
  font-size: 3rem;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.login-intro {
  margin-bottom: 28px;
  color: #c2d1d0;
  font-size: 1.5rem;
}

.login-form {
  text-align: left;
}

.login-label {
  display: block;
  margin-bottom: 8px;
  color: #ecfffb;
  font-size: 1.6rem;
  letter-spacing: 0.5px;
}

.login-input {
  width: 100%;
  height: 45px;
  padding: 5px 12px;
  border: 1px solid transparent;
  border-radius: 0;
  background-color: #000;
  color: #fff;
  font-size: 1.6rem;
}

.login-input:focus {
  border-color: #ffbe00;
}

.login-input:disabled {
  opacity: 0.6;
}

.login-error {
  margin: 12px 0 0;
  color: #ff2d55;
  font-size: 1.4rem;
}

.login-submit {
  width: 100%;
  margin-top: 22px;
}

.login-help {
  margin: 24px 0 0;
  color: #b9b8b8;
  font-size: 1.4rem;
  line-height: 1.55;
}

.login-checking {
  margin: 20px 0;
  color: #b9b8b8;
}

.login-email {
  margin-bottom: 8px;
  color: #ffbe00;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 2.4rem;
  letter-spacing: 0.5px;
  word-break: break-word;
}

.login-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 14px;
  margin-top: 26px;
}

@media (max-width: 767px) {
  .login-page {
    padding-top: 25px;
  }

  .login-card {
    padding: 28px 22px;
  }
}
</style>
