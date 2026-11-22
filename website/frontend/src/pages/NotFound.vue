<template>
  <div class="container not-found">
    <div class="page-header">
      <h1>Page not found</h1>
    </div>

    <div class="not-found-body">
      <p class="not-found-code" aria-hidden="true">404</p>
      <p class="not-found-message">
        That page drifted out of the map. Check the address, or head back to familiar space.
      </p>
      <div class="not-found-actions">
        <RouterLink class="btn-ha" to="/">Return home</RouterLink>
        <RouterLink class="btn-ha btn-ha-clear" to="/learn">Learn the game</RouterLink>
      </div>
    </div>
  </div>
</template>

<style scoped>
.not-found-body {
  max-width: 620px;
  margin: 0 auto 60px;
  text-align: center;
}

.not-found-code {
  margin: 0;
  color: #ffbe00;
  font-family: 'Equinox', 'Teko', sans-serif;
  font-size: 12rem;
  line-height: 1;
  letter-spacing: 8px;
  opacity: 0.9;
}

.not-found-message {
  margin: 20px auto 30px;
  color: #c2d1d0;
  font-size: 1.7rem;
  line-height: 1.6;
}

.not-found-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 14px;
}

@media (max-width: 767px) {
  .not-found-code {
    font-size: 8rem;
  }
}
</style>
