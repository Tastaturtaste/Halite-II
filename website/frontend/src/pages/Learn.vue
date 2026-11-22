<template>
  <div class="container">
    <ul class="breadcrumb">
      <li><RouterLink to="/">Home</RouterLink></li>
      <li><span>Learn</span></li>
    </ul>

    <div class="page-header">
      <h1>Learn the game</h1>
    </div>

    <div class="learn-layout">
      <article class="doc-panel">
        <h2>What is Halite II?</h2>
        <p>
          Halite II is a turn-based strategy game played by bots on a rectangular map. A match has
          two or four players, and each player controls a fleet of ships. Every turn your bot
          receives the current game state and returns a list of commands: where each ship should
          move, which planet it should dock on, and when it should attack.
        </p>
        <p>
          Planets hold <strong>halite</strong>, the resource that powers everything. Send a ship to
          a planet, dock, and it will mine halite over time. Halite pays for new ships, and planets
          you control can produce them for you. The player with the most halite when the game ends
          — or the last fleet still flying — wins.
        </p>

        <h2>How the competition works</h2>
        <p>
          You do not play matches by hand. You write a bot, submit it, and the matchmaker pairs it
          against other bots continuously. Each game is recorded and ranked, and your bot's position
          on the leaderboard is based on how it performs across those games.
        </p>
        <p>
          There is no single final round: the competition runs all the time. Improve your strategy,
          upload a new version of your bot, and keep climbing. Bots can be written in any language
          for which a starter kit is available.
        </p>

        <h2>Getting started</h2>
        <ol class="learn-steps">
          <li>
            <strong>Download a starter kit.</strong>
            Pick your language on the
            <RouterLink to="/learn/starter-kits">starter kits page</RouterLink>. Each kit includes
            a working bot and the local game environment.
          </li>
          <li>
            <strong>Run a game locally.</strong>
            Build the starter bot as-is and watch it play. It is deliberately simple, so there is
            plenty of room to improve.
          </li>
          <li>
            <strong>Write your strategy.</strong>
            Start with navigation: move ships toward the nearest rich planet and keep them mining.
            Then add expansion, defence and combat as you learn the map.
          </li>
          <li>
            <strong>Submit your bot.</strong>
            Package the bot as a .zip and upload it from the
            <RouterLink to="/play">Play page</RouterLink>. You need to be logged in first.
          </li>
          <li>
            <strong>Watch and iterate.</strong>
            Review your games on the
            <RouterLink to="/visualize">visualizer</RouterLink>, find weak turns, and upload an
            improved version.
          </li>
        </ol>

        <div class="learn-cta">
          <RouterLink class="btn-ha" to="/learn/starter-kits">Browse starter kits</RouterLink>
        </div>
      </article>

      <aside class="learn-aside">
        <h2>Quick links</h2>
        <ul class="list-ha">
          <li><RouterLink to="/learn/starter-kits"><span>Starter kits</span></RouterLink></li>
          <li><RouterLink to="/play"><span>Submit a bot</span></RouterLink></li>
          <li><RouterLink to="/my-bots"><span>My bots</span></RouterLink></li>
          <li><RouterLink to="/visualize"><span>Watch games</span></RouterLink></li>
        </ul>

        <h2>The short version</h2>
        <p class="learn-aside-text">
          Mine more halite than everyone else. Build ships. Do not let your fleet get wiped out.
          Upload early, then improve.
        </p>

        <p class="learn-aside-text">
          May the best bot win.
        </p>
      </aside>
    </div>
  </div>
</template>

<style scoped>
.learn-layout {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 300px;
  gap: 30px;
  align-items: start;
  margin-bottom: 40px;
}

.doc-panel {
  padding: 32px;
  background-color: #1c1a27;
  border-radius: 12px;
  line-height: 1.65;
}

.doc-panel h2 {
  margin: 28px 0 10px;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 2.8rem;
  letter-spacing: 1.5px;
  text-transform: uppercase;
}

.doc-panel h2:first-child {
  margin-top: 0;
}

.doc-panel p {
  margin-bottom: 16px;
  color: #d8e4e2;
  font-size: 1.6rem;
}

.doc-panel strong {
  color: #ecfffb;
}

.doc-panel a {
  color: #ffbe00;
}

.doc-panel a:hover {
  color: #f1a703;
  text-decoration: underline;
}

.learn-steps {
  margin: 0 0 28px;
  padding-left: 24px;
  color: #d8e4e2;
  font-size: 1.6rem;
}

.learn-steps li {
  margin-bottom: 12px;
  padding-left: 6px;
}

.learn-steps li::marker {
  color: #ffbe00;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 2rem;
}

.learn-cta {
  margin-top: 30px;
}

.learn-aside {
  padding: 24px;
  background-color: rgba(18, 17, 25, 0.75);
  border: 1px solid #363347;
  border-radius: 12px;
}

.learn-aside h2 {
  margin: 0 0 14px;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 2.2rem;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.learn-aside h2 + h2,
.learn-aside-text + h2 {
  margin-top: 26px;
}

.learn-aside .list-ha > li > a {
  padding: 4px 0;
}

.learn-aside-text {
  margin: 0;
  color: #b9b8b8;
  font-size: 1.5rem;
  line-height: 1.55;
}

@media (max-width: 991px) {
  .learn-layout {
    grid-template-columns: minmax(0, 1fr);
  }

  .doc-panel {
    padding: 24px;
  }
}
</style>
