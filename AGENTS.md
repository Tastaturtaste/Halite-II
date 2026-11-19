# Repository Instructions

## Revival Scope

- Treat `revival-plan.md` as the target. This checkout is the historical `d0a4ed1` runtime plus the plan; first reproduce that stack rather than modernizing it.
- Keep `COMPETITION_OPEN = True` and `COMPETITION_FINALS_PAIRING = False`. Do not add finals/shutdown behavior.
- Initially preserve the engine protocol/rules, matchmaking, TrueSkill updates, models/migrations, compiler, and coordinator/worker API contracts. Change them only for a demonstrated local-runtime incompatibility.
- Never reuse the blank/TODO production configuration as real credentials. Use a new Flask secret and local database, site, API, CORS, OAuth, and notification settings.

## Service Boundaries

- `environment/` builds the `halite` game executable. `apiserver/apiserver/server.py` is the public API; `coordinator_server.py` is the separate worker-facing coordinator.
- The coordinator issues compilation tasks before game tasks. Workers poll it, download and MD5-check bot archives, compile or run games, then POST artifacts/results; the coordinator persists results and updates ratings.
- `website/` is Jekyll plus a Vue/Webpack 3 bundle and depends on sibling `libhaliteviz/`. Its `_config.yml` still embeds production API/login/logout URLs.
- SQLAlchemy reflects tables when `apiserver.model` is imported. Create the database and run migrations before starting either Flask process.

## Historical Runtime

- Reproduce the pinned Python 3.6-era stack from `apiserver/requirements.txt` before attempting dependency upgrades; `setup.py` does not contain the full dependency set.
- Build the engine with the CMake presets: `cd environment && cmake --preset default && cmake --build --preset default`. The binary is `environment/build/halite`; use `--preset debug` for `environment/build/debug/halite`. Copy it into `apiserver/worker/` for the historical worker. CMake generates the version header in the build tree from `git describe`; no shell script is required. On Windows, run the same commands from a Developer Command Prompt so `cl` and Ninja are on `PATH`.
- Set the local MySQL/MariaDB URL in the current configuration mechanism, then run `cd apiserver && alembic upgrade head`. Do not initialize from `schema.sql`; with an empty `config.DATABASE_URL`, Alembic falls back to the placeholder in `alembic.ini`. `config.py` also reads `DATABASE_URL`, `FLASK_SECRET_KEY`, `SITE_URL`, and `API_URL` from the environment (used by the compose dev setup).
- Historical local invocations from `apiserver/` are `PYTHONPATH="$PWD" FLASK_APP=apiserver.server flask run --with-threads -h 0.0.0.0 -p 5000` and `PYTHONPATH="$PWD" FLASK_APP=apiserver.coordinator_server flask run --with-threads -h 127.0.0.1 -p 5001`. Keep port 5001 private; coordinator endpoints have no worker authentication.
- Do not run `apiserver/setup.sh` unchanged: it downloads Cloud SQL Proxy, uses `screen`, starts a nonexistent badge daemon, binds the coordinator publicly, and installs a production cron job.

## Local Storage And Worker

- Replacing GCS is not confined to `coordinator/storage.py`. Direct GCS calls also exist in `model.py`, `web/user_bot.py`, `web/match.py`, `web/user_match.py`, and `coordinator/coordinator.py`.
- Preserve storage identities: bot objects are keyed as `<user_id>_<bot_id>` and verified by MD5; replay DB/storage keys omit `.hlt`; error logs use `<replay_key>_error_log_<user_id>`; the worker artifact download is named `Halite.tgz`.
- `apiserver/worker/config.json` is read relative to the worker's current directory. `MANAGER_URL` must include the full trailing-slash prefix, for example `http://127.0.0.1:5001/v1/coordinator/`; required keys are `MANAGER_URL` and `SECRET_FOLDER`, with optional `CAPABILITIES`, `MAX_BOT_UPLOAD_SIZE`, `UNSANDBOXED`, and `HEALTH_CHECK_PORT`.
- The historical worker is not safe as a normal host process: compilation/game execution assumes `sudo`, dedicated Unix users, cgroup v1/`cgexec`, network isolation, and broad `pkill`. Trusted local bots still need resource limits; run untrusted submissions only in a network-disabled container or VM.
- A flag-gated container mode exists for trusted local bots: set `UNSANDBOXED: true` in `worker/config.json` and the worker runs compilation and games directly as root. `util.py` exposes `UNSANDBOXED` (read from `config.json`); when set, `worker.py` drops the `cgexec`/`sudo`/chown wrapper (uses `shutil.rmtree` cleanup) and `compiler.py` omits the `sudo -H -iu bot_compilation` prefix. Historical sandbox behavior is byte-for-byte unchanged when the flag is absent; untrusted submissions still require real isolation.
- Authentication is GitHub OAuth, not local-password registration. A local GitHub OAuth application is required if login logic remains unchanged. SendGrid calls also need disabling or a safe local replacement, not merely an empty API key.

## Website And Verification

- Historical website setup order is `cd website && bundle install --path=vendor/bundle && npm install`, then `cd ../libhaliteviz && npm install`, then `cd ../website && npm run build && bundle exec jekyll serve`. Pins include Ruby 2.0.0, Bundler 1.16 prerelease, Jekyll 3.4.4, Webpack 3, and node-sass 4; prefer a compatibility container.
- `npm test` intentionally fails in the website packages. CI only built the engine, selected starter bots, website, and visualizer; it did not run a repository test suite.
- Do not use `tests/runTests.sh` as a verification command: it references missing config/files and wrong historical paths. Prefer focused smoke checks: build `halite`, run two known bots, migrate a fresh database, hit `/health_check`, then verify the full submit -> compile -> game -> replay -> rating loop with at least two bots.

## Containerized Dev Environment

- `docker/Dockerfile.dev` builds a toolchain-only dev image: ubuntu:20.04 plus pyenv Python 3.6.15, nvm Node 8.17.0 (webpack 3/node-sass 4), rbenv Ruby 2.5.9 + Bundler 1.16.0.pre.3, and engine build deps. The repo is not COPYed in; `compose.yaml` bind-mounts `.` into `/workspace` and adds a `db` service (`mysql:5.7`, hostname `db`, database/user/password `halite`) with a named volume. 
- `.devcontainer/devcontainer.json` attaches the `dev` compose service. `scripts/dev_setup.sh` (postCreateCommand) installs `apiserver/requirements.txt` into `apiserver/venv`, builds the engine into `apiserver/worker/halite`, runs `bundle install`/`npm install` for `website/` and `libhaliteviz/`, and writes `apiserver/worker/config.json` with `UNSANDBOXED: true`. Dependencies live in the mounted workspace, not the image.
- `scripts/dev_start.sh` (run inside the container) runs `alembic upgrade head`, then starts in tmux: API `:5000`, coordinator `127.0.0.1:5001`, worker, and Jekyll `:4000` with `website/_config.dev.yml` overriding the API/login/logout URLs to localhost. Keep the coordinator loopback-only; its endpoints have no worker authentication.
- The compose `environment` block supplies `DATABASE_URL`, `FLASK_SECRET_KEY`, `SITE_URL`, `API_URL`, `OAUTH_GITHUB_CONSUMER_KEY`, `OAUTH_GITHUB_CONSUMER_SECRET`, `LANG`, and `LC_ALL`. `config.py` reads the config/env overrides. `LANG`/`LC_ALL` must be a UTF-8 locale (Flask 0.12's click aborts on ASCII); the OAuth keys are dev placeholders so the API boots, but real login still needs a real GitHub OAuth app.
- Local-runtime compatibility fixes in otherwise-unchanged sources: `web/user.py` points `tld`'s `NAMES_SOURCE_URL` at `publicsuffix.org` and swallows the download error (the historical `mxr.mozilla.org` default is dead and DNS-fails at import); `worker.py` serves its own Flask `/health_check` on port 5002 (the historical 5001 collides with the coordinator in a single container, where they normally ran on separate machines). The worker health-check port is set via `HEALTH_CHECK_PORT` in `config.json` (default 5002), so multiple workers can coexist in one container by binding distinct ports; it is internal-only and not published in `compose.yaml`.
