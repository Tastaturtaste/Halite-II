#!/usr/bin/env bash
# Starts the local competition stack inside the devcontainer using tmux:
#   api (5000), coordinator (127.0.0.1:5001), worker, website (4000).
# Attaches to the tmux session; detach with Ctrl-b d. Stop with:
#   tmux kill-session -t halite
set -euo pipefail

cd /workspace/apiserver

echo "==> Migrating database"
./venv/bin/alembic upgrade head

tmux new-session -d -s halite -n api
tmux send-keys -t halite:api \
  'cd /workspace/apiserver && PYTHONPATH="$PWD" FLASK_APP=apiserver.server ./venv/bin/flask run --with-threads -h 0.0.0.0 -p 5000' Enter

tmux new-window -t halite -n coordinator
tmux send-keys -t halite:coordinator \
  'cd /workspace/apiserver && PYTHONPATH="$PWD" FLASK_APP=apiserver.coordinator_server ./venv/bin/flask run --with-threads -h 127.0.0.1 -p 5001' Enter

tmux new-window -t halite -n worker
tmux send-keys -t halite:worker \
  'cd /workspace/apiserver/worker && /workspace/apiserver/venv/bin/python worker.py' Enter

tmux new-window -t halite -n website
tmux send-keys -t halite:website \
  'cd /workspace/website && npm run build && bundle exec jekyll serve --host 0.0.0.0 --port 4000 --config _config.yml,_config.dev.yml' Enter

echo "==> Attaching to tmux session 'halite' (Ctrl-b d to detach)"
tmux attach -t halite