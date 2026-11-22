<script setup lang="ts">
import hero from '../assets/images/hero.png'

const steps = [
  {
    title: 'Write a bot',
    text: 'Grab a starter kit in the language you know best and teach your bot to mine halite, expand and fight.',
  },
  {
    title: 'Upload it',
    text: 'Package your bot as a .zip and submit it from the Play page. Every upload joins the matchmaking pool.',
  },
  {
    title: 'Climb the leaderboard',
    text: 'Your bot keeps playing while you iterate. Watch its rank rise as it outlasts the competition.',
  },
]
</script>

<template>
  <div class="home-container">
    <section class="home-hero">
      <div
        class="home-hero-art"
        :style="{ backgroundImage: `url(${hero})` }"
        role="img"
        aria-label="Halite II — may the best bot win"
      ></div>

      <div class="home-hero-copy container">
        <h1 class="t2 c-wht">A programming competition in deep space</h1>
        <p class="t5 c-gry home-pitch">
          Halite II is an open-source AI challenge. Write a bot, mine halite, build a fleet and
          outmanoeuvre your rivals on a four-player map. No game engine experience required — just
          code.
        </p>

        <div class="home-actions">
          <RouterLink class="btn-ha btn-ha-lg" to="/learn">Learn the game</RouterLink>
          <RouterLink class="btn-ha btn-ha-lg btn-ha-clear" to="/visualize">
            Watch games
          </RouterLink>
        </div>
      </div>
    </section>

    <div class="container home-steps">
      <h2 class="t3 c-wht">How it works</h2>
      <div class="line-container">
        <i class="xline xline-top"></i>
      </div>

      <div class="home-step-grid">
        <div v-for="(step, index) in steps" :key="step.title" class="home-step">
          <div class="hex-li">
            <span class="hex-bullet">{{ index + 1 }}</span>
            <div class="home-step-copy">
              <h3 class="t4 c-yel">{{ step.title }}</h3>
              <p>{{ step.text }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-hero {
  margin-bottom: 50px;
}

.home-hero-art {
  width: 100%;
  aspect-ratio: 1140 / 400;
  background-color: #0d1117;
  background-position: center;
  background-repeat: no-repeat;
  background-size: cover;
  border-bottom: 1px solid #363347;
}

.home-hero-copy {
  padding-top: 35px;
}

.home-pitch {
  max-width: 760px;
  margin: 0 auto 30px;
  line-height: 1.6;
}

.home-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 16px;
}

.home-steps {
  margin-bottom: 40px;
}

.home-step-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 24px;
  margin-top: 35px;
  text-align: left;
}

.home-step {
  padding: 22px 24px;
  background-color: #1c1a27;
  border-radius: 12px;
}

.home-step .hex-bullet {
  flex: 0 0 56px;
  width: 56px;
  height: 67px;
  line-height: 66px;
  font-size: 28px;
}

.home-step-copy h3 {
  margin: 0 0 8px;
}

.home-step-copy p {
  margin: 0;
  color: #c2d1d0;
  font-size: 1.5rem;
  line-height: 1.5;
}

@media (max-width: 991px) {
  .home-step-grid {
    grid-template-columns: minmax(0, 1fr);
  }
}

@media (max-width: 767px) {
  .home-hero-art {
    aspect-ratio: 3 / 2;
    background-size: cover;
  }

  .home-hero-copy {
    padding-top: 25px;
  }

  .home-hero-copy .t2 {
    font-size: 2.6rem;
  }
}
</style>
