#!/usr/bin/env bash
# Validate packaged Node addon for filesystem mode on Debian Bookworm + Node 24.
# Copies the tree into a Linux-native path inside the container to avoid Windows
# bind-mount napi-rs filesystem transaction failures.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMAGE="argos-fs-bookworm:local"
VOLUME="argos-fs-bookworm-media"

echo "==> Building Bookworm validation image (Node 24 + Rust + build deps)"
docker build -t "$IMAGE" -f - "$ROOT" <<'EOF'
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl build-essential python3 pkg-config \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /work
EOF

echo "==> Ensuring named volume ${VOLUME}"
docker volume create "$VOLUME" >/dev/null

echo "==> Building napi linux-x64-gnu inside Bookworm and running filesystem example"
docker run --rm \
  -v "$ROOT:/src:ro" \
  -v "${VOLUME}:/media/library" \
  "$IMAGE" \
  bash -lc '
    set -euo pipefail
    echo "node=$(node -v) glibc=$(ldd --version | head -1)"
    rm -rf /work/argos
    mkdir -p /work
    # Copy without Windows node_modules / target noise when present
    cp -a /src /work/argos
    rm -rf /work/argos/target /work/argos/bindings/npm/node_modules \
      /work/argos/bindings/npm/*.node /work/argos/bindings/npm/.napi-rs-filesystem-transaction.swp || true
    mkdir -p /media/library/Artist/Album
    echo demo > /media/library/Artist/Album/track.flac
    cd /work/argos/bindings/npm
    npm install --ignore-scripts
    node build.mjs --release
    test -f argos.linux-x64-gnu.node
    echo "built $(ls -la argos.linux-x64-gnu.node)"
    ldd argos.linux-x64-gnu.node || true
    node -e "
      const { Workspace, watch } = require(\"./index.js\");
      const fs = require(\"fs\");
      const ws = Workspace.open(\"/media/library\", JSON.stringify({
        mode: \"filesystem\",
        symlinkMode: \"nofollow\",
        exclude: [\"**/*.partial\"]
      }));
      const snap = ws.currentSnapshot;
      if (snap.identity.mode !== \"filesystem\") throw new Error(\"mode\");
      if (!snap.planning.coverage.requiresReconciliation) throw new Error(\"startup-gap\");
      console.log(\"BOOKWORM_OPEN_OK\", JSON.stringify({
        mode: snap.identity.mode,
        scopes: ws.listScopes().length,
        coverage: snap.planning.coverage
      }));
      const ac = new AbortController();
      setTimeout(() => ac.abort(), 1200);
      setTimeout(() => {
        fs.writeFileSync(\"/media/library/Artist/Album/track.flac\", \"demo2\");
      }, 150);
      (async () => {
        let n = 0;
        for await (const ev of watch(ws, ac.signal)) {
          n++;
          console.log(\"event\", ev.kind || ev);
        }
        ws.close();
        console.log(\"BOOKWORM_WATCH_OK events=\" + n);
        if (n < 1) process.exit(2);
      })().catch(e => { console.error(e); process.exit(1); });
    "
  '

echo "==> Bookworm packaged-addon validation succeeded"
