import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import { useAuthStore } from './stores/auth'
import './styles/main.scss'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)

// Hydrate the session once at startup; a 401 just leaves the user logged out.
void useAuthStore(pinia).hydrateOnce()

app.mount('#app')
