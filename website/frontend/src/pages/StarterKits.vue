<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { fetchStarterKits, type StarterKit } from '../api'

const kits = ref<StarterKit[]>([])
const loading = ref(true)
const error = ref('')
const downloading = ref<string | null>(null)
const notice = ref('')
const noticeIsError = ref(false)

onMounted(async () => {
  try {
    const response = await fetchStarterKits()
    kits.value = response.starterkits
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Could not load starter kits.'
  } finally {
    loading.value = false
  }
})

function capitalize(message: string): string {
  return message.charAt(0).toUpperCase() + message.slice(1)
}

async function download(kit: StarterKit) {
  downloading.value = kit.name
  notice.value = ''

  try {
    const response = await fetch(kit.downloadUrl, { credentials: 'same-origin' })

    if (!response.ok) {
      let message = response.statusText || `Request failed (${response.status})`
      try {
        const body = (await response.json()) as { error?: string }
        if (body.error) {
          message = body.error
        }
      } catch {
        // No JSON body; keep the status text.
      }
      notice.value = `${kit.name}: ${capitalize(message)}`
      noticeIsError.value = true
      return
    }

    const blob = await response.blob()
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = kit.name
    link.click()
    URL.revokeObjectURL(url)
    notice.value = `${kit.name}: download started.`
    noticeIsError.value = false
  } catch (err) {
    notice.value = err instanceof Error ? err.message : `Could not download ${kit.name}.`
    noticeIsError.value = true
  } finally {
    downloading.value = null
  }
}
</script>

<template>
  <div class="container">
    <ul class="breadcrumb">
      <li><RouterLink to="/">Home</RouterLink></li>
      <li><RouterLink to="/learn">Learn</RouterLink></li>
      <li><span>Starter Kits</span></li>
    </ul>

    <div class="page-header">
      <h1>Starter Kits</h1>
    </div>

    <p class="page-description">
      Download a working bot in your favourite language, run it locally, and make it your own.
      Each kit ships with the game environment and a simple example bot.
    </p>

    <p
      v-if="notice"
      class="inline-notice"
      :class="{ 'inline-notice-error': noticeIsError }"
      role="status"
    >
      {{ notice }}
    </p>

    <p v-if="loading" class="loading-message">Loading starter kits…</p>
    <p v-else-if="error" class="inline-notice inline-notice-error" role="alert">{{ error }}</p>
    <p v-else-if="kits.length === 0" class="loading-message">No starter kits found.</p>

    <div v-else class="table-container">
      <table class="table-leader">
        <thead>
          <tr>
            <th>Name</th>
            <th class="hide-mobile">Language</th>
            <th>Download</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="kit in kits" :key="kit.name">
            <td>
              <strong>{{ kit.name }}</strong>
              <span class="language-mobile">{{ kit.language }}</span>
            </td>
            <td class="hide-mobile">{{ kit.language }}</td>
            <td>
              <button
                type="button"
                class="btn btn-sm"
                :disabled="downloading === kit.name"
                @click="download(kit)"
              >
                <span>{{ downloading === kit.name ? 'Downloading…' : 'Download' }}</span>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <p class="download-note">
      Downloads are not available yet — clicking a button reports the backend's current status.
    </p>
  </div>
</template>

<style scoped>
.inline-notice {
  max-width: 760px;
  margin: 0 auto 20px;
  padding: 12px 16px;
  border: 1px solid #474951;
  border-radius: 6px;
  background-color: rgba(18, 17, 25, 0.75);
  color: #c2d1d0;
  font-size: 1.5rem;
  text-align: center;
}

.inline-notice-error {
  border-color: #ff2d55;
  color: #ff2d55;
}

.loading-message {
  padding: 30px 0;
  color: #b9b8b8;
  text-align: center;
}

.language-mobile {
  display: none;
  margin-left: 10px;
  color: #b9b8b8;
  font-size: 1.4rem;
}

.download-note {
  margin: 24px 0 40px;
  color: #b9b8b8;
  font-size: 1.4rem;
  text-align: center;
}

@media (max-width: 991px) {
  .hide-mobile {
    display: none;
  }

  .language-mobile {
    display: inline;
  }
}
</style>
