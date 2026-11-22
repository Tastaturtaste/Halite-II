<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { fetchGame, fetchGames, type GameDetail, type GameSummary } from '../api'

const games = ref<GameSummary[]>([])
const loadingGames = ref(true)
const gamesError = ref('')

const selectedId = ref('')
const game = ref<GameDetail | null>(null)
const loadingGame = ref(false)
const gameError = ref('')

onMounted(async () => {
  try {
    const response = await fetchGames()
    games.value = response.games
  } catch (err) {
    gamesError.value = err instanceof Error ? err.message : 'Could not load the game list.'
  } finally {
    loadingGames.value = false
  }
})

async function loadSelected() {
  if (!selectedId.value) {
    return
  }

  game.value = null
  gameError.value = ''
  loadingGame.value = true

  try {
    game.value = await fetchGame(selectedId.value)
  } catch (err) {
    gameError.value = err instanceof Error ? err.message : 'Could not load this game.'
  } finally {
    loadingGame.value = false
  }
}
</script>

<template>
  <div class="container visuallizer-container">
    <div class="page-header">
      <h1>Game Visualizer</h1>
    </div>

    <p class="page-description">
      Pick a match to inspect. Replay rendering is not available yet, but the game metadata is
      loaded from the same endpoint the future player will use.
    </p>

    <p v-if="loadingGames" class="visuallizer-loading-message">Loading games…</p>
    <p v-else-if="gamesError" class="inline-error" role="alert">{{ gamesError }}</p>

    <div v-else class="game-picker">
      <label for="game-select">Select a game</label>
      <select id="game-select" v-model="selectedId" class="game-select" @change="loadSelected">
        <option value="" disabled>Select a game…</option>
        <option v-for="summary in games" :key="summary.id" :value="String(summary.id)">
          {{ summary.name }}
        </option>
      </select>
    </div>

    <div v-if="loadingGame" class="visuallizer-loading-message">Loading game…</div>
    <p v-else-if="gameError" class="inline-error" role="alert">{{ gameError }}</p>

    <template v-else-if="game">
      <div class="game-heading">
        <p class="game-heading-date">Game #{{ game.id }}</p>
        <div class="game-heading-players">
          <span
            v-for="(player, index) in game.players"
            :key="`${player}-${index}`"
            class="player"
            :class="`color-${(index % 4) + 1}`"
          >
            {{ player }}
          </span>
        </div>
        <i class="xline xline-left"></i>
        <i class="xline xline-right"></i>
      </div>

      <div class="game-meta">
        <div class="game-meta-item">
          <h3>Map size</h3>
          <p>{{ game.mapWidth }} × {{ game.mapHeight }}</p>
        </div>
        <div class="game-meta-item">
          <h3>Players</h3>
          <p>{{ game.players.length }}</p>
        </div>
        <div class="game-meta-item">
          <h3>Turns</h3>
          <p>{{ game.turns }}</p>
        </div>
      </div>

      <div class="replay-placeholder">
        <p class="replay-placeholder-title">Replay visualization coming soon</p>
        <p>
          The map, planets and ship movements will be rendered here. The future player will use
          the same game metadata shown above.
        </p>
      </div>
    </template>

    <div v-else-if="!gamesError && !loadingGames" class="replay-placeholder replay-placeholder-idle">
      <p class="replay-placeholder-title">Select a game above</p>
      <p>Choose a match from the list to load its map size, players and turn count.</p>
    </div>
  </div>
</template>

<style scoped>
.game-picker {
  max-width: 420px;
  margin: 0 auto 30px;
}

.game-picker label {
  display: block;
  margin-bottom: 8px;
  color: #b9b8b8;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 1.9rem;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.game-select {
  width: 100%;
  height: 45px;
  padding: 5px 12px;
  border: 1px solid #474951;
  border-radius: 0;
  background-color: #000;
  color: #fff;
  font-size: 1.6rem;
}

.game-select:focus {
  border-color: #ffbe00;
}

.game-meta {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 16px;
  margin: 0 auto 30px;
  max-width: 760px;
}

.game-meta-item {
  flex: 1 1 160px;
  padding: 16px;
  background-color: #1c1a27;
  border: 1px solid #363347;
  text-align: center;
}

.game-meta-item h3 {
  margin: 0 0 4px;
  color: #b9b8b8;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 1.7rem;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.game-meta-item p {
  margin: 0;
  color: #ecfffb;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 2.8rem;
  letter-spacing: 0.5px;
}

.replay-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 420px;
  margin: 0 auto 50px;
  padding: 30px;
  border: 2px dashed #474951;
  border-radius: 12px;
  background-color: rgba(18, 17, 25, 0.55);
  color: #b9b8b8;
  text-align: center;
}

.replay-placeholder p {
  max-width: 560px;
  margin: 0;
  font-size: 1.5rem;
  line-height: 1.6;
}

.replay-placeholder .replay-placeholder-title {
  margin-bottom: 10px;
  color: #ecfffb;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 2.8rem;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.replay-placeholder-idle {
  min-height: 320px;
}

.inline-error {
  max-width: 640px;
  margin: 0 auto 30px;
  color: #ff2d55;
  text-align: center;
}

@media (max-width: 767px) {
  .replay-placeholder {
    min-height: 300px;
  }
}
</style>
