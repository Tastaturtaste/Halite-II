# Plan: Roadmap Step 1 — Simplified Halite II Website

Recreate the original Halite II website in simplified form. Backend serves the site and logs
requests only (no real business logic); frontend provides all pages with dummy functionality.

## Key decisions (agreed with user)
- **Look & feel**: Reuse original assets from git history (commit `f79d9067`, paths
  `website/assets/{images,fonts}` and `website/_sass/`). Port what's easy (logos, hero, fonts,
  core SCSS partials); otherwise keep it simple and roughly match the style — no pixel-perfect chase.
- **Visualizer**: Placeholder page only (route + layout + dummy game list, no replay rendering).
- **Starter kits**: Backend enumerates `airesources/` directory live via `GET /api/starterkits`;
  download buttons stay dummy.
- **Auth**: In-memory session store on backend (`RwLock<HashMap<token, email>>`), HttpOnly cookie,
  identity = email only, no database. `POST /api/auth/login` auto-logs-in any email (no OTP yet).

## Project setup
- New top-level `website/` directory:
  ```
  website/
    backend/            # Rust crate
      src/main.rs       # router, static serving, request logging
      src/routes/       # one module per API area (auth, starterkits, bots, games)
      src/session.rs    # in-memory session store
    frontend/           # Vue 3.5 + TypeScript + Vite
      src/router.ts
      src/pages/        # Home, Learn, StarterKits, Login, Play, MyBots, Visualizer
      src/components/
      src/stores/       # Pinia: auth, starterkits, bots
      src/api.ts        # typed fetch wrappers
      src/assets/       # recovered images + fonts
      src/styles/       # ported SCSS partials
  ```
- `mise.toml`: add `rust = "latest"` (keep cmake/ninja/node 26); add tasks:
  - `mise run backend` → `cargo run` (axum on :3000)
  - `mise run frontend` → `vite dev` with `/api` proxied to :3000
  - `mise run build` → `cargo build --release` + `vite build`
- Rust deps: axum, tokio, tower, tower-http (fs + trace), tracing/tracing-subscriber, serde, uuid.

## Asset recovery (from commit f79d9067)
- Copy into `website/frontend/src/assets/`:
  - Images: `full_logo.svg`, `hero.png`, chevron/hexagon/icon SVGs from `website/assets/images/`
  - Fonts: `Equinox*`, `Teko-*` from `website/assets/fonts/`
- Port as global SCSS: `_generic`, `_nav`, `_home`, `_play`, `_table`, `_user-profile`,
  `_visuallizer`, `_buttons` (skip Angular-era cruft: modals, hackathon, leagues, editor).
- SCSS is ported as inspiration (colors, typography, spacing, useful class names), not compiled as-is.

## Backend (axum — serve + log)
- `TraceLayer` request logging on everything; every API handler logs its payload.
- Endpoints (dummy JSON via serde):
  - `GET /api/starterkits` — enumerated from `airesources/`
  - `POST /api/auth/login {email}` → session cookie; `GET /api/me`; `POST /api/logout`
  - `POST /api/bots/upload` — accepts multipart, logs it, dummy success
  - `GET /api/users/me/bots` — dummy stats (rank, games played, upload date, language)
  - `GET /api/games/{id}` — dummy game/replay metadata
- `ServeDir` + SPA fallback serves `frontend/dist` (one binary serves everything in production).

## Frontend (Vue 3.5 composition API + TS + Vite + vue-router + Pinia)
- Pinia `auth` store hydrated via `/api/me`; route guard redirects unauthenticated users to `/login`.
- Pages:
  - `/` — home with hero section (`hero.png`, Halite II style)
  - `/learn` — instructions/documentation (static content, original doc-page styling)
  - `/learn/starter-kits` — table from API, dummy download buttons
  - `/login` — email form, auto-login via `/api/auth/login` (no OTP check yet)
  - `/play` — bot upload area, logged-in only, non-functional upload UI
  - `/my-bots` — user's bots with dummy stats table
  - `/visualize` — placeholder: layout + "select a game" list from dummy data
- Global nav bar + footer matching original (`_nav.scss` / `_footer.scss`).

## Implementation order
1. Scaffolding: mise config, cargo crate, Vite app, dev-proxy setup
2. Asset extraction + global SCSS from `f79d9067`
3. Backend: logging, static serving, session store, all endpoints with dummy data
4. Frontend: router, auth store, then pages (Home → Learn → StarterKits → Login → Play → My Bots → Visualizer)
5. Polish: SPA fallback, favicon, 404 page, look-over against original