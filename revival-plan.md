# Halite II Local Competition Revival Plan

## Baseline
- Repository: HaliteChallenge/Halite-II
- Commit: `d0a4ed1`
- Preserve historical game engine, matchmaking, ratings, database model, compiler flow, and website.
- `COMPETITION_OPEN = True`
- `COMPETITION_FINALS_PAIRING = False`
- Do not include later finals or shutdown changes.

## Target
A local Halite II installation where users can:
1. Register and log in.
2. Submit bots.
3. Have bots compiled.
4. Continuously receive matchmaking tasks.
5. Run games indefinitely.
6. Store game results and replays.
7. Update rankings.
8. View bots, games, replays, and leaderboard through the website.

## Required Components
- Halite game engine (`environment/`)
- Public API/web backend (`apiserver`)
- Coordinator (`coordinator_server.py`)
- One or more workers (`worker/`)
- MySQL or MariaDB
- Historical website
- Local persistent storage for submissions, compiled artifacts, replays, and error logs

## Keep Unchanged Initially
- `environment/`
- `apiserver/apiserver/coordinator/coordinator.py`
- `apiserver/apiserver/coordinator/matchmaking.py`
- `apiserver/apiserver/coordinator/stat.py`
- `apiserver/apiserver/coordinator/compilation.py`
- `apiserver/apiserver/model.py`
- Alembic migrations
- Authentication/login logic
- `apiserver/worker/backend.py`
- `apiserver/worker/compiler.py`

## Required Changes

### 1. Local configuration
Patch or override `apiserver/apiserver/config.py`.

Set:
- `COMPETITION_OPEN = True`
- `COMPETITION_FINALS_PAIRING = False`
- New random Flask secret key
- Local `DATABASE_URL`
- Local `SITE_URL`
- Local `API_URL`
- Local CORS origin(s)

Remove runtime dependence on:
- Google Cloud SQL
- Google Cloud Storage credentials
- Production-only OAuth/SSO
- SendGrid, unless email features are wanted

Do not reuse historical production secrets.

### 2. Local database
Use MySQL/MariaDB.

Create database and application user, then initialize with:

```bash
cd apiserver
alembic upgrade head
```

Do not recreate the schema manually.

### 3. Replace Google Cloud Storage
Patch `apiserver/apiserver/coordinator/storage.py`.

Replace GCS operations with local filesystem storage.

Persistent directories:

```text
data/
├── bots/
├── compiled/
├── replays/
├── errors/
└── worker/
```

Preserve existing coordinator/worker API contracts, filenames, hashes, and artifact semantics where possible.

Required operations:
- Store submitted bot archives
- Retrieve bot archives for workers
- Store/retrieve compiled artifacts
- Store/retrieve replays
- Store/retrieve compilation and game error logs
- Provide worker artifact/configuration locally if still required by existing endpoints

Prefer a filesystem storage adapter over changing coordinator logic.

### 4. Worker setup
Run at least one worker locally.

Configure `apiserver/worker/config.json` with the local coordinator URL and required capabilities/secrets.

Keep `backend.py` unchanged unless local startup reveals a hard dependency on production infrastructure.

Build the Halite executable from `environment/` and make it available to the worker.

### 5. Bot execution sandbox
Patch `apiserver/worker/worker.py` only as required to replace production-specific execution.

The historical setup uses cgroups, sudo, Unix users, and network isolation.

For a trusted local installation:
- Run bots under a dedicated worker user.
- Execute each bot from its isolated working directory.
- Apply CPU, memory, process, timeout, and disk limits.

For untrusted submissions:
- Do not execute bots directly on the host.
- Replace the historical execution wrapper with container or VM-based isolation.
- Disable network access and mount only required files.

Keep the Halite stdin/stdout protocol unchanged.

### 6. Startup
Do not use the historical production `setup.sh` unchanged.

Create local startup configuration for:
- Database
- Public API on port 5000
- Coordinator on port 5001, bound privately
- Worker process
- Website/frontend
- Persistent data directories

A minimal deployment can use Docker Compose with services:

```text
db
api
coordinator
worker
website
```

The coordinator must not be publicly exposed.

## Build and Bring-Up Order

### Step 1: Pin source
```bash
git clone https://github.com/HaliteChallenge/Halite-II.git
cd Halite-II
git checkout d0a4ed1
```

### Step 2: Build engine
Build `environment/` with its historical CMake configuration.

Verify the `halite` executable can run known bots locally before integrating the server.

### Step 3: Create reproducible runtime
Initially use a Python 3.6-compatible container/runtime because the repository pins old Flask, SQLAlchemy, Alembic, Werkzeug, and related dependencies.

Do not modernize dependencies until the historical stack works.

### Step 4: Start database
Create the local database and application user.

Set `DATABASE_URL`.

Run:

```bash
cd apiserver
alembic upgrade head
```

### Step 5: Implement filesystem storage
Replace GCS-backed `storage.py` with a local storage adapter.

Test:
- upload
- download
- hashing
- compiled artifact retrieval
- replay storage

### Step 6: Start public API and coordinator
Run:
- public API: `:5000`
- coordinator: `127.0.0.1:5001`

Verify database access and coordinator endpoints.

### Step 7: Start one worker
Configure it to poll the local coordinator.

Verify:
1. Worker receives a task.
2. Worker downloads bot data.
3. Bot compiles.
4. Game starts.
5. Result is returned.
6. Replay is stored.
7. Rating updates.

### Step 8: Start website
Point the historical frontend at the local public API.

Verify:
- registration
- login/logout
- bot submission
- bot status
- leaderboard
- game history
- replay access

### Step 9: Continuous operation test
Submit at least two bots and verify the loop:

```text
submit
→ compile
→ enter matchmaking
→ assign game
→ execute game
→ upload result/replay
→ update ratings
→ repeat indefinitely
```

The coordinator must remain in normal competition mode:

```text
COMPETITION_OPEN = True
COMPETITION_FINALS_PAIRING = False
```

## Validation Checklist

### Engine
- [ ] Engine builds.
- [ ] Known bots can play.
- [ ] Replays are generated.

### Database
- [ ] Alembic migrations complete.
- [ ] Users persist.
- [ ] Bots/submissions persist.
- [ ] Games and rankings persist.

### Storage
- [ ] Bot archives persist.
- [ ] Workers can retrieve bot archives.
- [ ] Compiled artifacts persist.
- [ ] Replays persist.
- [ ] Error logs persist.

### Worker
- [ ] Worker registers/polls successfully.
- [ ] Compilation succeeds.
- [ ] Games execute.
- [ ] Results return to coordinator.
- [ ] Failed bots/games are handled without stopping the worker.

### Website
- [ ] User registration works.
- [ ] Login works.
- [ ] Submission works.
- [ ] Leaderboard updates.
- [ ] Games are visible.
- [ ] Replays are accessible.

### Continuous League
- [ ] At least two active bots continue receiving games.
- [ ] No competition closing date is enforced.
- [ ] No finals mode is enabled.
- [ ] Restarting services preserves database and stored artifacts.

## Minimal Patch Set
1. Local configuration/secret/database URLs.
2. Filesystem replacement for `coordinator/storage.py`.
3. Worker execution/sandbox adaptation in `worker.py`.
4. Local worker configuration.
5. Local startup/deployment files.

Everything else should remain unchanged until a concrete compatibility problem is encountered.

## Recommended First Implementation
Use one machine and one worker. Do not modernize the codebase yet. Do not add finals logic. Do not change game rules or matchmaking. First reproduce the historical pre-finals stack, replacing only cloud storage/database deployment and production worker infrastructure.
