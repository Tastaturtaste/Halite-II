<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { fetchMyBots, type Bot } from '../api'
import { useAuthStore } from '../stores/auth'

const auth = useAuthStore()

const bots = ref<Bot[]>([])
const loading = ref(true)
const error = ref('')

const bestRank = computed(() =>
  bots.value.length > 0 ? Math.min(...bots.value.map((bot) => bot.rank)) : null,
)
const totalGames = computed(() => bots.value.reduce((total, bot) => total + bot.gamesPlayed, 0))

onMounted(async () => {
  try {
    const response = await fetchMyBots()
    bots.value = response.bots
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Could not load your bots.'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="container user-profile">
    <div class="page-header">
      <h1>My Bots</h1>
    </div>

    <p v-if="auth.email" class="user-profile-detail">
      Bots uploaded by <span class="user-name">{{ auth.email }}</span>
    </p>

    <p v-if="loading" class="state-message">Loading your bots…</p>

    <div v-else-if="error" class="state-message state-message-error" role="alert">
      <p>{{ error }}</p>
      <RouterLink class="btn-ha" to="/play">Submit a bot</RouterLink>
    </div>

    <div v-else-if="bots.length === 0" class="profile-section section-empty">
      <h2>No bots yet</h2>
      <p>Upload a bot to join the matchmaking pool and start collecting results.</p>
      <RouterLink class="btn-ha" to="/play">Submit your first bot</RouterLink>
    </div>

    <template v-else>
      <div class="user-profile-rank-stats">
        <div class="stats-item">
          <h3>Bots</h3>
          <p>{{ bots.length }}</p>
        </div>
        <div class="stats-item">
          <h3>Best rank</h3>
          <p>{{ bestRank }}</p>
        </div>
        <div class="stats-item">
          <h3>Games played</h3>
          <p>{{ totalGames }}</p>
        </div>
      </div>

      <div class="profile-section">
        <h2>Your bots</h2>

        <div class="table-container">
          <table class="table-leader">
            <thead>
              <tr>
                <th>Bot name</th>
                <th class="hide-mobile">Language</th>
                <th>Rank</th>
                <th>Games played</th>
                <th class="hide-mobile">Uploaded</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="bot in bots" :key="bot.id">
                <td>{{ bot.name }}</td>
                <td class="hide-mobile">{{ bot.language }}</td>
                <td>{{ bot.rank }}</td>
                <td>{{ bot.gamesPlayed }}</td>
                <td class="hide-mobile">{{ bot.uploadDate }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div class="bots-actions">
        <RouterLink class="btn-ha" to="/play">Submit another bot</RouterLink>
      </div>
    </template>
  </div>
</template>

<style scoped>
.state-message {
  padding: 30px 0;
  color: #b9b8b8;
  text-align: center;
}

.state-message-error {
  color: #ff2d55;
}

.state-message-error .btn-ha {
  margin-top: 12px;
}

.user-profile-rank-stats {
  margin-top: 25px;
  padding: 10px 0 20px;
  border-bottom: 1px solid #363347;
}

.user-name {
  word-break: break-word;
}

.bots-actions {
  margin: 30px 0 40px;
  text-align: center;
}

@media (max-width: 991px) {
  .hide-mobile {
    display: none;
  }

  .user-profile-rank-stats {
    flex-wrap: wrap;
    gap: 16px;
  }

  .user-profile-rank-stats .stats-item {
    flex: 1 0 30%;
    margin: 0;
  }
}
</style>
