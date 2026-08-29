#!/usr/bin/env bash
# One-time setup run by the devcontainer's postCreateCommand.
# Installs API/website dependencies into the bind-mounted workspace and
# builds the engine. Idempotent.
set -euo pipefail

cd /workspace

echo "==> Python venv + API dependencies"
cd apiserver
if [ ! -d venv ]; then
  python3 -m venv venv
fi
./venv/bin/pip install -r requirements.txt
cd ..

echo "==> Building engine"
cmake -S environment -B environment/build
cmake --build environment/build -- -j2
cp environment/build/halite apiserver/worker/halite

echo "==> Website dependencies"
cd libhaliteviz
rm -rf node_modules
npm install
cd ../website
bundle install --path=vendor/bundle
# Clear node_modules to avoid stale .staging trees that break npm 6 re-runs
rm -rf node_modules
npm install
cd ..

echo "==> Worker config"
if [ ! -f apiserver/worker/config.json ]; then
  cat > apiserver/worker/config.json <<'EOF'
{
  "MANAGER_URL": "http://127.0.0.1:5001/v1/coordinator/",
  "SECRET_FOLDER": "local-dev",
  "UNSANDBOXED": true
}
EOF
fi

echo "==> Done. Start services with scripts/dev_start.sh"